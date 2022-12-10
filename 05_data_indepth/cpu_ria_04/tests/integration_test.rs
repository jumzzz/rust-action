
#[cfg(test)]
mod tests {
    use cpu_ria_04::cpu::CPU;

    #[test]
    fn test_or_xy() {
        println!("test_or_xy()");
        
        let mut cpu = CPU {
            registers: [0; 16],
            memory: [0; 4096],
            position_in_memory: 0,
            stack: [0; 16],
            stack_pointer: 0,
        };
    
        cpu.registers[0] = 0xFF;
        cpu.registers[1] = 0x00;
    
        let mem = &mut cpu.memory;

        mem[0x000] = 0x21; mem[0x001] = 0x00;
    
        mem[0x100] = 0x80; mem[0x101] = 0x11;
        mem[0x104] = 0x00; mem[0x105] = 0xEE;
    
        cpu.run();
    
        println!("cpu.registers[0] = 0x{:04X}", cpu.registers[0]);
        println!("");
        assert_eq!(cpu.registers[0],0xFF);
    

    }
    
    #[test]
    fn test_and_xy() {
        println!("test_and_xy()");
        
        let mut cpu = CPU {
            registers: [0; 16],
            memory: [0; 4096],
            position_in_memory: 0,
            stack: [0; 16],
            stack_pointer: 0,
        };
    
        cpu.registers[0] = 0xFF;
        cpu.registers[1] = 0x00;
    
        let mem = &mut cpu.memory;

        mem[0x000] = 0x21; mem[0x001] = 0x00;
        mem[0x100] = 0x80; mem[0x101] = 0x12;
        mem[0x104] = 0x00; mem[0x105] = 0xEE;
    
        cpu.run();
    
        println!("cpu.registers[0] = 0x{:04X}", cpu.registers[0]);
        println!(" "); 
        assert_eq!(cpu.registers[0], 0x00);
    
    }
    
    #[test]
    fn test_xor_xy() { 
        println!("test_xor_xy()");
        
        let mut cpu = CPU {
            registers: [0; 16],
            memory: [0; 4096],
            position_in_memory: 0,
            stack: [0; 16],
            stack_pointer: 0,
        };
    
        cpu.registers[0] = 0xFF;
        cpu.registers[1] = 0x00;
    
        let mem = &mut cpu.memory;

        mem[0x000] = 0x21; mem[0x001] = 0x00;
        mem[0x100] = 0x80; mem[0x101] = 0x13;
        mem[0x104] = 0x00; mem[0x105] = 0xEE;
    
        cpu.run();
    
        println!("cpu.registers[0] = {}, 0x{:04X}", 
                    cpu.registers[0],
                    cpu.registers[0]);
        assert_eq!(cpu.registers[0], 0x00FF);

    }
    
    #[test]
    fn test_add_xy() { 

        println!("test_add_xy()");
        
        let mut cpu = CPU {
            registers: [0; 16],
            memory: [0; 4096],
            position_in_memory: 0,
            stack: [0; 16],
            stack_pointer: 0,
        };
    
        cpu.registers[0] = 5;
        cpu.registers[1] = 5;
    
        let mem = &mut cpu.memory;

        mem[0x000] = 0x21; mem[0x001] = 0x00;
        mem[0x100] = 0x80; mem[0x101] = 0x14;
        mem[0x104] = 0x00; mem[0x105] = 0xEE;
    
        cpu.run();
    
        println!("cpu.registers[0] = {}, 0x{:04X}", 
                    cpu.registers[0],
                    cpu.registers[0]);
        println!(" "); 
        assert_eq!(cpu.registers[0], 10);

    }
    
    #[test]
    fn test_sub_xy() {

        println!("test_sub_xy()");
        
        let mut cpu = CPU {
            registers: [0; 16],
            memory: [0; 4096],
            position_in_memory: 0,
            stack: [0; 16],
            stack_pointer: 0,
        };
    
        cpu.registers[0] = 15;
        cpu.registers[1] = 5;
    
        let mem = &mut cpu.memory;

        mem[0x000] = 0x21; mem[0x001] = 0x00;
        mem[0x100] = 0x80; mem[0x101] = 0x15;
        mem[0x104] = 0x00; mem[0x105] = 0xEE;
    
        cpu.run();
    
        println!("cpu.registers[0] = {}, 0x{:04X}", 
                    cpu.registers[0],
                    cpu.registers[0]);
        println!(" "); 
        assert_eq!(cpu.registers[0], 10);
    
    }
    
    #[test]
    fn test_shr_xy() { 

        println!("test_shr_xy()");
        let mut cpu = CPU {
            registers: [0; 16],
            memory: [0; 4096],
            position_in_memory: 0,
            stack: [0; 16],
            stack_pointer: 0,
        };
    
        cpu.registers[0] = 0xF0;
        cpu.registers[1] = 4;
    
        let mem = &mut cpu.memory;

        mem[0x000] = 0x21; mem[0x001] = 0x00;
        mem[0x100] = 0x80; mem[0x101] = 0x16;
        mem[0x104] = 0x00; mem[0x105] = 0xEE;
    
        cpu.run();
    
        println!("cpu.registers[0] = {:32b}, 0x{:04X}", 
                    cpu.registers[0],
                    cpu.registers[0]);
        println!(" "); 
        assert_eq!(cpu.registers[0], 0x000F);
    }
    
    #[test]
    fn test_subn_xy() {

        println!("test_subn_xy()");
        
        let mut cpu = CPU {
            registers: [0; 16],
            memory: [0; 4096],
            position_in_memory: 0,
            stack: [0; 16],
            stack_pointer: 0,
        };
    
        cpu.registers[0] = 5;
        cpu.registers[1] = 15;
    
        let mem = &mut cpu.memory;

        mem[0x000] = 0x21; mem[0x001] = 0x00;
        mem[0x100] = 0x80; mem[0x101] = 0x17;
        mem[0x104] = 0x00; mem[0x105] = 0xEE;
    
        cpu.run();
    
        println!("cpu.registers[0] = {}, 0x{:04X}", 
                    cpu.registers[0],
                    cpu.registers[0]);
        println!(" "); 
        assert_eq!(cpu.registers[0], 10);

    }
    
    #[test]
    fn test_shl_xy() {
        println!("test_shl_xy()");
        let mut cpu = CPU {
            registers: [0; 16],
            memory: [0; 4096],
            position_in_memory: 0,
            stack: [0; 16],
            stack_pointer: 0,
        };
    
        cpu.registers[0] = 0x0F;
        cpu.registers[1] = 4;
    
        let mem = &mut cpu.memory;

        mem[0x000] = 0x21; mem[0x001] = 0x00;
        mem[0x100] = 0x80; mem[0x101] = 0x1E;
        mem[0x104] = 0x00; mem[0x105] = 0xEE;
    
        cpu.run();
    
        println!("cpu.registers[0] = {:32b}, 0x{:04X}", 
                    cpu.registers[0],
                    cpu.registers[0]);
        println!(" "); 
        assert_eq!(cpu.registers[0], 0x00F0);
    }

}