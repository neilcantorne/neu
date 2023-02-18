pub trait LayerTrainables {
    type Values;
}

impl<F> LayerTrainables for crate::Tensor<F> {
    type Values = crate::Value;
}

impl<F> LayerTrainables for (crate::Tensor<F>, crate::Tensor<F>) {
    type Values = (crate::Value, crate::Value);
}