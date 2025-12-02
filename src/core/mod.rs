pub mod progress;
pub mod binary;
pub mod merger;

pub use merger::merge_binaries;
pub use binary::{Architecture, OperatingSystem, BinaryInfo};
