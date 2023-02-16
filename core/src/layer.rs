pub trait Layer where 
    Self::Input: crate::LayerValue + Sized, 
    Self::Output: crate::LayerValue + Sized,
    Self::Trainables: crate::LayerTrainables + Sized {
    type Input;
    type Output;
    type Trainables;

    fn generate_kernel(&self, input: &Self::Input, builder: crate::OpBuilder) -> crate::Kernel<Self::Input, Self::Output, Self::Trainables>;
}