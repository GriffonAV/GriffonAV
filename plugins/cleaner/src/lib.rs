pub mod config;
pub mod context;
pub mod reports;
pub mod runner;
pub mod modules;
pub mod cache_paths;

pub use config::*;
pub use context::*;
pub use reports::*;
pub use runner::*;
pub use cache_paths::*;
pub use modules::CleanerModule;

pub type CleanerResult<T> = Result<T, CleanerError>;

#[derive(thiserror::Error, Debug)]
pub enum CleanerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Walkdir error: {0}")]
    Walkdir(#[from] walkdir::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub fn default_modules() -> Vec<Box<dyn CleanerModule>> {
    vec![
        Box::new(modules::cache::CacheCleaner::new()),
        Box::new(modules::logs::LogsCleaner::new()),
        Box::new(modules::packages::PackagesCleaner::new()),
        Box::new(modules::bigfiles::BigfilesScanner::new()),
    ]
}
