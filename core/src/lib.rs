mod engine;

mod layer;
mod layer_value;
mod layer_trainables;
mod kernel;
mod tensor;
mod element;
mod op_builder;
mod dimension;
mod error;

use error::ErrorVariants as Errors;
pub use error::Error;
pub use error::Result;
pub mod layers;
pub use engine::Engine;
pub use op_builder::OpBuilder;
pub use layer::Layer;
pub use layer_value::LayerValue;
pub use layer_trainables::LayerTrainables;
pub use kernel::Kernel;
pub use dimension::Dimension;
pub use tensor::Tensor;
pub use element::Element;