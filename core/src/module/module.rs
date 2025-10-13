pub trait Module: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &'static str;
    fn initialize(&self);
    fn shutdown(&self);

    fn anytime_run(&self);
}
