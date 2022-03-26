/*
https://en.wikipedia.org/wiki/CHIP-8
https://chip-8.github.io/links/
https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Instruction-Set
https://multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/
*/

struct CPU {
    // opcode: operation number and operands as registers.
    current_operation: u16,
    registers: [u8; 2],
}

impl CPU {
    fn new() -> Self {
        Self {
            // Initialisation with a No-Op - do nothing.
            current_operation: 0,
            registers: [0; 2],
        }
    }

    fn read_opcode(&mut self) -> u16 {
        self.current_operation
    }

    fn run(&mut self) {
        let opcode = self.read_opcode();

        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = ((opcode & 0x000F) >> 0) as u8;

        match (c, x, y, d) {
            (8, _, _, 4) => self.add(x, y),
            _ => panic!("opcode not implemented: {:x}", opcode),
        };
    }

    fn add(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize];
    }
}

fn main() {
    let mut cpu = CPU::new();
    // 8 signifies that the operation involves two registers.
    // 0 maps to cpu.registers[0].
    // 1 maps to cpu.registers[1].
    // 4 indicates addition.
    cpu.current_operation = 0x8014;
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    cpu.run();

    assert_eq!(cpu.registers[0], 15);
}
