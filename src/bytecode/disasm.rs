mod format;

use super::Bytecode;
use format::format_instr;

pub fn disassemble(bc: &Bytecode) -> String {
    let mut out = String::new();
    out.push_str(&format!("bytecode: {}\n", bc.name));
    out.push_str(&format!("entry: {}\n", bc.entry));
    out.push_str(&format!("functions: {}\n", bc.functions.len()));
    for (i, func) in bc.functions.iter().enumerate() {
        out.push_str(&format!(
            "\nfn #{} {} (params: {}, locals: {})\n",
            i, func.name, func.param_count, func.locals
        ));
        for (ip, instr) in func.code.iter().enumerate() {
            out.push_str(&format!("{:04}  {}\n", ip, format_instr(instr)));
        }
    }
    out
}
