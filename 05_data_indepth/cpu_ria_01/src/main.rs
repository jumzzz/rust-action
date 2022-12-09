#[derive(Debug)]
struct CPU {
    current_operation: u16,
    registers: [u8; 2],
}

impl CPU {

    fn read_opcode(&self) -> u16 {
        self.current_operation
    }

    fn run(&mut self) {
        
        let opcode = self.read_opcode();

        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = ((opcode & 0x000F) >> 0) as u8;
        

        println!("opcode = {:#04X}, {:032b}", opcode, opcode);
        println!("c (opcode group) = {:#04X}, {:032b}", c, c);
        println!("x (dst) = {:#04X}, {:032b}", x, x);
        println!("y (src) = {:#04X}, {:032b}", y, y);
        println!("d (number of bytes) = {:#04X}, {:032b}", d, d);

        match (c, x, y, d) {
            (0x8, _, _, 0x4) => self.add_xy(x, y),
            _ => todo!("opcode {:#04x}", opcode),
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize];
    }
}

fn cpu_ops() {

    let mut cpu = CPU {
        current_operation: 0,
        registers: [0; 2],
    };

    cpu.current_operation = 0x8014;
    
    cpu.registers[0] = 20;
    cpu.registers[1] = 10;

    println!("cpu.registers[0] = {}", cpu.registers[0]);
    println!("cpu.registers[1] = {}", cpu.registers[1]);

    println!("result = {}", cpu.registers[0]);

}


fn main() {
   cpu_ops(); 
}
