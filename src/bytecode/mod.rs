mod format;
mod instr;
mod disasm;

pub use format::{read_bytecode, write_bytecode};
pub use instr::{Bytecode, Function, Instr, OpCode};
pub use disasm::disassemble;
