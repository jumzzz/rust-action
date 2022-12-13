pub const TYPE_SYS:      u8 = 0x0;
pub const TYPE_OPS:      u8 = 0x8;
pub const TYPE_CALL:     u8 = 0x2;

pub const SYS_RET:       u8 = 0xE;

pub const OPS_OR:        u8 = 0x1;
pub const OPS_AND:       u8 = 0x2;
pub const OPS_XOR:       u8 = 0x3;
pub const OPS_ADD:       u8 = 0x4;
pub const OPS_SUB:       u8 = 0x5;
pub const OPS_SHR:       u8 = 0x6;
pub const OPS_SUBN:      u8 = 0x7;
pub const OPS_SHL:       u8 = 0xE;


pub struct CPU {
    pub registers: [u8; 16],
    pub position_in_memory: usize,
    pub memory: [u8; 4096],
    pub stack: [u16; 16],
    pub stack_pointer: usize,
}

impl CPU {
    pub fn read_opcode(&self) -> u16 {

        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        println!("(p0,p1) = (0x{:04X}, 0x{:04X})", p, p + 1);
        println!("(o0,o1) = (0x{:04X}, 0x{:04X})", op_byte1, op_byte2);

        op_byte1 << 8 | op_byte2
    }

    pub fn run(&mut self) {

        loop {
            
            let opcode = self.read_opcode();
            self.position_in_memory += 2;
        
            // Parses opcode to c,x,y,d
            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            let nnn = opcode & 0x0FFF;

            println!("opcode = 0x{:04X}, reg[0] = {}, \t(x,y) = ({},{}), pos = 0x{:04X})", 
                    opcode, 
                    self.registers[0],
                    x,y, self.position_in_memory);
            
            match (c, x, y, d) {
                (TYPE_OPS, _, _, OPS_OR)            => self.or_xy(x, y),
                (TYPE_OPS, _, _, OPS_AND)           => self.and_xy(x, y),
                (TYPE_OPS, _, _, OPS_XOR)           => self.xor_xy(x, y),
                (TYPE_OPS, _, _, OPS_ADD)           => self.add_xy(x, y),
                (TYPE_OPS, _, _, OPS_SUB)           => self.sub_xy(x, y),
                (TYPE_OPS, _, _, OPS_SHR)           => self.shr_xy(x, y),
                (TYPE_OPS, _, _, OPS_SUBN)          => self.subn_xy(x, y),
                (TYPE_OPS, _, _, OPS_SHL)           => self.shl_xy(x, y),
                (TYPE_SYS, 0, SYS_RET, SYS_RET)     => self.ret(),
                (TYPE_CALL, _, _, _)                => self.call(nnn),
                (0, 0, 0, 0)                        => { return; } ,
                _                                   => todo!("opcode {:04x}", opcode),
            }
        }    
    }

}

/// Function Call Implementation
impl CPU {
    pub fn call(&mut self, addr: u16) {
        println!("ops = call");

        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack Overflow!")
        }

        stack[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
    }

    pub fn ret(&mut self) {
        println!("ops = ret");

        if self.stack_pointer == 0 {
            panic!("Stack Underflow")
        }

        self.stack_pointer -= 1;
        let call_addr = self.stack[self.stack_pointer];
        self.position_in_memory = call_addr as usize;
    }

}


impl CPU {

    fn or_xy(&mut self, x: u8, y: u8) {
        println!("ops = or");
    
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];
    
        self.registers[x as usize] = arg1 | arg2;
    }

    fn and_xy(&mut self, x: u8, y: u8) {
        println!("ops = and");

        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];
        
        self.registers[x as usize] = arg1 & arg2;
    }
   
    fn xor_xy(&mut self, x: u8, y: u8) {
        println!("ops = xor");

        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        println!("arg1 = {:032b}", arg1);
        println!("arg2 = {:032b}", arg2);

        self.registers[x as usize] = arg1 ^ arg2;
    }

    fn set_overflow_flag(&mut self, overflow: bool){
        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }
    
    fn add_xy(&mut self, x: u8, y: u8) {
        println!("ops = add");

        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);  // overflow returns as true when overflow is detected
        self.registers[x as usize] = val;

        self.set_overflow_flag(overflow);
    }

    fn sub_xy(&mut self, x: u8, y: u8) {
        println!("ops = sub");

        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];
        
        let (val, overflow) = arg1.overflowing_sub(arg2);
        self.registers[x as usize] = val;
        self.set_overflow_flag(overflow);
    }

    fn shr_xy(&mut self, x: u8, y: u8) {
        println!("ops = shr");

        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_shr(arg2 as u32);

        println!("arg1 = {:032b}", arg1);
        println!("arg2 = {}", arg2);
        
        self.registers[x as usize] = val;

        self.set_overflow_flag(overflow);
    }

    fn subn_xy(&mut self, x: u8, y: u8) {
        println!("ops = subn");

        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg2.overflowing_sub(arg1);
        self.registers[x as usize] = val;
        self.set_overflow_flag(overflow);

    }

    fn shl_xy(&mut self, x: u8, y: u8) {
        println!("ops = shl");

        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];
        
        let (val, overflow) = arg1.overflowing_shl(arg2 as u32);

        self.registers[x as usize] = val;
        self.set_overflow_flag(overflow);
    }


}