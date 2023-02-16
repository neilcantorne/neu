use std::ptr::NonNull;

pub struct Tensor<F> {
    buffer: NonNull<F>,
    dimension: crate::Dimension,
}