struct CPU {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 4096],
    stack: [u16; 16],
    stack_pointer: usize
}

impl CPU {
    fn run(&mut self) {
        loop {
            let op_byte1 = self.memory[self.position_in_memory] as u16;
            let op_byte2 = self.memory[self.position_in_memory + 1] as u16;
            let opcode: u16 = op_byte1 << 8 | op_byte2;

            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let kk = (opcode & 0x00FF) as u8;
            let op_miner = (opcode & 0x000F) as u8;
            let addr = opcode & 0x0FFF;

            self.position_in_memory += 2;
            match opcode {
                0x0000 => { return; },
                0x00E0 => { },
                0x00EE => { self.ret(); },
                0x1000..=0x1FFF => { self.jmp(addr); },
                0x2000..=0x2FFF => { self.call(addr); },
                0x3000..=0x3FFF => { self.se(x, kk); },
                0x4000..=0x4FFF => { self.sne(x, kk); },
                0x5000..=0x5FFF => { self.se(x, y); },
                0x6000..=0x6FFF => { self.ld(x, kk); },
                0x7000..=0x7FFF => { self.add(x, kk); },
                0x8000..=0x8FFF => { 
                    match op_miner {
                        0 => { self.ld(x, self.registers[y as usize])},
                        1 => { self.or_xy(x, y)},
                        2 => { self.and_xy(x, y)},
                        3 => { self.xor_xy(x, y)},
                        4 => { self.add_xy(x, y)},
                        _ => { todo!("opcode: {:04x}", opcode); },
                    }
                },
                _ => { todo!("opcode: {:04x}", opcode); },
            }
        }
    }

    fn ld(&mut self, vx: u8, kk: u8) {
        self.registers[vx as usize] = kk;
    }

    fn add(&mut self, vx: u8, kk: u8) {
        self.registers[vx as usize] += kk;
    }

    fn se(&mut self, vx: u8, kk: u8) {
        if vx == kk {
            self.position_in_memory += 2;
        }
    }

    fn sne(&mut self, vx: u8, kk:u8) {
        if vx != kk {
            self.position_in_memory += 2;
        }
    }

    fn jmp(&mut self, addr: u16) {
        self.position_in_memory = addr as usize;
    }

    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp >= stack.len() {
            panic!("Stack Overflow");
        }

        stack[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack overflow");
        }
        self.stack_pointer -= 1;
        self.position_in_memory = self.stack[self.stack_pointer] as usize;
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];
    //    println!("{} {}", arg1, arg2);
        let (val, overflow) = arg1.overflowing_add(arg2);

        self.registers[x as usize] = val;
    //    println!("{}", val);
        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    fn and_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[x as usize ] & self.registers[y as usize];
    }

    fn or_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[x as usize] | self.registers[y as usize];
    }

    fn xor_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[x as usize] ^ self.registers[y as usize];
    }


}



fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
        stack: [0; 16],
        stack_pointer: 0
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    let mem = &mut cpu.memory;
    mem[0x000] = 0x21; mem[0x0001] = 0x00; // ; call 0x100
    mem[0x002] = 0x21; mem[0x0003] = 0x00; // ; call 0x100
    mem[0x004] = 0x00; mem[0x0005] = 0x00;

    mem[0x100] = 0x80; mem[0x101] = 0x14; // ; add reg1, reg2
    mem[0x102] = 0x80; mem[0x103] = 0x14; // ; add reg1, reg2
    mem[0x104] = 0x00; mem[0x105] = 0xEE; // ; ret

    cpu.run();

    assert_eq!(cpu.registers[0], 45);

    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}