struct CPU {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 0x1000]
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;
//         10000000 ; op_byte1 0x80
// 1000000000000000 ; op_byte << 8 shift it to left to make room for rest of 8bits
//         00010100 ; op_byte2 0x14
// 1000000000010100 ; or both to get all 1bits 0x8014
        op_byte1 << 8 | op_byte2
    }

    fn run(&mut self) {
         loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >>  8) as u8;
            let y = ((opcode & 0x00F0) >>  4) as u8;
            let d = ((opcode & 0x000F) >>  0) as u8;

            match (c, x, y, d) {
                (0, 0, 0, 0) => { return; },
                (0x8, _, _, 0x4) => self.add_xy(x,y),
                _ => todo!("opcode {:04x}", opcode)
            }
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
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
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0
    };

    // Store the values in registers
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;

    let mem = &mut cpu.memory;

    // Three instructions
    // add reg0 += reg1 ; 0x8014
    // add reg0 += reg2 ; 0x8024
    // add reg0 += reg3 ; 0x8034
    mem[0] = 0x80; mem[1] = 0x14;
    mem[2] = 0x80; mem[3] = 0x24;
    mem[4] = 0x80; mem[5] = 0x34;

    cpu.run();

    assert_eq!(cpu.registers[0], 35);

    println!("5 + 10 + 10 + 10 = {}", cpu.registers[0]);
    
    
}


// Debug

// let c = ((opcode & 0xF000) >> 12) as u8;
// println!("{:016b}",(opcode));
// println!("{:016b}",(0xF000));
// println!("{:016b}",(opcode & 0xF000));
// println!("{:016b}",(opcode & 0xF000) >> 12);
// println!("");
// let x = ((opcode & 0x0F00) >>  8) as u8;
// println!("{:016b}",(opcode));
// println!("{:016b}",(0x0F00));
// println!("{:016b}",(opcode & 0x0F00));
// println!("{:016b}",(opcode & 0x0F00) >> 8);
// println!("");
// let y = ((opcode & 0x00F0) >>  4) as u8;
// println!("{:016b}",(opcode));
// println!("{:016b}",(0x00F0));
// println!("{:016b}",(opcode & 0x00F0));
// println!("{:016b}",(opcode & 0x00F0) >> 4);
// println!("");
// let d = ((opcode & 0x000F) >>  0) as u8;
// println!("{:016b}",(opcode));
// println!("{:016b}",(0x000F));
// println!("{:016b}",(opcode & 0x000F));
// println!("{:016b}",(opcode & 0x000F) >> 0);
// println!("");
