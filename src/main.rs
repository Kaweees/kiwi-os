#![no_std] // Disable the standard library

// Include
mod kernel;
mod drivers;
mod utils;
mod board;
mod shell;

// Entry point of the operating system
fn main() {
  // Initialize board-specific peripherals
  board::initialization::init();

  // Initialize the drivers
  drivers::timer::init();
  drivers::gpio::init();
  drivers::uart::init();

  // Initialize the kernel
  kernel::entry::init();

  // Initialize the shell
  shell::init();
  // How did we get here? Hang the processor if somehow reached.
  loop {}
}
