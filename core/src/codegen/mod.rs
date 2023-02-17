mod value;
mod general_type;
mod scalar_type;
mod node;
mod constant;
mod element_type;
mod operand;

pub use value::Value;
use general_type::GeneralType;
use scalar_type::ScalarType;
use node::Node;
use element_type::ElementType;
use constant::Constant;
use operand::{
    IntoOperand,
    Operand
};