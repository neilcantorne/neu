pub struct OpGraph {
    root: OpNode
}

pub enum OpNode {
    Add(Operand, Operand),
    Subtraction(Operand, Operand),
    Division(Operand, Operand),
    Multiply(Operand, Operand),
    Hadamard(Operand, Operand),
}

pub enum Operand {
    Tensor2(u32, u32, ValueType),
    Tensor3(u32, u32, u32, ValueType),
    Scalar(ValueType),
    Node(Box<OpNode>)
}

pub struct RefValue {
    dimension: crate::Dimension,
    type_: ValueType,
}

pub enum ValueType {
    F32,
    F64,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64
}