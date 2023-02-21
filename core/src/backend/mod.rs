mod engine;
mod device;
mod query_device_iter;
mod backend_api;

pub use engine::Engine;
pub use device::Device;
pub use query_device_iter::QueryDeviceIter;
pub use backend_api::BackendApi;

use engine::{ CudaEngine, ClEngine };
use device::{ CudaDevice, ClDevice };

// Cuda initialization related code
static CudaInit: bool = false;

fn initialize_cuda() -> crate::Result<()>{
    unsafe {
        if !CudaInit && cuda_driver_sys::cuInit(0) != cuda_driver_sys::cudaError_enum::CUDA_SUCCESS {
            return crate::Errors::FailedToInitializeCuda.into();
        }

        Ok(())
    }
}