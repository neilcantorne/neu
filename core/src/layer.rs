pub trait Layer where 
    Self::Input: crate::LayerInput + Sized, 
    Self::Output: crate::LayerOutput + Sized,
    Self::Trainables: crate::LayerTrainables + Sized {
    type Input;
    type Output;
    type Trainables;

    fn generate_kernel(builder: crate::OpBuilder<Self::Input>) -> crate::Kernel<Self::Input, Self::Output, Self::Trainables>;
}