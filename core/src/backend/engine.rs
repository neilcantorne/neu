use std::sync::Arc;

pub struct Engine(pub(super) Arc<dyn EngineInner>);

impl Engine {
    
    //-- Privates

}

pub(super) trait EngineInner {

}

pub(super) struct CudaEngine {
    pub(super) context: cuda_driver_sys::CUcontext,
}


impl EngineInner for CudaEngine {

}


impl Drop for CudaEngine {
    fn drop(&mut self) {
        unsafe {
            cuda_driver_sys::cuCtxDestroy_v2(self.context);
        }
    }
}

pub(super) struct ClEngine {
    pub(super) context: super::VoidPtr
}

impl EngineInner for ClEngine {
    
}

impl Drop for ClEngine {
    fn drop(&mut self) {
        unsafe {
            cl3::ext::clReleaseContext(self.context);
        }
    }
}