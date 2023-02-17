pub trait Layer where Self::Trainables: crate::LayerTrainables {
    type Input;
    type Output;
    type Trainables;

    fn operations(&self, input: crate::Value) -> crate::Value;
}