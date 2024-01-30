// Declare memory-related submodules
pub mod allocator;
pub mod paging;

// Export symbols
pub use allocator::*;
pub use paging::*;

// Other memory-related functionality can go here
