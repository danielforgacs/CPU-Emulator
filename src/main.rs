struct CPU {
    registers: [u8; 16],
    // Normally referred to as "program counter"
    position_in_memory: usize,
    memory: [u8; 0x1000],
    stack: [u16; 16],
    stack_pointer: usize,
}

impl CPU {
    fn new() -> Self {
        Self {
            registers: [0; 16],
            position_in_memory: 0,
            memory: [0; 4096],
            stack: [0; 16],
            stack_pointer: 0,
        }
    }

    fn read_opcode(&mut self) -> u16 {
        /* Memory is 1 byte,
        opcodes are 2 bytes:
        2 mem addresses need to be read for 1 opcode. */
        let op_byte1 = self.memory[self.position_in_memory] as u16;
        self.position_in_memory += 1;
        let op_byte2 = self.memory[self.position_in_memory] as u16;
        self.position_in_memory += 1;

        /* To get the opcode, bits from each memory address
        are converted to 16 bits then logicall OR together to get the original opcode. */
        op_byte1 << 8 | op_byte2
    }

    fn decode_opcode(&self, opcode: u16) -> (u8, u8, u8, u8, u16) {
        // decoding opcode
        /*
        ADD         0x8014      add register 1 to register 0, store result in register 0
        CALL        0x2nnn      nnn is memory address of a function
        RETURN      0x00EE      sets mem position to previous CALL opcode
        */
        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = ((opcode & 0x000F) >> 0) as u8;
        let nnn = opcode & 0x0FFF;
        // Unused so far:
        // let kk = (opcode & 0x00FF) as u8;

        (c, x, y, d, nnn)
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();

            // // decoding opcode
            // /*
            // ADD         0x8014      add register 1 to register 0, store result in register 0
            // CALL        0x2nnn      nnn is memory address of a function
            // RETURN      0x00EE      sets mem position to previous CALL opcode
            // */
            // let c = ((opcode & 0xF000) >> 12) as u8;
            // let x = ((opcode & 0x0F00) >> 8) as u8;
            // let y = ((opcode & 0x00F0) >> 4) as u8;
            // let d = ((opcode & 0x000F) >> 0) as u8;
            // let nnn = opcode & 0x0FFF;
            // // let kk = (opcode & 0x00FF) as u8;

            let (c, x, y, d, nnn) = self.decode_opcode(opcode);

            match (c, x, y, d) {
                (0, 0, 0, 0) => { return; },
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x2, _, _, _) => self.call(nnn),
                (0x8, _, _, 0x4) => self.add(x, y),
                _ => panic!("opcode not implemented: {:x}", opcode),
            };
        }
    }

    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("stack overflow");
        }

        stack[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("stack underflow.");
        }

        self.stack_pointer -= 1;
        let addr = self.stack[self.stack_pointer];
        self.position_in_memory = addr as usize;
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

    let mem = &mut cpu.memory;

    // Adding 2 byte opcodes to 1 byte memory
    mem[0x000] = 0x21; mem[0x001] = 0x00;   // 0x2100       CALL, mem 0x100
    mem[0x002] = 0x21; mem[0x003] = 0x00;   // 0x2100       CALL, mem 0x100
    mem[0x004] = 0x00; mem[0x005] = 0x00;   // 0x0000       NOOP

    // The next 3 line act as a function with the stack.
    mem[0x100] = 0x80; mem[0x101] = 0x14;   // 0x8014       ADD, reg 0, reg 1, store result in reg 0
    mem[0x102] = 0x80; mem[0x103] = 0x14;   // 0x8014       ADD, reg 0, reg 1, store result in reg 0
    mem[0x104] = 0x00; mem[0x105] = 0xEE;   // 0x00EE       RETURN

    cpu.run();

    assert_eq!(cpu.registers[0], 45);
    println!("finished properly.");
}
