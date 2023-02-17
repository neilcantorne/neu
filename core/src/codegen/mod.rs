mod value;
mod general_type;
mod scalar_type;
mod node;
mod element;
mod operand;

pub use value::Value;
use general_type::GeneralType;
use scalar_type::ScalarType;
use node::Node;
use element::Element;
use operand::{
    IntoOperand,
    Operand
};