use std::marker::PhantomData;

pub struct OpBuilder<I: crate::LayerInput> {
    _input: PhantomData<I>
}

impl<I: crate::LayerInput> OpBuilder<I> {

}