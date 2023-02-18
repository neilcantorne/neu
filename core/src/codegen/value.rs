use super::{
    Operand,
    Constant,
    Node,
    GeneralType,
    ElementType,
    ScalarType,
};

use crate::Errors;

pub struct Value {
    pub(super) inner: Operand,
    pub(super) general_type: GeneralType,
}

impl Value {
    #[allow(clippy::should_implement_trait)]
    pub fn add(self, operand: Self) -> crate::Result<Self> {
        // Check operand types
        match (self.general_type, operand.general_type) {
            (GeneralType::Tensor(ax, ay, az, at), GeneralType::Tensor(bx, by, bz, bt)) => {
                if at != bt {
                    return Errors::DifferentOperandTypes.into();
                }
    
                if ax != bx || ay != by || az != bz {
                    return Errors::DifferentOperandDimension.into();
                }

                Ok(Self {
                    inner: Operand::Node(Box::new(Node::Add(self.inner, operand.inner))),
                    general_type: GeneralType::Tensor(ax, ay, az, at)
                })
            },
            (GeneralType::Element(ElementType(an, at)), GeneralType::Element(ElementType(bn, bt))) => {
                if at != bt {
                    return Errors::DifferentOperandTypes.into();
                }

                if an != bn {
                    return Errors::DifferentOperandDimension.into();
                }

                Ok(Self {
                    inner: Operand::Node(Box::new(Node::Add(self.inner, operand.inner))),
                    general_type: self.general_type
                })
            },
            _ => { Errors::InvalidOperandTypes.into() }
        }
    }
    
    pub fn multiply(self, operand: Self) -> crate::Result<Self> {
        let general_type = match (self.general_type, operand.general_type) {
            (GeneralType::Tensor(ax, ay, az, at), GeneralType::Tensor(bx, by, bz, bt)) => {
                if at != bt {
                    return Errors::DifferentOperandTypes.into();
                }
                
                if ay != bx || az != bz {
                    return Errors::IncompatibleOperandDimensions.into();
                }

                GeneralType::Tensor(ay, bx, bz, at)
            },
            (GeneralType::Tensor(_, _, _, at), GeneralType::Element(bt)) => {
                if at != bt {
                    return Errors::DifferentOperandTypes.into();
                }

                self.general_type
            },
            (GeneralType::Element(at), GeneralType::Tensor(_, _, _, bt)) => {
                if at != bt {
                    return Errors::DifferentOperandTypes.into();
                }

                operand.general_type
            }
            (GeneralType::Element(at), GeneralType::Element(bt)) => {
                if at != bt {
                    return Errors::DifferentOperandTypes.into();
                }

                self.general_type
            },
        };

        Ok(Self {
            inner: Operand::Node(Box::new(Node::Multiply(self.inner, operand.inner))),
            general_type
        })
    }

    pub fn hadamard_product(self, operand: Self) -> crate::Result<Self> {

        // Start checking operands
        match (self.general_type, operand.general_type) {
            (GeneralType::Tensor(ax, ay, az, at), GeneralType::Tensor(bx, by, bz, bt)) => {
                if at != bt {
                    return Errors::DifferentOperandTypes.into();
                }
    
                if ax != bx || ay != by || az != bz {
                    return Errors::DifferentOperandDimension.into();
                }

                Ok(Self {
                    inner: Operand::Node(Box::new(Node::HadamardProduct(self.inner, operand.inner))), 
                    general_type: self.general_type
                })
            },
            _ => { return Errors::InvalidOperandTypes.into(); }
        }
    }

    pub fn divide(self, operand: Self) -> crate::Result<Self> {

        let general_type = match (self.general_type, operand.general_type) {
            (GeneralType::Tensor(_, _, _, _), GeneralType::Tensor(_, _, _, _)) => {
                return Errors::IncompatibleOperandTypes.into();
            },
            (GeneralType::Tensor(_, _, _, at), GeneralType::Element(bt)) => if at != bt {
                return Errors::DifferentOperandTypes.into();
            } else { self.general_type },
            (GeneralType::Element(at), GeneralType::Tensor(_, _, _, bt)) => if at != bt {
                return Errors::DifferentOperandTypes.into();
            } else { operand.general_type },
            (GeneralType::Element(at), GeneralType::Element(bt)) => if at != bt {
                return Errors::DifferentOperandTypes.into();
            } else { self.general_type },
        };

        Ok(Self {
            inner: Operand::Node(Box::new(Node::Divide(self.inner, operand.inner))),
            general_type
        })
    }
}

impl TryFrom<f32> for Value {
    type Error = crate::Error;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Ok(Self {
            inner: Operand::Constant(Constant::ElementF32(value.try_into()?)),
            general_type: GeneralType::Element(ElementType(1, ScalarType::F32))
        })
    }
}

impl TryFrom<f64> for Value {
    type Error = crate::Error;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Ok(Self {
            inner: Operand::Constant(Constant::ElementF64(value.try_into()?)),
            general_type: GeneralType::Element(ElementType(1, ScalarType::F64))
        })
    }
}

impl TryFrom<u8> for Value {
    type Error = crate::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(Self {
            inner: Operand::Constant(Constant::ElementU8(value.try_into()?)),
            general_type: GeneralType::Element(ElementType(1, ScalarType::U8))
        })
    }
}

impl TryFrom<u16> for Value {
    type Error = crate::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(Self {
            inner: Operand::Constant(Constant::ElementU16(value.try_into()?)),
            general_type: GeneralType::Element(ElementType(1, ScalarType::U16))
        })
    }
}

impl TryFrom<u32> for Value {
    type Error = crate::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(Self {
            inner: Operand::Constant(Constant::ElementU32(value.try_into()?)),
            general_type: GeneralType::Element(ElementType(1, ScalarType::U32))
        })
    }
}

impl TryFrom<u64> for Value {
    type Error = crate::Error;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Ok(Self {
            inner: Operand::Constant(Constant::ElementU64(value.try_into()?)),
            general_type: GeneralType::Element(ElementType(1, ScalarType::U64))
        })
    }
}

impl TryFrom<i8> for Value {
    type Error = crate::Error;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        Ok(Self {
            inner: Operand::Constant(Constant::ElementI8(value.try_into()?)),
            general_type: GeneralType::Element(ElementType(1, ScalarType::I8))
        })
    }
}

impl TryFrom<i16> for Value {
    type Error = crate::Error;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        Ok(Self {
            inner: Operand::Constant(Constant::ElementI16(value.try_into()?)),
            general_type: GeneralType::Element(ElementType(1, ScalarType::I16))
        })
    }
}

impl TryFrom<i32> for Value {
    type Error = crate::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(Self {
            inner: Operand::Constant(Constant::ElementI32(value.try_into()?)),
            general_type: GeneralType::Element(ElementType(1, ScalarType::I32))
        })
    }
}

impl TryFrom<i64> for Value {
    type Error = crate::Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(Self {
            inner: Operand::Constant(Constant::ElementI64(value.try_into()?)),
            general_type: GeneralType::Element(ElementType(1, ScalarType::F64))
        })
    }
}