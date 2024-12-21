

#[derive(Copy, Clone, Debug, Default)]
struct Address(u16);

#[derive(Copy, Clone, Debug, Default, PartialEq)]
struct Byte(u8);

#[derive(Copy, Clone, Debug, Default)]
struct Word(u16);


const MEMORY_SIZE: usize = 4 * 1024; // 4kb of memory
const REGISTERS_COUNT: usize = 6; // Count of cpu registers
const STACK_SIZE: usize = 16; // Number of levels in the call stack
const DISPLAY_SIZE: (usize, usize) = (64, 32); // Display size as a tuple (w, h)
const KEY_COUNT: usize = 16; // Count of input keys


// TODO(Issam): Add docs for where to find this stuff and what it means
// It helps long term
#[derive(Debug, Default)]
struct Chip8 {
      memory: Vec<Byte>,
}

impl Chip8 {
  fn new() -> Self {
    Chip8 {
      memory: vec![Byte(0); MEMORY_SIZE],
      ..Default::default()
    }
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let chip8= Chip8::new();
        assert_eq!(chip8.memory.len(), MEMORY_SIZE, "Memory should have correct size");
        assert!(chip8.memory.iter().all(|&byte| byte == Byte(0)), "Memory should be initialized to 0");
    }
}

