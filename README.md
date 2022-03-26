### CHIP-8 CPU Emulator

[wikipedia](https://en.wikipedia.org/wiki/CHIP-8)  
[A curated list of awesome CHIP-8 resources](https://chip-8.github.io/links/)  
[CHIP‐8 Instruction Set](https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Instruction-Set)  
[https://multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/](https://multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/)  

An cpu emulator for the CHIP-8 cpu written in rust.


### CPU Specs:

- memory:     8bit,   4096
- registers:  8bit,   16
- stack:      16bit,  16


### Supported opcodes:

    0x0000      NOOP        
    0x8xy4      ADD         add register y to x, store result in register x
    0x2nnn      CALL        nnn is memory address of a function
    0x00EE      RETURN      sets mem position to previous CALL opcode
