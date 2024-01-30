// Import necessary modules
use crate::memory;
use crate::interrupts;
use crate::scheduler

// The entry point of the kernel
pub fn init() {
  // Initialize the memory subsystem
  memory::init();
  // Initialize the interrupt subsystem
  interrupts::init();
  // Initialize the scheduler
  scheduler::init();
  // Initialize the filesystem
  filesystem::init();
}
