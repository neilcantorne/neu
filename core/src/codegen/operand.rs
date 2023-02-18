#[derive(PartialEq)]
pub enum Operand {
    Parameter(u32, super::GeneralType),
    Constant(super::Constant),
    Node(Box<super::Node>),
}
