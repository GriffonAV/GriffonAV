pub struct SampleModule;

impl Module for SampleModule {
    fn name(&self) -> &str {
        "SampleModule"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    fn initialize(&self) {
        println!("{} v{} initialized.", self.name(), self.version());
    }

    fn shutdown(&self) {
        println!("{} v{} shutting down.", self.name(), self.version());
    }

    fn anytime_run(&self) {
        println!("{} is running anytime tasks.", self.name());
    }
}
