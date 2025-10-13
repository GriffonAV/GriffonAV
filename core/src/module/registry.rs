use std::sync::Arc;

pub struct ModuleRegistry {
    modules: Vec<Arc<dyn Module>>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self { modules: Vec::new() }
    }

    pub fn register<M: Module + 'static>(&mut self, module: M) {
        self.modules.push(Arc::new(module));
    }

    pub fn get_modules(&self) -> &[Arc<dyn Module>] {
        &self.modules
    }
}