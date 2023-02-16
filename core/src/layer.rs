pub trait Layer where 
    Self::Input: crate::LayerValue + Sized, 
    Self::Output: crate::LayerValue + Sized,
    Self::Trainables: crate::LayerTrainables + Sized {
    type Input;
    type Output;
    type Trainables;

    fn generate_kernel(&self, input: crate::RefValue) -> crate::Kernel<Self::Input, Self::Output, Self::Trainables>;
}