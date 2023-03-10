use std::marker::PhantomData;

pub struct Kernel<I: crate::LayerValue, O: crate::LayerValue, T: crate::LayerTrainables> {
    _input: PhantomData<I>,
    _output: PhantomData<O>,
    _trainable: PhantomData<T>,
}