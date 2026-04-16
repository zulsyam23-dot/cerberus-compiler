mod ast;
mod bytecode;
mod codegen;
mod error;
mod lexer;
mod parser;
mod typecheck;
mod vm;

use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

use anyhow::Context;
use codegen::Codegen;
use parser::Parser;

fn main() -> anyhow::Result<()> {
    let mut args = env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() {
        eprintln!("usage: cerberus-compiler <input> [output]");
        eprintln!(
            "   or: cerberus-compiler run [--limit-steps N] [--limit-stack N] [--limit-call N] <bytecode> [-- <args>]"
        );
        eprintln!("   or: cerberus-compiler dump <bytecode>");
        eprintln!(
            "   or: cerberus-compiler selfhost [--run] [--no-typecheck|--bootstrap] [--no-runtime-cache] [--limit-steps N] [--limit-stack N] [--limit-call N] <input> [output] [-- <args>]"
        );
        eprintln!(
            "   or: cerberus-compiler [--native] [--run] [--limit-steps N] [--limit-stack N] [--limit-call N] <input> [output]"
        );
        eprintln!(
            "   or: cerberus-compiler [--native] [--run] [--no-typecheck|--bootstrap] [--limit-steps N] [--limit-stack N] [--limit-call N] <input> [output]"
        );
        eprintln!(
            "   or: cerberus-compiler [--no-runtime-cache] [--limit-steps N] [--limit-stack N] [--limit-call N] <vm-script.cer> -- <args>"
        );
        eprintln!("   or: cerberus-compiler <vm-script.cer> -- <args>");
        eprintln!(
            "   or: cerberus-compiler [--native] [--run] [--no-typecheck|--bootstrap] [--limit-steps N] [--limit-stack N] [--limit-call N] <input> [output] -- <args>"
        );
        eprintln!("note: default compile path is selfhosted; use --native for Rust pipeline");
        std::process::exit(1);
    }

    if args[0] == "run" {
        if args.len() < 2 {
            eprintln!(
                "usage: cerberus-compiler run [--limit-steps N] [--limit-stack N] [--limit-call N] <bytecode> [-- <args>]"
            );
            std::process::exit(1);
        }
        let mut run_args = args[1..].to_vec();
        let mut program_args = Vec::new();
        if let Some(pos) = run_args.iter().position(|a| a == "--") {
            program_args = run_args[(pos + 1)..].to_vec();
            run_args.truncate(pos);
        }
        let mut limits = vm::VmLimits::default();
        let mut bc_path = String::new();
        let mut i = 0;
        while i < run_args.len() {
            let cur = &run_args[i];
            if !bc_path.is_empty() {
                program_args.push(cur.clone());
                i += 1;
                continue;
            }
            if cur == "--limit-steps" {
                if i + 1 >= run_args.len() {
                    eprintln!("error: --limit-steps requires a value");
                    std::process::exit(1);
                }
                apply_limit_flag(&mut limits, "--limit-steps", &run_args[i + 1]);
                i += 2;
                continue;
            }
            if cur == "--limit-stack" {
                if i + 1 >= run_args.len() {
                    eprintln!("error: --limit-stack requires a value");
                    std::process::exit(1);
                }
                apply_limit_flag(&mut limits, "--limit-stack", &run_args[i + 1]);
                i += 2;
                continue;
            }
            if cur == "--limit-call" {
                if i + 1 >= run_args.len() {
                    eprintln!("error: --limit-call requires a value");
                    std::process::exit(1);
                }
                apply_limit_flag(&mut limits, "--limit-call", &run_args[i + 1]);
                i += 2;
                continue;
            }
            if cur.starts_with("--") {
                eprintln!("error: unknown run option '{}'", cur);
                eprintln!(
                    "usage: cerberus-compiler run [--limit-steps N] [--limit-stack N] [--limit-call N] <bytecode> [-- <args>]"
                );
                std::process::exit(1);
            }
            bc_path = cur.clone();
            i += 1;
        }
        if bc_path.is_empty() {
            eprintln!("error: missing bytecode path");
            eprintln!(
                "usage: cerberus-compiler run [--limit-steps N] [--limit-stack N] [--limit-call N] <bytecode> [-- <args>]"
            );
            std::process::exit(1);
        }
        let bc = match bytecode::read_bytecode(&bc_path) {
            Ok(bc) => bc,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        };
        if let Err(e) = run_compiled_bytecode(&bc, program_args, Some(limits)) {
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

    if args[0] == "selfhost" {
        if args.len() < 2 {
            eprintln!(
                "usage: cerberus-compiler selfhost [--run] [--no-typecheck|--bootstrap] [--no-runtime-cache] [--limit-steps N] [--limit-stack N] [--limit-call N] <input> [output] [-- <args>]"
            );
            std::process::exit(1);
        }
        let mut sh_args = args[1..].to_vec();
        let mut program_args = Vec::new();
        if let Some(pos) = sh_args.iter().position(|a| a == "--") {
            program_args = sh_args[(pos + 1)..].to_vec();
            sh_args.truncate(pos);
        }

        let mut run_after = false;
        let mut skip_typecheck = false;
        let mut compiler_cache_enabled = true;
        let mut limits = vm::VmLimits::default();
        let mut limits_set = false;
        let mut positional = Vec::new();

        let mut i = 0;
        while i < sh_args.len() {
            let cur = &sh_args[i];
            if cur == "--run" {
                run_after = true;
                i += 1;
                continue;
            }
            if cur == "--no-typecheck" || cur == "--bootstrap" {
                skip_typecheck = true;
                i += 1;
                continue;
            }
            if cur == "--no-runtime-cache" {
                compiler_cache_enabled = false;
                i += 1;
                continue;
            }
            if cur == "--limit-steps" || cur == "--limit-stack" || cur == "--limit-call" {
                if i + 1 >= sh_args.len() {
                    eprintln!("error: {} requires a value", cur);
                    std::process::exit(1);
                }
                apply_limit_flag(&mut limits, cur, &sh_args[i + 1]);
                limits_set = true;
                i += 2;
                continue;
            }
            if cur.starts_with("--") {
                eprintln!("error: unknown selfhost option '{}'", cur);
                eprintln!(
                    "usage: cerberus-compiler selfhost [--run] [--no-typecheck|--bootstrap] [--no-runtime-cache] [--limit-steps N] [--limit-stack N] [--limit-call N] <input> [output] [-- <args>]"
                );
                std::process::exit(1);
            }
            positional.push(cur.clone());
            i += 1;
        }

        if positional.is_empty() {
            eprintln!("error: missing input path");
            eprintln!(
                "usage: cerberus-compiler selfhost [--run] [--no-typecheck|--bootstrap] [--no-runtime-cache] [--limit-steps N] [--limit-stack N] [--limit-call N] <input> [output] [-- <args>]"
            );
            std::process::exit(1);
        }
        if positional.len() > 2 {
            eprintln!(
                "error: too many positional arguments (expected: <input> [output], got {})",
                positional.len()
            );
            std::process::exit(1);
        }

        let input_path = positional.remove(0);
        let output_path = positional.pop().unwrap_or_else(|| "out.cerb".to_string());
        let exec_limits_opt = if limits_set { Some(limits) } else { None };

        if !run_after && !program_args.is_empty() {
            eprintln!("note: target program args are ignored without --run");
        }

        let stdlib_compiler_path = resolve_compiler_entry_path()
            .ok_or_else(|| anyhow::anyhow!("could not locate stdlib/compiler.cer"))?;
        let compiler_bc =
            load_or_compile_compiler_bytecode(&stdlib_compiler_path, compiler_cache_enabled)?;

        let mut compiler_args = Vec::new();
        if skip_typecheck {
            compiler_args.push("--no-typecheck".to_string());
        }
        compiler_args.push(input_path.clone());
        compiler_args.push(output_path.clone());
        run_compiled_bytecode(&compiler_bc, compiler_args, exec_limits_opt)?;

        if run_after {
            let compiled_bc =
                bytecode::read_bytecode(&output_path).map_err(|e| anyhow::anyhow!("{}", e))?;
            run_compiled_bytecode(&compiled_bc, program_args, exec_limits_opt)?;
        }
        return Ok(());
    }

    let mut run_after = false;
    let mut skip_typecheck = false;
    let mut runtime_cache_enabled = true;
    let mut force_native = false;
    let mut exec_limits = vm::VmLimits::default();
    let mut exec_limits_set = false;
    let mut program_args = Vec::new();
    if let Some(pos) = args.iter().position(|a| a == "--") {
        program_args = args[(pos + 1)..].to_vec();
        args.truncate(pos);
    }
    let mut filtered_args = Vec::new();
    let mut j = 0;
    while j < args.len() {
        let a = &args[j];
        if a == "--run" {
            run_after = true;
            j += 1;
            continue;
        }
        if a == "--no-runtime-cache" {
            runtime_cache_enabled = false;
            j += 1;
            continue;
        }
        if a == "--native" {
            force_native = true;
            j += 1;
            continue;
        }
        if a == "--selfhost" {
            force_native = false;
            j += 1;
            continue;
        }
        if a == "--no-typecheck" || a == "--bootstrap" {
            skip_typecheck = true;
            j += 1;
            continue;
        }
        if a == "--limit-steps" || a == "--limit-stack" || a == "--limit-call" {
            if j + 1 >= args.len() {
                eprintln!("error: {} requires a value", a);
                std::process::exit(1);
            }
            apply_limit_flag(&mut exec_limits, a, &args[j + 1]);
            exec_limits_set = true;
            j += 2;
            continue;
        }
        filtered_args.push(a.clone());
        j += 1;
    }
    args = filtered_args;

    if args.is_empty() {
        eprintln!("error: missing input path");
        eprintln!("usage: cerberus-compiler <input> [output]");
        std::process::exit(1);
    }
    if args.len() > 2 {
        eprintln!(
            "error: too many positional arguments (expected: <input> [output], got {})",
            args.len()
        );
        std::process::exit(1);
    }

    let exec_limits_opt = if exec_limits_set {
        Some(exec_limits)
    } else {
        None
    };

    let input_path = args.remove(0);
    let output_given = !args.is_empty();
    let output_path = args.pop().unwrap_or_else(|| "out.cerb".to_string());

    let source = fs::read_to_string(&input_path)
        .with_context(|| format!("failed to read {}", input_path))?;

    let vm_script_mode = is_vm_text_script(&source);
    if !run_after && !vm_script_mode && exec_limits_set && force_native {
        eprintln!("note: execution limits are ignored without --run");
    }

    if vm_script_mode {
        if output_given {
            eprintln!(
                "note: output file is ignored in @cerberus_vm script mode; running script directly"
            );
        }
        if let Err(e) = run_vm_text_script(
            &input_path,
            program_args,
            runtime_cache_enabled,
            exec_limits_opt,
        ) {
            eprintln!("{e}");
            std::process::exit(1);
        }
        return Ok(());
    }

    if !force_native {
        let stdlib_compiler_path = resolve_compiler_entry_path()
            .ok_or_else(|| anyhow::anyhow!("could not locate stdlib/compiler.cer"))?;
        let compiler_bc =
            load_or_compile_compiler_bytecode(&stdlib_compiler_path, runtime_cache_enabled)?;

        let mut compiler_args = Vec::new();
        if skip_typecheck {
            compiler_args.push("--no-typecheck".to_string());
        }
        compiler_args.push(input_path.clone());
        compiler_args.push(output_path.clone());
        run_compiled_bytecode(&compiler_bc, compiler_args, exec_limits_opt)?;

        if run_after {
            let compiled_bc =
                bytecode::read_bytecode(&output_path).map_err(|e| anyhow::anyhow!("{}", e))?;
            run_compiled_bytecode(&compiled_bc, program_args, exec_limits_opt)?;
        }
        return Ok(());
    }

    let bytecode = match compile_source_to_bytecode(&source, Path::new(&input_path), skip_typecheck)
    {
        Ok(bc) => bc,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };

    if let Err(e) = bytecode::write_bytecode(&output_path, &bytecode) {
        eprintln!("{}", e);
        std::process::exit(1);
    }

    println!("wrote {}", output_path);

    if run_after {
        if let Err(e) = run_compiled_bytecode(&bytecode, program_args, exec_limits_opt) {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
    Ok(())
}

fn parse_positive_u64(flag: &str, value: &str) -> u64 {
    match value.parse::<u64>() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("error: {} expects an integer, got '{}'", flag, value);
            std::process::exit(1);
        }
    }
}

fn parse_positive_usize(flag: &str, value: &str) -> usize {
    let v = parse_positive_u64(flag, value);
    if v == 0 {
        eprintln!("error: {} must be > 0", flag);
        std::process::exit(1);
    }
    if v > usize::MAX as u64 {
        eprintln!("error: {} is too large", flag);
        std::process::exit(1);
    }
    v as usize
}

fn apply_limit_flag(limits: &mut vm::VmLimits, flag: &str, value: &str) {
    if flag == "--limit-steps" {
        let v = parse_positive_u64(flag, value);
        if v == 0 {
            eprintln!("error: {} must be > 0", flag);
            std::process::exit(1);
        }
        limits.max_steps = v;
        return;
    }
    if flag == "--limit-stack" {
        limits.max_stack_size = parse_positive_usize(flag, value);
        return;
    }
    if flag == "--limit-call" {
        limits.max_call_depth = parse_positive_usize(flag, value);
        return;
    }
    eprintln!("error: unknown limit flag '{}'", flag);
    std::process::exit(1);
}

fn compile_source_to_bytecode(
    source: &str,
    input_path: &Path,
    skip_typecheck: bool,
) -> anyhow::Result<bytecode::Bytecode> {
    let mut program = Parser::new(source)
        .and_then(|mut p| p.parse_program())
        .map_err(|e| anyhow::anyhow!("{}", e.render(source)))?;

    if !program.uses.is_empty() {
        let base_dir = input_path
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
        checker
            .check_program(&program)
            .map_err(|e| anyhow::anyhow!("{}", e))?;
    }

    let module_name = input_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("module");

    let mut codegen = Codegen::new(&program.name, module_name);
    codegen
        .emit_program(&program)
        .map_err(|e| anyhow::anyhow!("{}", e))
}

fn run_compiled_bytecode(
    bc: &bytecode::Bytecode,
    args: Vec<String>,
    limits: Option<vm::VmLimits>,
) -> anyhow::Result<()> {
    let mut vm = if let Some(custom_limits) = limits {
        let config = vm::VmConfig {
            limits: custom_limits,
            ..vm::VmConfig::default()
        };
        vm::Vm::with_config(bc, args, config).map_err(|e| anyhow::anyhow!("{}", e))?
    } else {
        vm::Vm::new(bc, args).map_err(|e| anyhow::anyhow!("{}", e))?
    };
    vm.run().map_err(|e| anyhow::anyhow!("{}", e))
}

fn run_vm_text_script(
    script_path: &str,
    extra_args: Vec<String>,
    runtime_cache_enabled: bool,
    limits: Option<vm::VmLimits>,
) -> anyhow::Result<()> {
    let runtime_path = resolve_runtime_entry_path()
        .ok_or_else(|| anyhow::anyhow!("could not locate stdlib/runtime/runtime.cer"))?;
    let runtime_bc = load_or_compile_runtime_bytecode(&runtime_path, runtime_cache_enabled)?;

    let mut vm_args = Vec::with_capacity(1 + extra_args.len());
    vm_args.push(script_path.to_string());
    vm_args.extend(extra_args);
    run_compiled_bytecode(&runtime_bc, vm_args, limits)
}

fn load_or_compile_runtime_bytecode(
    runtime_path: &Path,
    use_runtime_cache: bool,
) -> anyhow::Result<bytecode::Bytecode> {
    let cache_path = runtime_cache_path(runtime_path);
    let cache_path_str = cache_path.to_string_lossy();
    if use_runtime_cache && is_runtime_cache_fresh(runtime_path, &cache_path) {
        if let Ok(cached) = bytecode::read_bytecode(&cache_path_str) {
            return Ok(cached);
        }
    }

    let runtime_src = fs::read_to_string(runtime_path)
        .with_context(|| format!("failed to read {}", runtime_path.display()))?;
    let runtime_bc = compile_source_to_bytecode(&runtime_src, runtime_path, false)?;
    if use_runtime_cache {
        let _ = bytecode::write_bytecode(&cache_path_str, &runtime_bc);
    }
    Ok(runtime_bc)
}

fn load_or_compile_compiler_bytecode(
    compiler_path: &Path,
    use_compiler_cache: bool,
) -> anyhow::Result<bytecode::Bytecode> {
    let cache_path = compiler_cache_path(compiler_path);
    let cache_path_str = cache_path.to_string_lossy();
    let source_root = compiler_path.parent().unwrap_or_else(|| Path::new("."));
    if use_compiler_cache && is_cache_fresh(&cache_path, source_root) {
        if let Ok(cached) = bytecode::read_bytecode(&cache_path_str) {
            return Ok(cached);
        }
    }

    let compiler_src = fs::read_to_string(compiler_path)
        .with_context(|| format!("failed to read {}", compiler_path.display()))?;
    let compiler_bc = compile_source_to_bytecode(&compiler_src, compiler_path, false)?;
    if use_compiler_cache {
        let _ = bytecode::write_bytecode(&cache_path_str, &compiler_bc);
    }
    Ok(compiler_bc)
}

fn runtime_cache_path(runtime_path: &Path) -> std::path::PathBuf {
    runtime_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(".runtime_cache.cerb")
}

fn compiler_cache_path(compiler_path: &Path) -> std::path::PathBuf {
    compiler_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(".compiler_cache.cerb")
}

fn is_runtime_cache_fresh(runtime_path: &Path, cache_path: &Path) -> bool {
    let runtime_root = match runtime_path.parent() {
        Some(p) => p,
        None => return false,
    };
    is_cache_fresh(cache_path, runtime_root)
}

fn is_cache_fresh(cache_path: &Path, source_root: &Path) -> bool {
    let cache_modified = match fs::metadata(cache_path).and_then(|m| m.modified()) {
        Ok(t) => t,
        Err(_) => return false,
    };
    cache_modified >= newest_source_mtime(source_root)
}

fn newest_source_mtime(root: &Path) -> SystemTime {
    let mut newest = SystemTime::UNIX_EPOCH;
    collect_newest_source_mtime(root, &mut newest);
    newest
}

fn collect_newest_source_mtime(dir: &Path, newest: &mut SystemTime) {
    let entries = match fs::read_dir(dir) {
        Ok(v) => v,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let md = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };
        if md.is_dir() {
            collect_newest_source_mtime(&path, newest);
            continue;
        }
        if !md.is_file() {
            continue;
        }
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        if ext != "cer" && ext != "cerberus" {
            continue;
        }
        if let Ok(modified) = md.modified() {
            if modified > *newest {
                *newest = modified;
            }
        }
    }
}

fn resolve_runtime_entry_path() -> Option<std::path::PathBuf> {
    resolve_stdlib_entry_path(&["runtime", "runtime.cer"])
}

fn resolve_compiler_entry_path() -> Option<std::path::PathBuf> {
    resolve_stdlib_entry_path(&["compiler.cer"])
}

fn resolve_stdlib_entry_path(relative: &[&str]) -> Option<std::path::PathBuf> {
    if let Some(stdlib) = stdlib_dir() {
        let mut p = stdlib;
        for segment in relative {
            p = p.join(segment);
        }
        if p.exists() {
            return Some(p);
        }
    }
    if let Ok(cwd) = std::env::current_dir() {
        let mut p = cwd.join("stdlib");
        for segment in relative {
            p = p.join(segment);
        }
        if p.exists() {
            return Some(p);
        }
    }
    None
}

fn is_vm_text_script(source: &str) -> bool {
    fn is_vm_token(token: &str) -> bool {
        matches!(
            token,
            "@cerberus_vm"
                | "@entry"
                | "@limit_steps"
                | "@limit_stack"
                | "@limit_call"
                | "label"
                | "halt"
                | "nop"
                | "locals"
                | "const_int"
                | "const_bool"
                | "const_str"
                | "load"
                | "store"
                | "add"
                | "sub"
                | "mul"
                | "div"
                | "eq"
                | "ne"
                | "lt"
                | "le"
                | "gt"
                | "ge"
                | "and"
                | "or"
                | "not"
                | "neg"
                | "jump"
                | "jump_if_false"
                | "call"
                | "ret"
                | "ret_val"
                | "println"
                | "readfile"
                | "writefile"
                | "vec_new"
                | "vec_len"
                | "vec_get"
                | "vec_set"
                | "vec_push"
                | "vec_remove"
                | "vec_last"
                | "vec_pop"
                | "map_new"
                | "map_len"
                | "map_set"
                | "map_get"
                | "map_has"
                | "map_remove"
                | "limit_steps"
                | "limit_stack"
                | "limit_call"
        )
    }

    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let no_comment = trimmed.split('#').next().unwrap_or("").trim();
        if no_comment.is_empty() {
            continue;
        }
        let token = no_comment
            .split_whitespace()
            .next()
            .unwrap_or("")
            .trim()
            .trim_end_matches(';');
        return is_vm_token(&token.to_ascii_lowercase());
    }
    false
}

fn resolve_module_path(base: &Path, name: &str) -> Option<std::path::PathBuf> {
    let mut candidates = vec![
        base.join(format!("{}.cer", name)),
        base.join(format!("{}.cerberus", name)),
        base.join("runtime").join(format!("{}.cer", name)),
        base.join("runtime").join(format!("{}.cerberus", name)),
        base.join("vm").join(format!("{}.cer", name)),
        base.join("vm").join(format!("{}.cerberus", name)),
        base.join("vm")
            .join("runtime")
            .join(format!("{}.cer", name)),
        base.join("vm")
            .join("runtime")
            .join(format!("{}.cerberus", name)),
        base.join("lexer").join(format!("{}.cer", name)),
        base.join("parser").join(format!("{}.cer", name)),
        base.join("codegen").join(format!("{}.cer", name)),
        base.join("typecheck").join(format!("{}.cer", name)),
        base.join("bytecode").join(format!("{}.cer", name)),
        base.join("bytecode")
            .join("format")
            .join(format!("{}.cer", name)),
        base.join("bytecode")
            .join("disasm")
            .join(format!("{}.cer", name)),
        base.join("lexer")
            .join("scanner")
            .join(format!("{}.cer", name)),
        base.join("codegen")
            .join("expr")
            .join(format!("{}.cer", name)),
        base.join("codegen")
            .join("stmt")
            .join(format!("{}.cer", name)),
        base.join("codegen")
            .join("expr")
            .join("builtins")
            .join(format!("{}.cer", name)),
        base.join("codegen")
            .join("stmt")
            .join("builtins")
            .join(format!("{}.cer", name)),
        base.join("typecheck")
            .join("builtins")
            .join(format!("{}.cer", name)),
    ];
    if let Some(parent) = base.parent() {
        candidates.extend([
            parent.join(format!("{}.cer", name)),
            parent.join(format!("{}.cerberus", name)),
            parent.join("runtime").join(format!("{}.cer", name)),
            parent.join("runtime").join(format!("{}.cerberus", name)),
            parent.join("vm").join(format!("{}.cer", name)),
            parent.join("vm").join(format!("{}.cerberus", name)),
            parent
                .join("vm")
                .join("runtime")
                .join(format!("{}.cer", name)),
            parent
                .join("vm")
                .join("runtime")
                .join(format!("{}.cerberus", name)),
            parent.join("lexer").join(format!("{}.cer", name)),
            parent.join("parser").join(format!("{}.cer", name)),
            parent.join("codegen").join(format!("{}.cer", name)),
            parent.join("typecheck").join(format!("{}.cer", name)),
            parent.join("bytecode").join(format!("{}.cer", name)),
            parent
                .join("bytecode")
                .join("format")
                .join(format!("{}.cer", name)),
            parent
                .join("bytecode")
                .join("disasm")
                .join(format!("{}.cer", name)),
            parent
                .join("lexer")
                .join("scanner")
                .join(format!("{}.cer", name)),
            parent
                .join("codegen")
                .join("expr")
                .join(format!("{}.cer", name)),
            parent
                .join("codegen")
                .join("stmt")
                .join(format!("{}.cer", name)),
            parent
                .join("codegen")
                .join("expr")
                .join("builtins")
                .join(format!("{}.cer", name)),
            parent
                .join("codegen")
                .join("stmt")
                .join("builtins")
                .join(format!("{}.cer", name)),
            parent
                .join("typecheck")
                .join("builtins")
                .join(format!("{}.cer", name)),
        ]);
    }
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
            stdlib
                .join("lexer")
                .join("scanner")
                .join(format!("{}.cer", name)),
            stdlib.join("parser").join(format!("{}.cer", name)),
            stdlib.join("codegen").join(format!("{}.cer", name)),
            stdlib
                .join("codegen")
                .join("expr")
                .join(format!("{}.cer", name)),
            stdlib
                .join("codegen")
                .join("stmt")
                .join(format!("{}.cer", name)),
            stdlib
                .join("codegen")
                .join("expr")
                .join("builtins")
                .join(format!("{}.cer", name)),
            stdlib
                .join("codegen")
                .join("stmt")
                .join("builtins")
                .join(format!("{}.cer", name)),
            stdlib.join("typecheck").join(format!("{}.cer", name)),
            stdlib
                .join("typecheck")
                .join("builtins")
                .join(format!("{}.cer", name)),
            stdlib.join("bytecode").join(format!("{}.cer", name)),
            stdlib
                .join("bytecode")
                .join("format")
                .join(format!("{}.cer", name)),
            stdlib
                .join("bytecode")
                .join("disasm")
                .join(format!("{}.cer", name)),
            stdlib.join("runtime").join(format!("{}.cer", name)),
            stdlib.join("runtime").join(format!("{}.cerberus", name)),
            stdlib.join("vm").join(format!("{}.cer", name)),
            stdlib.join("vm").join(format!("{}.cerberus", name)),
            stdlib
                .join("vm")
                .join("runtime")
                .join(format!("{}.cer", name)),
            stdlib
                .join("vm")
                .join("runtime")
                .join(format!("{}.cerberus", name)),
            stdlib
                .join("runtime")
                .join("vm")
                .join(format!("{}.cer", name)),
            stdlib
                .join("runtime")
                .join("vm")
                .join(format!("{}.cerberus", name)),
            stdlib
                .join("runtime")
                .join("vm")
                .join("runtime")
                .join(format!("{}.cer", name)),
            stdlib
                .join("runtime")
                .join("vm")
                .join("runtime")
                .join(format!("{}.cerberus", name)),
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
