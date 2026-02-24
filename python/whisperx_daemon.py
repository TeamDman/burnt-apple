import argparse
import json
import pathlib
import socket
import sys
import traceback

import whisperx

def response_ok(payload: dict | None = None) -> dict:
    return {"ok": True, "payload": payload or {}}


def response_err(message: str) -> dict:
    return {"ok": False, "error": message}


def ensure_parent(path: pathlib.Path) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)


def write_outputs(input_path: pathlib.Path, output_dir: pathlib.Path, result: dict) -> dict:
    base = input_path.stem
    output_dir.mkdir(parents=True, exist_ok=True)

    json_path = output_dir / f"{base}.json"
    txt_path = output_dir / f"{base}.txt"

    ensure_parent(json_path)
    ensure_parent(txt_path)

    with open(json_path, "w", encoding="utf-8") as f:
        json.dump(result, f, ensure_ascii=False, indent=2)

    segments = result.get("segments", [])
    text = " ".join(str(segment.get("text", "")).strip() for segment in segments).strip()
    with open(txt_path, "w", encoding="utf-8") as f:
        f.write(text)

    return {
        "json": str(json_path),
        "txt": str(txt_path),
    }


def handle_request(model, request: dict) -> dict:
    cmd = request.get("cmd")
    if cmd == "health":
        return response_ok({"status": "ready"})

    if cmd != "infer":
        return response_err(f"unsupported command: {cmd}")

    input_path = pathlib.Path(str(request.get("input", ""))).resolve()
    output_home = pathlib.Path(str(request.get("output_home", ""))).resolve()

    if not input_path.exists():
        return response_err(f"input file does not exist: {input_path}")

    try:
        audio = whisperx.load_audio(str(input_path))
        result = model.transcribe(audio, batch_size=16)
        files = write_outputs(input_path, output_home, result)
        return response_ok({"files": files})
    except Exception as exc:
        return response_err(f"inference failed: {exc}")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--port", type=int, required=True)
    parser.add_argument("--model", type=str, default="large-v2")
    parser.add_argument("--language", type=str, default="en")
    parser.add_argument("--device", type=str, default="cuda")
    parser.add_argument("--compute-type", type=str, default="float16")
    args = parser.parse_args()

    try:
        model = whisperx.load_model(
            args.model,
            device=args.device,
            language=args.language,
            compute_type=args.compute_type,
        )
    except Exception:
        traceback.print_exc()
        return 2

    server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    server.bind(("127.0.0.1", args.port))
    server.listen(16)

    while True:
        conn, _ = server.accept()
        with conn:
            try:
                file = conn.makefile("rwb")
                raw_line = file.readline()
                if not raw_line:
                    continue
                request = json.loads(raw_line.decode("utf-8"))
                response = handle_request(model, request)
                file.write((json.dumps(response) + "\n").encode("utf-8"))
                file.flush()
            except Exception as exc:
                fallback = response_err(f"daemon exception: {exc}")
                try:
                    conn.sendall((json.dumps(fallback) + "\n").encode("utf-8"))
                except Exception:
                    pass


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except KeyboardInterrupt:
        sys.exit(0)
