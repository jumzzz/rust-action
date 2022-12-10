# Emulating Chip-8 

## Purpose
- Extend `cpu_ria_03` to accomodate additional logical and arithmetic opcodes

## Reference
Use the following Chip-8 Specifications here in [Chip-8 Design Specification](http://www.cs.columbia.edu/~sedwards/classes/2016/4840-spring/designs/Chip8.pdf)


### DONE: Implement Additional Op Codes
- **OR Vx, Vy** - `8xy1`  (Done)
- **AND Vx, Vy** - `8xy2` (Done)
- **XOR Vx, Vy** - `8xy3` (Done)
- **ADD Vx, Vy** - `8xy4` (Done)
- **SUB Vx, Vy** - `8xy5` (Done)
- **SHR Vx, Vy** - `8xy6` (Done)
- **SUBN Vx, Vy** - `8xy7` (Done)
- **SHL Vx, Vy** - `8xyE` (Done)

**Note:**
- This is a quick and dirty implementation.