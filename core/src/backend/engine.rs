use std::sync::Arc;

pub struct Engine(Arc<dyn EngineInner>);

impl Engine {
    
    //-- Privates

}

pub(super) trait EngineInner {

}

pub(super) struct CudaEngine {
    
}


impl EngineInner for CudaEngine {

}

pub(super) struct ClEngineInner {

}



impl EngineInner for ClEngineInner {
    
}