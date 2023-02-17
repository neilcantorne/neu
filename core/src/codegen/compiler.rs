use inkwell::OptimizationLevel;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;

pub struct Compiler<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
}

impl<'ctx> Compiler<'ctx> {
    pub fn build(&self, graph: super::graph::Graph) {
        self.module.add_function("", ty, None);
        //self.module.
    }
}