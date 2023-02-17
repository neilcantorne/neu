#[derive(PartialEq)]
pub enum Operand {
    Parameter(u32, super::GeneralType),
    Constant(super::Constant),
    Node(Box<super::Node>),
}

impl Operand {
    fn general_type(&self) -> super::GeneralType {
        match self {
            Operand::Parameter(_, type_) => *type_,
            Operand::Constant(constant)
                => constant.general_type(),
            Operand::Node(node) => node.result_type(),
        }
    }
}

pub trait IntoOperand {
    fn operand(&self) -> Operand;
}