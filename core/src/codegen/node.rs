#[derive(PartialEq)]
pub enum Node {
    Add(super::Operand, super::Operand),
    Subtraction(super::Operand, super::Operand),
    Division(super::Operand, super::Operand),
    Multiply(super::Operand, super::Operand),
    HadamardProduct(super::Operand, super::Operand),
}

impl Node {
    pub(super) fn result_type(&self) -> super::GeneralType {
        match self {
            Node::Add(_, _) => todo!(),
            Node::Subtraction(_, _) => todo!(),
            Node::Division(_, _) => todo!(),
            Node::Multiply(_, _) => todo!(),
            Node::HadamardProduct(_, _) => todo!(),
        }
    }
}