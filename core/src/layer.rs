pub trait Layer where Self::Trainables: crate::LayerTrainables {
    type Trainables;

    fn operations(&self, input: crate::Value, trainables: <Self::Trainables as crate::LayerTrainables>::Values) -> crate::Value;
}