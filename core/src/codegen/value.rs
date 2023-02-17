use super::{
    Operand,
    Constant,
    Node,
    GeneralType,
    ElementType,
};

use crate::Errors;

pub struct Value(pub(super) Operand);

impl Value {
    #[allow(clippy::should_implement_trait)]
    pub fn add(self, operand: Value) -> crate::Result<Self> {

        // Check operand types
        match (self.0.general_type(), operand.0.general_type()) {
            (GeneralType::Tensor(ax, ay, az, at), GeneralType::Tensor3(bx, by, bz, bt)) => {
                if at != bt {
                    return Errors::DifferentOperandTypes.into();
                }
    
                if ax != bx || ay != by || az != bz {
                    return Errors::DifferentOperandDimension.into();
                }
            },
            (GeneralType::Element(ElementType(an, at)), GeneralType::Element(ElementType(bn, bt))) => {
                if at != bt {
                    return Errors::DifferentOperandTypes.into();
                }

                if an != bn {
                    return Errors::DifferentOperandDimension.into();
                }
            },
            _ => { return Errors::InvalidOperandTypes.into(); }
        }

        Ok(Self(Operand::Node(Box::new(Node::Add(self.0, operand.0)))))
    }
    
    pub fn multiply(self, operand: Value) -> crate::Result<Self> {
        match (self.0.general_type(), operand.0.general_type()) {
            (GeneralType::Tensor(ax, ay, az, at), GeneralType::Tensor(bx, by, bz, bt)) => {
                if at != bt {
                    return Errors::DifferentOperandTypes.into();
                }
                
                if ay != bx || az != bz {
                    return Errors::IncompatibleOperandDimensions.into();
                }
            },
            (GeneralType::Tensor(_, _, _, at), GeneralType::Element(bt)) => {
                if at != bt {
                    return Errors::DifferentOperandTypes.into();
                }
            },
            (GeneralType::Element(at), GeneralType::Tensor(_, _, _, bt)) => {
                if at != bt {
                    return Errors::DifferentOperandTypes.into();
                }
            }
            (GeneralType::Element(at), GeneralType::Element(abt)) => {
                if at != bt {
                    return Errors::DifferentOperandTypes.into();
                }
            }
        }

        Ok(Self(Operand::Node(Box::new(Node::Multiply(self.0, operand.0)))))
    }

    pub fn hadamard(self, operand: Value) -> crate::Result<Self> {

        // Start checking operands
        match (self.0.general_type(), operand.0.general_type()) {
            (GeneralType::Tensor(ax, ay, az, at), GeneralType::Tensor(bx, by, bz, bt)) => {
                if at != bt {
                    return Errors::DifferentOperandTypes.into();
                }
    
                if ax != bx || ay != by || az != bz {
                    return Errors::DifferentOperandDimension.into();
                }
            },
            _ => { return Errors::InvalidOperandTypes.into(); }
        }

        Ok(Self(Operand::Node(Box::new(Node::HadamardProduct(self.0, operand.0)))))
    }
}

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