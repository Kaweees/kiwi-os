// Declare submodules
pub mod entry;
pub mod interrupts;
pub mod scheduler;
pub mod memory;

// Export symbols
pub use entry::init;

// Other kernel-wide functionality can go here
