pub struct Value {
    pub(super) operand: super::Operand,
}

#[allow(clippy::from_over_into)]
impl Into<super::Operand> for Value {
    #[inline(always)]
    fn into(self) -> super::Operand {
        self.operand
    }
}