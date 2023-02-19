#[derive(PartialEq)]
pub enum Node {
    Add(super::Operand, super::Operand),
    Subtract(super::Operand, super::Operand),
    Divide(super::Operand, super::Operand),
    Multiply(super::Operand, super::Operand),
    HadamardProduct(super::Operand, super::Operand),
    Convolve(super::Operand, (u32, u32), (u32, u32)),
    ConvergeSum(super::Operand),
    Sigmoid(super::Operand),
    Tanh(super::Operand),
    Relu(super::Operand),
    LeakyRelu(super::Operand, f32),
    Elu(super::Operand),
    Swish(super::Operand),
    Softplus(super::Operand, f32),
}