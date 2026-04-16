mod disasm;
mod format;
mod instr;

pub use disasm::disassemble;
pub use format::{read_bytecode, write_bytecode};
pub use instr::{Bytecode, Function, Instr, OpCode};
