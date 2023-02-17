pub struct Value {
    pub(super) operand: super::Operand,
}

impl super::IntoOperand for Value {
    fn operand(&self) -> super::operand::Operand {
        return self.operand;
    }
}