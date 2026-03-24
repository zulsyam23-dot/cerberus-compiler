mod ast;
mod bytecode;
mod codegen;
mod error;
mod lexer;
mod parser;
mod typecheck;
mod vm;

use std::env;
use std::fs;
use std::path::Path;
use std::collections::HashSet;

use anyhow::Context;
use codegen::Codegen;
use parser::Parser;

fn main() -> anyhow::Result<()> {
    let mut args = env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() {
        eprintln!("usage: cerberus-compiler <input> [output]");
        eprintln!("   or: cerberus-compiler run <bytecode>");
        eprintln!("   or: cerberus-compiler dump <bytecode>");
        eprintln!("   or: cerberus-compiler [--run] <input> [output]");
        eprintln!("   or: cerberus-compiler [--run] [--no-typecheck|--bootstrap] <input> [output]");
        eprintln!("   or: cerberus-compiler [--run] [--no-typecheck|--bootstrap] <input> [output] -- <args>");
        std::process::exit(1);
    }

    if args[0] == "run" {
        if args.len() < 2 {
            eprintln!("usage: cerberus-compiler run <bytecode>");
            std::process::exit(1);
        }
        let bc_path = &args[1];
        let program_args = if args.len() > 2 {
            args[2..].to_vec()
        } else {
            Vec::new()
        };
        let bc = match bytecode::read_bytecode(bc_path) {
            Ok(bc) => bc,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        };
        let mut vm = vm::Vm::new(&bc, program_args);
        if let Err(e) = vm.run() {
            eprintln!("{}", e);
            std::process::exit(1);
        }
        return Ok(());
    }
    if args[0] == "dump" {
        if args.len() < 2 {
            eprintln!("usage: cerberus-compiler dump <bytecode>");
            std::process::exit(1);
        }
        let bc_path = &args[1];
        let bc = match bytecode::read_bytecode(bc_path) {
            Ok(bc) => bc,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        };
        print!("{}", bytecode::disassemble(&bc));
        return Ok(());
    }

    let mut run_after = false;
    let mut skip_typecheck = false;
    let mut program_args = Vec::new();
    if let Some(pos) = args.iter().position(|a| a == "--") {
        program_args = args[(pos + 1)..].to_vec();
        args.truncate(pos);
    }
    args.retain(|a| {
        if a == "--run" {
            run_after = true;
            false
        } else if a == "--no-typecheck" || a == "--bootstrap" {
            skip_typecheck = true;
            false
        } else {
            true
        }
    });

    let input_path = args.remove(0);
    let output_path = args.pop().unwrap_or_else(|| "out.cerb".to_string());

    let source = fs::read_to_string(&input_path)
        .with_context(|| format!("failed to read {}", input_path))?;

    let mut program = match Parser::new(&source).and_then(|mut p| p.parse_program()) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e.render(&source));
            std::process::exit(1);
        }
    };

    if !program.uses.is_empty() {
        let base_dir = Path::new(&input_path)
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| Path::new(".").to_path_buf());
        let mut merged_decls = Vec::new();
        let mut visited = HashSet::new();
        for module in &program.uses {
            let decls = load_module_decls(&base_dir, module, &mut visited)?;
            merged_decls.extend(decls);
        }
        merged_decls.extend(program.block.declarations);
        program.block.declarations = merged_decls;
    }

    if !skip_typecheck {
        let mut checker = typecheck::TypeChecker::new();
        if let Err(e) = checker.check_program(&program) {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }

    let module_name = Path::new(&input_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("module");

    let mut codegen = Codegen::new(&program.name, module_name);
    let bytecode = match codegen.emit_program(&program) {
        Ok(bc) => bc,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = bytecode::write_bytecode(&output_path, &bytecode) {
        eprintln!("{}", e);
        std::process::exit(1);
    }

    println!("wrote {}", output_path);

    if run_after {
        let mut vm = vm::Vm::new(&bytecode, program_args);
        if let Err(e) = vm.run() {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
    Ok(())
}

fn resolve_module_path(base: &Path, name: &str) -> Option<std::path::PathBuf> {
    let candidates = [
        base.join(format!("{}.cer", name)),
        base.join(format!("{}.cerberus", name)),
    ];
    for p in candidates {
        if p.exists() {
            return Some(p);
        }
    }
    if let Some(stdlib) = stdlib_dir() {
        let candidates = [
            stdlib.join(format!("{}.cer", name)),
            stdlib.join(format!("{}.cerberus", name)),
            stdlib.join("lexer").join(format!("{}.cer", name)),
            stdlib.join("parser").join(format!("{}.cer", name)),
            stdlib.join("codegen").join(format!("{}.cer", name)),
            stdlib.join("typecheck").join(format!("{}.cer", name)),
        ];
        for p in candidates {
            if p.exists() {
                return Some(p);
            }
        }
    }
    None
}

fn load_module_decls(
    base_dir: &Path,
    name: &str,
    visited: &mut HashSet<String>,
) -> anyhow::Result<Vec<crate::ast::Decl>> {
    if visited.contains(name) {
        return Ok(Vec::new());
    }
    visited.insert(name.to_string());
    let module_path = resolve_module_path(base_dir, name)
        .ok_or_else(|| anyhow::anyhow!("module '{}' not found", name))?;
    let src = fs::read_to_string(&module_path)
        .with_context(|| format!("failed to read {}", module_path.display()))?;
    let mod_program = Parser::new(&src)
        .and_then(|mut p| p.parse_program())
        .map_err(|e| anyhow::anyhow!("{}", e.render(&src)))?;
    if mod_program
        .block
        .statements
        .iter()
        .any(|s| !matches!(s, crate::ast::Stmt::Empty))
    {
        return Err(anyhow::anyhow!(
            "module '{}' must not contain statements",
            name
        ));
    }
    let mut merged = Vec::new();
    if !mod_program.uses.is_empty() {
        let module_base = module_path
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| base_dir.to_path_buf());
        for u in &mod_program.uses {
            let decls = load_module_decls(&module_base, u, visited)?;
            merged.extend(decls);
        }
    }
    merged.extend(mod_program.block.declarations);
    Ok(merged)
}

fn stdlib_dir() -> Option<std::path::PathBuf> {
    if let Ok(path) = std::env::var("CERBERUS_STDLIB") {
        let p = std::path::PathBuf::from(path);
        if p.exists() {
            return Some(p);
        }
    }
    if let Ok(cwd) = std::env::current_dir() {
        let p = cwd.join("stdlib");
        if p.exists() {
            return Some(p);
        }
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let p = dir.join("stdlib");
            if p.exists() {
                return Some(p);
            }
        }
    }
    None
}
