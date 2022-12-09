# Emulating Chip-8 

## Purpose
- Extend `cpu_ria_03` to accomodate additional logical and arithmetic opcodes

## Reference
Use the following Chip-8 Specifications here in [Chip-8 Design Specification](http://www.cs.columbia.edu/~sedwards/classes/2016/4840-spring/designs/Chip8.pdf)


### TODO: Implement Additional Op Codes
- **LD Vx, Vy** - `8xy0` 
- **OR Vx, Vy** - `8xy1` 
- **AND Vx, Vy** - `8xy2` 
- **XOR Vx, Vy** - `8xy3` 
- **ADD Vx, Vy** - `8xy4` 
- **SUB Vx, Vy** - `8xy5` 
- **SHR Vx, Vy** - `8xy6` 
- **SUBN Vx, Vy** - `8xy7` 
- **SHL Vx, Vy** - `8xyE`

### TODO: Make it read binary files
- Use command-line arguments to read a binary test files

### TODO: Refactor Code to Make it more readable
- Use enums/struct to make Op Codes more readable
- Decouple Source Files