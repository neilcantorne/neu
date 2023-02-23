mod engine;
mod device;
mod query_device_iter;
mod backend_api;
mod dynamic_library;
mod handle;
mod cu;
mod cl;

pub use engine::Engine;
pub use device::Device;
pub use query_device_iter::QueryDeviceIter;
pub use backend_api::BackendApi;

use engine::{ CudaEngine, ClEngine };
use device::{ CudaDevice, ClDevice };
use dynamic_library::DynamicLibrary;
use handle::Handle;
