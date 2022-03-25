struct CPU {
    // opcode: operation number and operands as registers.
    current_operation: u16,
    registers: [u8; 2],
}

impl CPU {
    fn new() -> Self {
        Self {
            current_operation: 0,
            registers: [0; 2],
        }
    }
}

fn main() {
    let mut cpu = CPU::new();
}
