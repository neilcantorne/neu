#[derive(PartialEq)]
pub enum Operand {
    Parameter(u32, super::GeneralType),
    Node(Box<super::Node>)
}

impl Operand {
    fn general_type(&self) -> super::GeneralType {
        match self {
            Operand::Parameter(_, ty) => *ty,
            Operand::Node(inner) => inner.result_type(),
        }
    }
}

pub trait IntoOperand {
    fn operand(&self) -> Operand;
}