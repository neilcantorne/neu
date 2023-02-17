use super::{
    Operand,
    Constant,
    Node,
};

pub struct Value(pub(super) Operand);

#[allow(clippy::from_over_into)]
impl Into<super::Operand> for Value {
    #[inline(always)]
    fn into(self) -> super::Operand {
        self.0
    }
}

impl TryFrom<f32> for Value {
    type Error = crate::Error;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Ok(Self(Operand::Constant(Constant::ElementF32(value.try_into()?))))
    }
}

impl TryFrom<f64> for Value {
    type Error = crate::Error;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Ok(Self(Operand::Constant(Constant::ElementF64(value.try_into()?))))
    }
}

impl TryFrom<u8> for Value {
    type Error = crate::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(Self(Operand::Constant(Constant::ElementU8(value.try_into()?))))
    }
}

impl TryFrom<u16> for Value {
    type Error = crate::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(Self(Operand::Constant(Constant::ElementU16(value.try_into()?))))
    }
}

impl TryFrom<u32> for Value {
    type Error = crate::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(Self(Operand::Constant(Constant::ElementU32(value.try_into()?))))
    }
}

impl TryFrom<u64> for Value {
    type Error = crate::Error;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Ok(Self(Operand::Constant(Constant::ElementU64(value.try_into()?))))
    }
}

impl TryFrom<i8> for Value {
    type Error = crate::Error;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        Ok(Self(Operand::Constant(Constant::ElementI8(value.try_into()?))))
    }
}

impl TryFrom<i16> for Value {
    type Error = crate::Error;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        Ok(Self(Operand::Constant(Constant::ElementI16(value.try_into()?))))
    }
}

impl TryFrom<i32> for Value {
    type Error = crate::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(Self(Operand::Constant(Constant::ElementI32(value.try_into()?))))
    }
}

impl TryFrom<i64> for Value {
    type Error = crate::Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(Self(Operand::Constant(Constant::ElementI64(value.try_into()?))))
    }
}