mod engine;
mod device;
mod query_device_iter;
mod backend_api;
mod dynamic_library;

pub use engine::Engine;
pub use device::Device;
pub use query_device_iter::QueryDeviceIter;
pub use backend_api::BackendApi;

use engine::{ CudaEngine, ClEngine };
use device::{ CudaDevice, ClDevice };
use dynamic_library::DynamicLibrary;

// Alias *mut std::ffi::c_void for convenience
type VoidPtr = *mut std::ffi::c_void;

// Cuda initialization related code
static IS_CUDA_INIT: bool = false;

fn initialize_cuda() -> crate::Result<()>{
    unsafe {
        if !IS_CUDA_INIT && cuda_driver_sys::cuInit(0) != cuda_driver_sys::cudaError_enum::CUDA_SUCCESS {
            return crate::Errors::FailedToInitializeCuda.into();
        }

        Ok(())
    }
}