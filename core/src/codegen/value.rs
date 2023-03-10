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
    convergent: Convergent,
    inner: Operand,
    general_type: GeneralType,
}

#[derive(PartialEq)]
enum Convergent {
    None,
    Convolve((u32, u32, u32), (u32, u32), (u32, u32)),
}

impl Value {
    #[allow(clippy::should_implement_trait)]
    pub fn add(self, operand: impl Into<Self>) -> crate::Result<Self> {
        let operand = operand.into();
        
        // Check operand types
        match (self.general_type, operand.general_type) {
            (GeneralType::Tensor(ax, ay, az, at), GeneralType::Tensor(bx, by, bz, bt)) => {
                if at != bt {
                    return Errors::DifferentOperandTypes.into();
                }
    
                if ax != bx || ay != by || az != bz {
                    return Errors::DifferentOperandDimensions.into();
                }

                Ok(Self {
                    inner: Operand::Node(Box::new(Node::Add(self.inner, operand.inner))),
                    general_type: GeneralType::Tensor(ax, ay, az, at),
                    convergent: self.convergent
                })
            },
            (GeneralType::Element(ElementType(an, at)), GeneralType::Element(ElementType(bn, bt))) => {
                if at != bt {
                    return Errors::DifferentOperandTypes.into();
                }

                if an != bn {
                    return Errors::DifferentOperandDimensions.into();
                }

                Ok(Self {
                    inner: Operand::Node(Box::new(Node::Add(self.inner, operand.inner))),
                    general_type: self.general_type,
                    convergent: self.convergent
                })
            },
            _ => { Errors::InvalidOperandTypes.into() }
        }
    }

    pub fn subtract(self, operand: impl Into<Self>) -> crate::Result<Self> {
        let operand = operand.into();
        
        // Check operand types
        match (self.general_type, operand.general_type) {
            (GeneralType::Tensor(ax, ay, az, at), GeneralType::Tensor(bx, by, bz, bt)) => {
                if at != bt {
                    return Errors::DifferentOperandTypes.into();
                }
    
                if ax != bx || ay != by || az != bz {
                    return Errors::DifferentOperandDimensions.into();
                }

                Ok(Self {
                    inner: Operand::Node(Box::new(Node::Add(self.inner, operand.inner))),
                    general_type: GeneralType::Tensor(ax, ay, az, at),
                    convergent: self.convergent
                })
            },
            (GeneralType::Element(ElementType(an, at)), GeneralType::Element(ElementType(bn, bt))) => {
                if at != bt {
                    return Errors::DifferentOperandTypes.into();
                }

                if an != bn {
                    return Errors::DifferentOperandDimensions.into();
                }

                Ok(Self {
                    inner: Operand::Node(Box::new(Node::Subtract(self.inner, operand.inner))),
                    general_type: self.general_type,
                    convergent: self.convergent
                })
            },
            _ => { Errors::InvalidOperandTypes.into() }
        }
    }
    
    pub fn multiply(self, operand: impl Into<Self>) -> crate::Result<Self> {
        let operand = operand.into();

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
            general_type,
            convergent: self.convergent
        })
    }

    pub fn hadamard_product(self, operand: impl Into<Self>) -> crate::Result<Self> {
        let operand = operand.into();

        // Start checking operands
        match (self.general_type, operand.general_type) {
            (GeneralType::Tensor(ax, ay, az, at), GeneralType::Tensor(bx, by, bz, bt)) => {
                if at != bt {
                    return Errors::DifferentOperandTypes.into();
                }
    
                if ax != bx || ay != by || az != bz {
                    return Errors::DifferentOperandDimensions.into();
                }

                Ok(Self {
                    inner: Operand::Node(Box::new(Node::HadamardProduct(self.inner, operand.inner))), 
                    general_type: self.general_type,
                    convergent: self.convergent
                })
            },
            _ => { Errors::InvalidOperandTypes.into() }
        }
    }

    pub fn divide(self, operand: impl Into<Self>) -> crate::Result<Self> {
        let operand = operand.into();

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
            general_type,
            convergent: self.convergent
        })
    }

    pub fn convolve(mut self, size: (u32, u32), stride: (u32, u32)) -> crate::Result<Self> {

        if self.convergent != Convergent::None {
            return Errors::UnableToConvolve.into();
        }

        let general_type = match self.general_type {
            GeneralType::Tensor(x, y, z, ty) => {
                self.convergent = Convergent::Convolve((x, y, z), size, stride);

                GeneralType::Tensor(size.0, size.1, z, ty)
            },
            GeneralType::Element(_) => {
                return Errors::RequiresTensor.into();
            }
        };


        Ok(Self {
            inner: Operand::Node(Box::new(Node::Convolve(self.inner, size, stride))),
            general_type,
            convergent: self.convergent
        })
    }

    pub fn converge_sum(self) -> crate::Result<Self> {
        let general_type = match self.convergent {
            Convergent::None => {
                return Errors::UnableToConvergeOperand.into();
            },
            Convergent::Convolve(input, filter, stride)
                => if let GeneralType::Element(element) = self.general_type {
                let x = (input.0 - filter.0)/ stride.0 + 1;
                let y = (input.1 - filter.1)/ stride.1 + 1;
                GeneralType::Tensor(x, y, input.2, element)
            } else {
                return Errors::UnableToConvergeOperand.into();
            },
        };

        Ok(Self {
            inner: Operand::Node(Box::new(Node::ConvergeSum(self.inner))),
            general_type,
            convergent: Convergent::None,
        })
    }

    pub fn activation(self, activation_fn: &crate::ActivationFunction) -> crate::Result<Self> {
        match activation_fn {
            crate::ActivationFunction::Sigmoid => self.sigmoid(),
            crate::ActivationFunction::Tanh => self.tanh(),
            crate::ActivationFunction::Relu => self.relu(),
            crate::ActivationFunction::LeakyRelu(beta) => self.leaky_relu(*beta),
            crate::ActivationFunction::Elu => self.elu(),
            crate::ActivationFunction::Swish => self.swish(),
            crate::ActivationFunction::Softplus(beta) => self.softplus(*beta),
        }
    }

    pub fn sigmoid(self) -> crate::Result<Self> {
        Ok(Self {
            inner: Operand::Node(Box::new(Node::Sigmoid(self.inner))),
            general_type: self.general_type,
            convergent: self.convergent
        })
    }

    pub fn tanh(self) -> crate::Result<Self> {
        Ok(Self {
            inner: Operand::Node(Box::new(Node::Tanh(self.inner))),
            general_type: self.general_type,
            convergent: self.convergent
        })
    }

    pub fn relu(self) -> crate::Result<Self> {
        Ok(Self {
            inner: Operand::Node(Box::new(Node::Relu(self.inner))),
            general_type: self.general_type,
            convergent: self.convergent
        })
    }

    pub fn leaky_relu(self, beta: f32) -> crate::Result<Self> {
        Ok(Self {
            inner: Operand::Node(Box::new(Node::LeakyRelu(self.inner, beta))),
            general_type: self.general_type,
            convergent: self.convergent
        })
    }

    pub fn elu(self) -> crate::Result<Self> {
        Ok(Self {
            inner: Operand::Node(Box::new(Node::Elu(self.inner))),
            general_type: self.general_type,
            convergent: self.convergent
        })
    }

    pub fn swish(self) -> crate::Result<Self> {
        Ok(Self {
            inner: Operand::Node(Box::new(Node::Swish(self.inner))),
            general_type: self.general_type,
            convergent: self.convergent
        })
    }

    pub fn softplus(self, beta: f32) -> crate::Result<Self> {
        Ok(Self {
            inner: Operand::Node(Box::new(Node::Softplus(self.inner, beta ))),
            general_type: self.general_type,
            convergent: self.convergent
        })
    }
}

impl From<f32> for Value {

    fn from(value: f32) -> Self {
        Self {
            inner: Operand::Constant(Constant::ScalarF32(value)),
            general_type: GeneralType::Element(ElementType(1, ScalarType::F32)),
            convergent: Convergent::None,
        }
    }
}

impl From<f64> for Value {

    fn from(value: f64) -> Self {
        Self {
            inner: Operand::Constant(Constant::ScalarF64(value)),
            general_type: GeneralType::Element(ElementType(1, ScalarType::F64)),
            convergent: Convergent::None,
        }
    }
}

impl From<u8> for Value {

    fn from(value: u8) -> Self {
        Self {
            inner: Operand::Constant(Constant::ScalarU8(value)),
            general_type: GeneralType::Element(ElementType(1, ScalarType::U8)),
            convergent: Convergent::None,
        }
    }
}

impl From<u16> for Value {

    fn from(value: u16) -> Self {
        Self {
            inner: Operand::Constant(Constant::ScalarU16(value)),
            general_type: GeneralType::Element(ElementType(1, ScalarType::U16)),
            convergent: Convergent::None,
        }
    }
}

impl From<u32> for Value {

    fn from(value: u32) -> Self {
        Self {
            inner: Operand::Constant(Constant::ScalarU32(value)),
            general_type: GeneralType::Element(ElementType(1, ScalarType::U32)),
            convergent: Convergent::None,
        }
    }
}

impl From<u64> for Value {

    fn from(value: u64) -> Self {
        Self {
            inner: Operand::Constant(Constant::ScalarU64(value)),
            general_type: GeneralType::Element(ElementType(1, ScalarType::U64)),
            convergent: Convergent::None,
        }
    }
}

impl From<i8> for Value {


    fn from(value: i8) -> Self {
        Self {
            inner: Operand::Constant(Constant::ScalarI8(value)),
            general_type: GeneralType::Element(ElementType(1, ScalarType::I8)),
            convergent: Convergent::None,
        }
    }
}

impl From<i16> for Value {

    fn from(value: i16) -> Self {
        Self {
            inner: Operand::Constant(Constant::ScalarI16(value)),
            general_type: GeneralType::Element(ElementType(1, ScalarType::I16)),
            convergent: Convergent::None,
        }
    }
}

impl From<i32> for Value {

    fn from(value: i32) -> Self {
        Self {
            inner: Operand::Constant(Constant::ScalarI32(value)),
            general_type: GeneralType::Element(ElementType(1, ScalarType::I32)),
            convergent: Convergent::None,
        }
    }
}

impl From<i64> for Value {

    fn from(value: i64) -> Self {
        Self {
            inner: Operand::Constant(Constant::ScalarI64(value)),
            general_type: GeneralType::Element(ElementType(1, ScalarType::I32)),
            convergent: Convergent::None,
        }
    }
}