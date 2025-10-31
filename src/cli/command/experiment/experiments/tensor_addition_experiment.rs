use burn::tensor::Tensor;
use burn::tensor::backend::Backend;
use tracing::info;

pub struct TensorAdditionExperiment {}
impl TensorAdditionExperiment {
    pub fn run<B: Backend>() -> eyre::Result<()> {
        info!("Running Tensor Addition experiment...");

        let device = Default::default();

        // Creation of two tensors, the first with explicit values and the second one with ones, with the same shape as the first
        let tensor_1 = Tensor::<B, 2>::from_data([[2., 3.], [4., 5.]], &device);
        let tensor_2 = Tensor::<B, 2>::ones_like(&tensor_1);

        // Print the element-wise addition (done with the WGPU backend) of the two tensors.
        println!("{}", tensor_1 + tensor_2);

        Ok(())
    }
}
