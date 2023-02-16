use std::marker::PhantomData;

pub struct Kernel<I: crate::LayerInput, O: crate::LayerOutput, T: crate::LayerTrainables> {
    _input: PhantomData<I>,
    _output: PhantomData<O>,
    _trainable: PhantomData<T>,
}