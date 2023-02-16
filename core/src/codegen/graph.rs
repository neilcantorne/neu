pub struct Graph {
    pub(super) root: Option<Node>
}

pub enum Node {
    Add(Operand, Operand),
    Subtraction(Operand, Operand),
    Division(Operand, Operand),
    Multiply(Operand, Operand),
    Hadamard(Operand, Operand),
}

pub enum Operand {
    Parameter(u32, GeneralType),
    Node(Box<Node>)
}

pub enum GeneralType {
    Tensor2(u32, u32, Element),
    Tensor3(u32, u32, u32, Element),
    Scalar(ScalarType),
}

pub enum ScalarType {
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

pub struct Element {
    pub(super) channels: usize,
    pub(super) type_: ScalarType,
}