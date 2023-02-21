use std::sync::Arc;

pub struct Engine(Arc<dyn EngineInner>);

impl Engine {
    
    //-- Privates

}

pub(super) trait EngineInner {

}

struct CudaEngine {
    
}


impl EngineInner for CudaEngine {

}

struct ClEngineInner {

}



impl EngineInner for ClEngineInner {
    
}