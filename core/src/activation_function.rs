#[derive(Clone, Copy)]
pub enum ActivationFunction {
    Sigmoid,
    Tanh,
    Relu,
    LeakyRelu(f32),
    Elu,
    Swish,
    Softplus(f32),
}