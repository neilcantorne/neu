mod engine;
mod device;
mod query_device_iter;

pub use engine::Engine;
pub use device::Device;
pub use query_device_iter::QueryDeviceIter;

use engine::{ CudaEngine, ClEngine };
use device::{ CudaDevice, ClDevice };

// Short enums
pub enum Backend {
    Cuda,
    OpenCl,
    All,
}

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