mod engine;

mod layer;
mod layer_input;
mod layer_output;
mod layer_trainables;
mod kernel;
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
pub use layer_input::LayerInput;
pub use layer_output::LayerOutput;
pub use layer_trainables::LayerTrainables;
pub use kernel::Kernel;
pub use dimension::Dimension;