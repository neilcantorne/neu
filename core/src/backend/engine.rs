use std::sync::Arc;

use super::{ cl, cu };

pub struct Engine(pub(super) Arc<dyn EngineInner>);

impl Engine {
    
    //-- Privates

}

pub(super) trait EngineInner {

}

pub(super) struct CudaEngine {
    pub(super) context: cu::Context,
    pub(super) cu: cu::Cuda,
}


impl EngineInner for CudaEngine {

}


impl Drop for CudaEngine {
    fn drop(&mut self) {
        unsafe {
            self.cu.ctx_destroy_v2(self.context);
        }
    }
}

pub(super) struct ClEngine {
    pub(super) context: cl::Context,
    pub(super) queue: cl::CommandQueue,
    pub(super) cl: cl::OpenCl,
}

impl EngineInner for ClEngine {
    
}

impl Drop for ClEngine {
    fn drop(&mut self) {
        unsafe {
            self.cl.release_command_queue(self.queue);
            self.cl.release_context(self.context);
        }
    }
}