/*
https://en.wikipedia.org/wiki/CHIP-8
https://chip-8.github.io/links/
https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Instruction-Set
https://multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/
*/

struct CPU {
    registers: [u8; 16],
    // Normally referred to as "program counter"
    position_in_memory: usize,
    memory: [u8; 0x1000],
}

impl CPU {
    fn new() -> Self {
        Self {
            registers: [0; 16],
            position_in_memory: 0,
            memory: [0; 4096],
        }
    }

    fn read_opcode(&mut self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;
        op_byte1 << 8 | op_byte2
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2;

            // decoding opcode
            /*
            ADD         0x8014      add register 1 to register 0, store result in register 0
            CALL        0x2nnn      nnn is memory address of a function
            RETURN      0x00FF      sets mem position to previous CALL opcode
            */
            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            match (c, x, y, d) {
                (0, 0, 0, 0) => { return; },
                (8, _, _, 4) => self.add(x, y),
                _ => panic!("opcode not implemented: {:x}", opcode),
            };
        }
    }

    fn add(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }
}

fn main() {
    let mut cpu = CPU::new();

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;

    let mem = &mut cpu.memory;

    mem[0] = 0x80; mem[1] = 0x14;
    mem[2] = 0x80; mem[3] = 0x24;
    mem[4] = 0x80; mem[5] = 0x34;

    cpu.run();

    assert_eq!(cpu.registers[0], 35);
}
