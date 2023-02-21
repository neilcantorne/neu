mod backend;
mod codegen;
mod layer;
mod layer_trainables;
mod tensor;
mod element;
mod dimension;
mod error;
mod activation_function;

use error::ErrorVariants as Errors;
pub use error::Error;
pub use error::Result;
pub use backend::{ Device, Engine, BackendApi };
pub mod layers;
pub use codegen::Value;
pub use layer::Layer;
pub use layer_trainables::LayerTrainables;
pub use dimension::Dimension;
pub use tensor::Tensor;
pub use activation_function::ActivationFunction;
pub use element::{ Element, ChannelCount };

mod tests;