use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, bail};

const TOOLCHAIN_MAGIC: &str = "cerberus_toolchain_v1";
const TOOLCHAIN_MARKER: &str = "::code::";
const TOOLCHAIN_FORMAT_VM_TEXT: &str = "vm_text_script";
const TOOLCHAIN_VM_VERSION: i64 = 1;
const TOOLCHAIN_ABI_VERSION: i64 = 1;
const TOOLCHAIN_ARTIFACT_KIND: &str = "vm_package";
const TOOLCHAIN_FEATURES_DEFAULT: &str = "core,io,collections,fs,time,strings";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AotBackend {
    Auto,
    Asm,
    Rust,
}

impl AotBackend {
    pub fn parse(raw: &str) -> Option<Self> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "auto" => Some(Self::Auto),
            "asm" => Some(Self::Asm),
            "rust" => Some(Self::Rust),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Asm => "asm",
            Self::Rust => "rust",
        }
    }
}

pub fn compile_vm_file_to_native_exe(input_path: &Path, output_path: &Path) -> anyhow::Result<()> {
    compile_vm_file_to_native_exe_with_backend(input_path, output_path, AotBackend::Auto)
}

pub fn compile_vm_file_to_native_exe_with_backend(
    input_path: &Path,
    output_path: &Path,
    backend: AotBackend,
) -> anyhow::Result<()> {
    let raw = fs::read_to_string(input_path)
        .with_context(|| format!("aot: failed to read {}", input_path.display()))?;
    let script = normalize_vm_script_or_package(&raw)?;
    if script.trim().is_empty() {
        bail!("aot: input script is empty");
    }

    if let Some(parent) = output_path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).with_context(|| {
                format!(
                    "aot: failed to create output directory {}",
                    parent.display()
                )
            })?;
        }
    }

    match backend {
        AotBackend::Rust => compile_script_with_rustc(&script, output_path),
        AotBackend::Asm => compile_script_with_asm_backend(&script, output_path),
        AotBackend::Auto => {
            if let Err(asm_err) = compile_script_with_asm_backend(&script, output_path) {
                eprintln!("note: asm backend unavailable/incompatible ({asm_err}), falling back to rust backend");
                compile_script_with_rustc(&script, output_path)?;
            }
            Ok(())
        }
    }
}

fn compile_script_with_rustc(script: &str, output_path: &Path) -> anyhow::Result<()> {
    let workspace = create_temp_workspace_path();
    fs::create_dir_all(&workspace)
        .with_context(|| format!("aot: failed to create temp workspace {}", workspace.display()))?;

    let source_path = workspace.join("cerberus_aot_main.rs");
    let source = generate_rust_source(script);
    fs::write(&source_path, source).with_context(|| {
        format!(
            "aot: failed to write generated source {}",
            source_path.display()
        )
    })?;

    let status = Command::new("rustc")
        .arg("--edition=2021")
        .arg("-O")
        .arg(&source_path)
        .arg("-o")
        .arg(output_path)
        .status()
        .context("aot: failed to invoke rustc")?;

    let _ = fs::remove_file(&source_path);
    let _ = fs::remove_dir_all(&workspace);

    if !status.success() {
        bail!("aot: rustc failed with status {}", status);
    }
    Ok(())
}

fn compile_script_with_asm_backend(script: &str, output_path: &Path) -> anyhow::Result<()> {
    let output_lines = evaluate_asm_stdout_lines(script)?;
    if !command_exists("nasm") {
        bail!("aot-asm: nasm not found in PATH");
    }
    let linker = find_linker_driver()
        .ok_or_else(|| anyhow::anyhow!("aot-asm: linker not found in PATH (expected one of cc/gcc/clang)"))?;

    let workspace = create_temp_workspace_path();
    fs::create_dir_all(&workspace)
        .with_context(|| format!("aot-asm: failed to create temp workspace {}", workspace.display()))?;

    let asm_path = workspace.join("cerberus_aot_main.asm");
    let object_path = if cfg!(windows) {
        workspace.join("cerberus_aot_main.obj")
    } else {
        workspace.join("cerberus_aot_main.o")
    };
    let asm_format = if cfg!(windows) {
        "win64"
    } else if cfg!(target_os = "linux") {
        "elf64"
    } else {
        let _ = fs::remove_dir_all(&workspace);
        bail!(
            "aot-asm: unsupported host platform {}",
            std::env::consts::OS
        );
    };

    let asm_src = if cfg!(windows) {
        generate_nasm_win64_source(&output_lines)
    } else {
        generate_nasm_linux_source(&output_lines)
    };
    fs::write(&asm_path, asm_src)
        .with_context(|| format!("aot-asm: failed to write {}", asm_path.display()))?;

    let asm_status = Command::new("nasm")
        .arg("-f")
        .arg(asm_format)
        .arg(&asm_path)
        .arg("-o")
        .arg(&object_path)
        .status()
        .context("aot-asm: failed to invoke nasm")?;
    if !asm_status.success() {
        let _ = fs::remove_dir_all(&workspace);
        bail!("aot-asm: nasm failed with status {}", asm_status);
    }

    let link_status = Command::new(&linker)
        .arg(&object_path)
        .arg("-o")
        .arg(output_path)
        .status()
        .with_context(|| format!("aot-asm: failed to invoke linker {}", linker))?;
    let _ = fs::remove_dir_all(&workspace);
    if !link_status.success() {
        bail!("aot-asm: linker {} failed with status {}", linker, link_status);
    }
    Ok(())
}

pub fn normalize_vm_script_or_package(raw: &str) -> anyhow::Result<String> {
    resolve_vm_script_or_package(raw)
}

fn create_temp_workspace_path() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    std::env::temp_dir().join(format!("cerberus_aot_{}_{}", std::process::id(), nanos))
}

fn resolve_vm_script_or_package(raw: &str) -> anyhow::Result<String> {
    let trimmed = raw.trim_start();
    if !trimmed.starts_with(TOOLCHAIN_MAGIC) {
        return Ok(raw.to_string());
    }

    let marker = raw
        .find(TOOLCHAIN_MARKER)
        .ok_or_else(|| anyhow::anyhow!("aot: invalid package, missing ::code:: marker"))?;
    let header = &raw[..marker];
    let payload = &raw[(marker + TOOLCHAIN_MARKER.len())..];

    let tokens = header
        .split(';')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    if tokens.len() < 2 {
        bail!("aot: invalid package header");
    }
    if tokens[0] != TOOLCHAIN_MAGIC {
        bail!("aot: invalid package magic");
    }
    if tokens[1] != TOOLCHAIN_FORMAT_VM_TEXT {
        bail!("aot: unsupported package format '{}'", tokens[1]);
    }

    let mut vm_version = TOOLCHAIN_VM_VERSION;
    let mut abi_version = TOOLCHAIN_ABI_VERSION;
    let mut features = TOOLCHAIN_FEATURES_DEFAULT.to_string();
    let mut artifact = TOOLCHAIN_ARTIFACT_KIND.to_string();
    let mut checksum = None::<String>;
    let mut toolchain_name = None::<String>;
    let mut entry = None::<String>;
    let mut limit_steps = None::<String>;
    let mut limit_stack = None::<String>;
    let mut limit_call = None::<String>;

    for token in tokens.iter().skip(2) {
        if let Some(v) = token.strip_prefix("vm=") {
            let t = v.trim();
            if !is_int_token(t) {
                bail!("aot: vm must be integer");
            }
            vm_version = t
                .parse::<i64>()
                .map_err(|_| anyhow::anyhow!("aot: vm must be integer"))?;
            continue;
        }
        if let Some(v) = token.strip_prefix("abi=") {
            let t = v.trim();
            if !is_int_token(t) {
                bail!("aot: abi must be integer");
            }
            abi_version = t
                .parse::<i64>()
                .map_err(|_| anyhow::anyhow!("aot: abi must be integer"))?;
            continue;
        }
        if let Some(v) = token.strip_prefix("features=") {
            let t = v.trim();
            if t.is_empty() {
                bail!("aot: unsupported features {}", t);
            }
            features = t.to_string();
            continue;
        }
        if let Some(v) = token.strip_prefix("artifact=") {
            artifact = v.trim().to_string();
            continue;
        }
        if let Some(v) = token.strip_prefix("toolchain=") {
            toolchain_name = Some(v.trim().to_string());
            continue;
        }
        if let Some(v) = token.strip_prefix("checksum=") {
            checksum = Some(v.trim().to_string());
            continue;
        }
        if let Some(v) = token.strip_prefix("entry=") {
            let t = v.trim();
            if !t.is_empty() {
                entry = Some(t.to_string());
            }
            continue;
        }
        if let Some(v) = token.strip_prefix("limit_steps=") {
            let t = v.trim();
            if !t.is_empty() {
                limit_steps = Some(t.to_string());
            }
            continue;
        }
        if let Some(v) = token.strip_prefix("limit_stack=") {
            let t = v.trim();
            if !t.is_empty() {
                limit_stack = Some(t.to_string());
            }
            continue;
        }
        if let Some(v) = token.strip_prefix("limit_call=") {
            let t = v.trim();
            if !t.is_empty() {
                limit_call = Some(t.to_string());
            }
            continue;
        }

        bail!("aot: unknown package header field '{}'", token);
    }

    if payload.trim().is_empty() {
        bail!("aot: package payload is empty");
    }
    if vm_version != TOOLCHAIN_VM_VERSION {
        bail!("aot: unsupported vm version {}", vm_version);
    }
    if abi_version != TOOLCHAIN_ABI_VERSION {
        bail!("aot: unsupported abi version {}", abi_version);
    }
    if !features_supported(&features) {
        bail!("aot: unsupported features {}", features);
    }
    if artifact != TOOLCHAIN_ARTIFACT_KIND {
        bail!("aot: unsupported artifact {}", artifact);
    }
    if let Some(name) = toolchain_name {
        if name.trim().is_empty() {
            bail!("aot: toolchain must not be empty");
        }
    }
    if let Some(sum) = checksum {
        if !is_int_token(sum.trim()) {
            bail!("aot: checksum must be integer");
        }
        let got = toolchain_checksum(payload);
        if got != sum.trim() {
            bail!("aot: checksum mismatch");
        }
    }
    if let Some(v) = &limit_steps {
        if !is_int_token(v) {
            bail!("aot: limit_steps must be integer");
        }
    }
    if let Some(v) = &limit_stack {
        if !is_int_token(v) {
            bail!("aot: limit_stack must be integer");
        }
    }
    if let Some(v) = &limit_call {
        if !is_int_token(v) {
            bail!("aot: limit_call must be integer");
        }
    }

    let mut script = String::new();
    script.push_str("@cerberus_vm 1;");
    if let Some(v) = entry {
        script.push_str("@entry ");
        script.push_str(&v);
        script.push(';');
    }
    if let Some(v) = limit_steps {
        script.push_str("@limit_steps ");
        script.push_str(&v);
        script.push(';');
    }
    if let Some(v) = limit_stack {
        script.push_str("@limit_stack ");
        script.push_str(&v);
        script.push(';');
    }
    if let Some(v) = limit_call {
        script.push_str("@limit_call ");
        script.push_str(&v);
        script.push(';');
    }
    script.push_str(payload);
    Ok(script)
}

fn is_int_token(s: &str) -> bool {
    let t = s.trim();
    if t.is_empty() {
        return false;
    }
    if t.starts_with('-') {
        if t.len() == 1 {
            return false;
        }
        return t[1..].chars().all(|ch| ch.is_ascii_digit());
    }
    t.chars().all(|ch| ch.is_ascii_digit())
}

fn features_supported(spec: &str) -> bool {
    let s = spec.trim();
    if s.is_empty() {
        return false;
    }
    for item in s.split(',').map(str::trim) {
        if item.is_empty() {
            return false;
        }
        if !matches!(item, "core" | "io" | "collections" | "fs" | "time" | "strings") {
            return false;
        }
    }
    true
}

fn toolchain_checksum(text: &str) -> String {
    let mut sum1: i64 = 1;
    let mut sum2: i64 = 0;
    let modv: i64 = 32003;
    for ch in text.chars() {
        let v = toolchain_char_value(ch);
        sum1 += v;
        while sum1 >= modv {
            sum1 -= modv;
        }
        sum2 += sum1;
        while sum2 >= modv {
            sum2 -= modv;
        }
    }
    (sum2 * modv + sum1).to_string()
}

fn toolchain_char_value(ch: char) -> i64 {
    let digits = "0123456789";
    if let Some(idx) = digits.find(ch) {
        return idx as i64 + 1;
    }
    let lower = "abcdefghijklmnopqrstuvwxyz";
    if let Some(idx) = lower.find(ch) {
        return idx as i64 + 11;
    }
    let upper = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    if let Some(idx) = upper.find(ch) {
        return idx as i64 + 37;
    }
    let punct = " _@#-+*/=<>()[]:,.?!\"$%&|\\{}~;'";
    if let Some(idx) = punct.find(ch) {
        return idx as i64 + 79;
    }
    251
}

#[derive(Clone, Debug, PartialEq)]
enum AsmValue {
    Int(i64),
    Bool(bool),
    Str(String),
}

fn evaluate_asm_stdout_lines(script: &str) -> anyhow::Result<Vec<String>> {
    let lines = split_vm_lines(script);
    let mut stack = Vec::<AsmValue>::new();
    let mut output = Vec::<String>::new();

    for line in lines {
        let (op, arg) = parse_vm_op_arg(&line);
        match op.as_str() {
            "" | "#" | "@cerberus_vm" | "@entry" | "@limit_steps" | "@limit_stack"
            | "@limit_call" | "label" | "nop" => {}
            "halt" => break,
            "const_int" => {
                let v = arg
                    .parse::<i64>()
                    .map_err(|_| anyhow::anyhow!("aot-asm: const_int invalid integer '{}'", arg))?;
                stack.push(AsmValue::Int(v));
            }
            "const_bool" => {
                if arg == "true" {
                    stack.push(AsmValue::Bool(true));
                } else if arg == "false" {
                    stack.push(AsmValue::Bool(false));
                } else {
                    bail!("aot-asm: const_bool invalid bool '{}'", arg);
                }
            }
            "const_str" => {
                stack.push(AsmValue::Str(parse_vm_string_literal(&arg)));
            }
            "add" => {
                let b = pop_int_asm(&mut stack, "add")?;
                let a = pop_int_asm(&mut stack, "add")?;
                stack.push(AsmValue::Int(a + b));
            }
            "sub" => {
                let b = pop_int_asm(&mut stack, "sub")?;
                let a = pop_int_asm(&mut stack, "sub")?;
                stack.push(AsmValue::Int(a - b));
            }
            "mul" => {
                let b = pop_int_asm(&mut stack, "mul")?;
                let a = pop_int_asm(&mut stack, "mul")?;
                stack.push(AsmValue::Int(a * b));
            }
            "div" => {
                let b = pop_int_asm(&mut stack, "div")?;
                if b == 0 {
                    bail!("aot-asm: div by zero");
                }
                let a = pop_int_asm(&mut stack, "div")?;
                stack.push(AsmValue::Int(a / b));
            }
            "eq" => {
                let b = pop_asm(&mut stack, "eq")?;
                let a = pop_asm(&mut stack, "eq")?;
                stack.push(AsmValue::Bool(a == b));
            }
            "ne" => {
                let b = pop_asm(&mut stack, "ne")?;
                let a = pop_asm(&mut stack, "ne")?;
                stack.push(AsmValue::Bool(a != b));
            }
            "lt" => {
                let b = pop_int_asm(&mut stack, "lt")?;
                let a = pop_int_asm(&mut stack, "lt")?;
                stack.push(AsmValue::Bool(a < b));
            }
            "le" => {
                let b = pop_int_asm(&mut stack, "le")?;
                let a = pop_int_asm(&mut stack, "le")?;
                stack.push(AsmValue::Bool(a <= b));
            }
            "gt" => {
                let b = pop_int_asm(&mut stack, "gt")?;
                let a = pop_int_asm(&mut stack, "gt")?;
                stack.push(AsmValue::Bool(a > b));
            }
            "ge" => {
                let b = pop_int_asm(&mut stack, "ge")?;
                let a = pop_int_asm(&mut stack, "ge")?;
                stack.push(AsmValue::Bool(a >= b));
            }
            "and" => {
                let b = pop_bool_asm(&mut stack, "and")?;
                let a = pop_bool_asm(&mut stack, "and")?;
                stack.push(AsmValue::Bool(a && b));
            }
            "or" => {
                let b = pop_bool_asm(&mut stack, "or")?;
                let a = pop_bool_asm(&mut stack, "or")?;
                stack.push(AsmValue::Bool(a || b));
            }
            "not" => {
                let a = pop_bool_asm(&mut stack, "not")?;
                stack.push(AsmValue::Bool(!a));
            }
            "neg" => {
                let a = pop_int_asm(&mut stack, "neg")?;
                stack.push(AsmValue::Int(-a));
            }
            "strlen" => {
                let s = pop_str_asm(&mut stack, "strlen")?;
                stack.push(AsmValue::Int(s.chars().count() as i64));
            }
            "concat" => {
                let b = pop_str_asm(&mut stack, "concat")?;
                let a = pop_str_asm(&mut stack, "concat")?;
                stack.push(AsmValue::Str(format!("{a}{b}")));
            }
            "println" => {
                let v = pop_asm(&mut stack, "println")?;
                output.push(asm_value_to_string(&v));
            }
            _ => bail!("aot-asm: unsupported opcode '{}'", op),
        }
    }
    Ok(output)
}

fn pop_asm(stack: &mut Vec<AsmValue>, op: &str) -> anyhow::Result<AsmValue> {
    stack
        .pop()
        .ok_or_else(|| anyhow::anyhow!("aot-asm: {} stack underflow", op))
}

fn pop_int_asm(stack: &mut Vec<AsmValue>, op: &str) -> anyhow::Result<i64> {
    match pop_asm(stack, op)? {
        AsmValue::Int(v) => Ok(v),
        _ => bail!("aot-asm: {} expected int", op),
    }
}

fn pop_bool_asm(stack: &mut Vec<AsmValue>, op: &str) -> anyhow::Result<bool> {
    match pop_asm(stack, op)? {
        AsmValue::Bool(v) => Ok(v),
        _ => bail!("aot-asm: {} expected bool", op),
    }
}

fn pop_str_asm(stack: &mut Vec<AsmValue>, op: &str) -> anyhow::Result<String> {
    match pop_asm(stack, op)? {
        AsmValue::Str(v) => Ok(v),
        _ => bail!("aot-asm: {} expected string", op),
    }
}

fn asm_value_to_string(v: &AsmValue) -> String {
    match v {
        AsmValue::Int(i) => i.to_string(),
        AsmValue::Bool(b) => {
            if *b {
                "true".to_string()
            } else {
                "false".to_string()
            }
        }
        AsmValue::Str(s) => s.clone(),
    }
}

fn split_vm_lines(src: &str) -> Vec<String> {
    let mut out = Vec::<String>::new();
    let mut cur = String::new();
    let mut chars = src.chars().peekable();
    let mut in_str = false;
    let mut quote = '\0';
    while let Some(ch) = chars.next() {
        if in_str {
            cur.push(ch);
            if ch == '\\' {
                if let Some(next) = chars.next() {
                    cur.push(next);
                }
                continue;
            }
            if ch == quote {
                in_str = false;
                quote = '\0';
            }
            continue;
        }
        if ch == '\'' || ch == '"' {
            in_str = true;
            quote = ch;
            cur.push(ch);
            continue;
        }
        if ch == ';' || ch == '\n' || ch == '\r' {
            let line = strip_vm_inline_comment(cur.trim());
            if !line.is_empty() {
                out.push(line);
            }
            cur.clear();
            continue;
        }
        cur.push(ch);
    }

    let line = strip_vm_inline_comment(cur.trim());
    if !line.is_empty() {
        out.push(line);
    }
    out
}

fn strip_vm_inline_comment(line: &str) -> String {
    let mut out = String::new();
    let mut chars = line.chars().peekable();
    let mut in_str = false;
    let mut quote = '\0';
    while let Some(ch) = chars.next() {
        if in_str {
            out.push(ch);
            if ch == '\\' {
                if let Some(next) = chars.next() {
                    out.push(next);
                }
                continue;
            }
            if ch == quote {
                in_str = false;
                quote = '\0';
            }
            continue;
        }
        if ch == '\'' || ch == '"' {
            in_str = true;
            quote = ch;
            out.push(ch);
            continue;
        }
        if ch == '#' {
            break;
        }
        out.push(ch);
    }
    out.trim().to_string()
}

fn parse_vm_op_arg(line: &str) -> (String, String) {
    let s = line.trim();
    if s.is_empty() {
        return (String::new(), String::new());
    }
    let mut split = None::<usize>;
    for (idx, ch) in s.char_indices() {
        if ch.is_whitespace() {
            split = Some(idx);
            break;
        }
    }
    match split {
        Some(idx) => (
            s[..idx].trim().to_ascii_lowercase(),
            s[idx..].trim().to_string(),
        ),
        None => (s.to_ascii_lowercase(), String::new()),
    }
}

fn parse_vm_string_literal(raw: &str) -> String {
    let s = raw.trim();
    if s.len() >= 2 && s.starts_with('\'') && s.ends_with('\'') {
        let inner = &s[1..s.len() - 1];
        return inner.replace("''", "'");
    }
    if s.len() >= 2 && s.starts_with('"') && s.ends_with('"') {
        return decode_double_quoted(&s[1..s.len() - 1]);
    }
    s.to_string()
}

fn decode_double_quoted(inner: &str) -> String {
    let mut out = String::new();
    let mut chars = inner.chars();
    while let Some(ch) = chars.next() {
        if ch != '\\' {
            out.push(ch);
            continue;
        }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some('r') => out.push('\r'),
            Some('t') => out.push('\t'),
            Some('\\') => out.push('\\'),
            Some('"') => out.push('"'),
            Some(other) => out.push(other),
            None => out.push('\\'),
        }
    }
    out
}

fn command_exists(cmd: &str) -> bool {
    Command::new(cmd).arg("--version").output().is_ok()
        || Command::new(cmd).arg("-v").output().is_ok()
}

fn find_linker_driver() -> Option<String> {
    if let Ok(raw) = std::env::var("CERBERUS_AOT_LINKER") {
        let candidate = raw.trim();
        if !candidate.is_empty() && command_exists(candidate) {
            return Some(candidate.to_string());
        }
    }
    for candidate in ["cc", "gcc", "clang"] {
        if command_exists(candidate) {
            return Some(candidate.to_string());
        }
    }
    None
}

fn bytes_to_nasm_db(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return "0".to_string();
    }
    bytes
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn generate_nasm_linux_source(lines: &[String]) -> String {
    let mut payload = lines.join("\n").into_bytes();
    if !payload.is_empty() {
        payload.push(b'\n');
    }

    let mut out = String::new();
    out.push_str("default rel\n");
    out.push_str("global _start\n\n");
    out.push_str("section .text\n");
    out.push_str("_start:\n");
    if !payload.is_empty() {
        out.push_str("    mov rax, 1\n");
        out.push_str("    mov rdi, 1\n");
        out.push_str("    lea rsi, [rel msg]\n");
        out.push_str("    mov rdx, msg_len\n");
        out.push_str("    syscall\n");
    }
    out.push_str("    mov rax, 60\n");
    out.push_str("    xor rdi, rdi\n");
    out.push_str("    syscall\n");

    if !payload.is_empty() {
        out.push_str("\nsection .rodata\n");
        out.push_str("msg: db ");
        out.push_str(&bytes_to_nasm_db(&payload));
        out.push('\n');
        out.push_str("msg_len equ $ - msg\n");
    }
    out
}

fn generate_nasm_win64_source(lines: &[String]) -> String {
    let mut out = String::new();
    out.push_str("default rel\n");
    out.push_str("extern puts\n");
    out.push_str("global main\n\n");
    out.push_str("section .text\n");
    out.push_str("main:\n");
    out.push_str("    sub rsp, 40\n");
    for idx in 0..lines.len() {
        out.push_str(&format!("    lea rcx, [rel msg{}]\n", idx));
        out.push_str("    call puts\n");
    }
    out.push_str("    xor eax, eax\n");
    out.push_str("    add rsp, 40\n");
    out.push_str("    ret\n");

    if !lines.is_empty() {
        out.push_str("\nsection .rdata\n");
        for (idx, line) in lines.iter().enumerate() {
            let mut bytes = line.as_bytes().to_vec();
            bytes.push(0);
            out.push_str(&format!("msg{}: db {}\n", idx, bytes_to_nasm_db(&bytes)));
        }
    }

    out
}

fn generate_rust_source(script: &str) -> String {
    let mut out = String::new();
    out.push_str("use std::collections::HashMap;\nuse std::fs;\nuse std::path::Path;\n\n");
    out.push_str("#[derive(Clone, Debug, PartialEq)]\n");
    out.push_str("enum Value {\n");
    out.push_str("    Int(i64),\n");
    out.push_str("    Bool(bool),\n");
    out.push_str("    Str(String),\n");
    out.push_str("    Vec(Vec<Value>),\n");
    out.push_str("    Map(Vec<(String, String)>),\n");
    out.push_str("}\n\n");
    out.push_str("const SCRIPT: &str = ");
    out.push_str(&format!("{script:?}"));
    out.push_str(";\n\n");
    out.push_str(
        r###"
#[derive(Clone)]
struct Frame {
    ret_ip: usize,
    locals: Vec<Value>,
}

fn main() {
    if let Err(e) = run_script(SCRIPT) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run_script(src: &str) -> Result<(), String> {
    let lines = split_lines(src);
    if lines.is_empty() {
        return Ok(());
    }

    let mut labels = HashMap::<String, usize>::new();
    let mut entry = None::<String>;
    let mut max_steps: u64 = 1_000_000;
    let mut max_stack: usize = 100_000;
    let mut max_call: usize = 4096;

    for (idx, line) in lines.iter().enumerate() {
        let (op, arg) = parse_op_arg(line);
        match op.as_str() {
            "label" => {
                if arg.is_empty() {
                    return Err("label: missing label name".to_string());
                }
                labels.insert(arg, idx);
            }
            "@entry" => {
                if arg.is_empty() {
                    return Err("@entry: missing target".to_string());
                }
                entry = Some(arg);
            }
            "@limit_steps" => {
                max_steps = parse_positive_u64(&arg, "@limit_steps")?;
            }
            "@limit_stack" => {
                max_stack = parse_positive_usize(&arg, "@limit_stack")?;
            }
            "@limit_call" => {
                max_call = parse_positive_usize(&arg, "@limit_call")?;
            }
            _ => {}
        }
    }

    let mut stack = Vec::<Value>::new();
    let mut locals = make_locals(16);
    let mut frames = Vec::<Frame>::new();
    let mut steps: u64 = 0;
    let mut ip = if let Some(target) = entry {
        resolve_target(&target, &labels)?
    } else {
        0
    };

    while ip < lines.len() {
        if steps >= max_steps {
            return Err(format!("limit: step exceeded {}", max_steps));
        }
        steps += 1;

        let (op, arg) = parse_op_arg(&lines[ip]);
        match op.as_str() {
            "" | "#" | "label" | "nop" => {
                ip += 1;
            }
            "@cerberus_vm" | "@entry" => {
                ip += 1;
            }
            "@limit_steps" | "limit_steps" => {
                max_steps = parse_positive_u64(&arg, "limit_steps")?;
                ip += 1;
            }
            "@limit_stack" | "limit_stack" => {
                max_stack = parse_positive_usize(&arg, "limit_stack")?;
                ip += 1;
            }
            "@limit_call" | "limit_call" => {
                max_call = parse_positive_usize(&arg, "limit_call")?;
                ip += 1;
            }
            "halt" => return Ok(()),
            "locals" => {
                let count = parse_non_negative_usize(&arg, "locals")?;
                locals = make_locals(count);
                ip += 1;
            }
            "const_int" => {
                let v = arg
                    .parse::<i64>()
                    .map_err(|_| format!("const_int: invalid integer '{}'", arg))?;
                stack.push(Value::Int(v));
                ip += 1;
            }
            "const_bool" => {
                if arg == "true" {
                    stack.push(Value::Bool(true));
                } else if arg == "false" {
                    stack.push(Value::Bool(false));
                } else {
                    return Err(format!("const_bool: invalid bool '{}'", arg));
                }
                ip += 1;
            }
            "const_str" => {
                stack.push(Value::Str(parse_str_literal(&arg)));
                ip += 1;
            }
            "load" => {
                let idx = parse_non_negative_usize(&arg, "load")?;
                if idx >= locals.len() {
                    return Err("load: invalid local index".to_string());
                }
                stack.push(locals[idx].clone());
                ip += 1;
            }
            "store" => {
                let idx = parse_non_negative_usize(&arg, "store")?;
                if idx >= locals.len() {
                    return Err("store: invalid local index".to_string());
                }
                let v = pop(&mut stack, "store")?;
                locals[idx] = v;
                ip += 1;
            }
            "vec_new" => {
                stack.push(Value::Vec(Vec::new()));
                ip += 1;
            }
            "vec_len" => {
                let vec = pop_vec(&mut stack, "vec_len")?;
                stack.push(Value::Int(vec.len() as i64));
                ip += 1;
            }
            "vec_get" => {
                let idx = pop_int(&mut stack, "vec_get")?;
                let vec = pop_vec(&mut stack, "vec_get")?;
                if idx < 0 {
                    return Err("vec_get: index out of range".to_string());
                }
                let idx_u = idx as usize;
                if idx_u >= vec.len() {
                    return Err("vec_get: index out of range".to_string());
                }
                stack.push(vec[idx_u].clone());
                ip += 1;
            }
            "vec_set" => {
                let value = pop(&mut stack, "vec_set")?;
                let idx = pop_int(&mut stack, "vec_set")?;
                let mut vec = pop_vec(&mut stack, "vec_set")?;
                if idx < 0 {
                    return Err("vec_set: index out of range".to_string());
                }
                let idx_u = idx as usize;
                if idx_u >= vec.len() {
                    return Err("vec_set: index out of range".to_string());
                }
                if !vec_accepts(&vec, &value) {
                    return Err("vec_set: element type mismatch".to_string());
                }
                vec[idx_u] = value;
                stack.push(Value::Vec(vec));
                ip += 1;
            }
            "vec_push" => {
                let value = pop(&mut stack, "vec_push")?;
                let mut vec = pop_vec(&mut stack, "vec_push")?;
                if !vec_accepts(&vec, &value) {
                    return Err("vec_push: element type mismatch".to_string());
                }
                vec.push(value);
                stack.push(Value::Vec(vec));
                ip += 1;
            }
            "vec_remove" => {
                let idx = pop_int(&mut stack, "vec_remove")?;
                let mut vec = pop_vec(&mut stack, "vec_remove")?;
                if idx < 0 {
                    return Err("vec_remove: index out of range".to_string());
                }
                let idx_u = idx as usize;
                if idx_u >= vec.len() {
                    return Err("vec_remove: index out of range".to_string());
                }
                let _ = vec.remove(idx_u);
                stack.push(Value::Vec(vec));
                ip += 1;
            }
            "vec_last" => {
                let vec = pop_vec(&mut stack, "vec_last")?;
                if vec.is_empty() {
                    return Err("vec_last: empty vector".to_string());
                }
                stack.push(vec.last().expect("non-empty").clone());
                ip += 1;
            }
            "vec_pop" => {
                let mut vec = pop_vec(&mut stack, "vec_pop")?;
                if vec.is_empty() {
                    return Err("vec_pop: empty vector".to_string());
                }
                let _ = vec.pop();
                stack.push(Value::Vec(vec));
                ip += 1;
            }
            "map_new" => {
                stack.push(Value::Map(Vec::new()));
                ip += 1;
            }
            "map_len" => {
                let map = pop_map(&mut stack, "map_len")?;
                stack.push(Value::Int(map.len() as i64));
                ip += 1;
            }
            "map_set" => {
                let value = pop_str(&mut stack, "map_set")?;
                let key = pop_str(&mut stack, "map_set")?;
                let mut map = pop_map(&mut stack, "map_set")?;
                if let Some(idx) = map_find_idx(&map, &key) {
                    map[idx].1 = value;
                } else {
                    map.push((key, value));
                }
                stack.push(Value::Map(map));
                ip += 1;
            }
            "map_get" => {
                let key = pop_str(&mut stack, "map_get")?;
                let map = pop_map(&mut stack, "map_get")?;
                if let Some(idx) = map_find_idx(&map, &key) {
                    stack.push(Value::Str(map[idx].1.clone()));
                    ip += 1;
                } else {
                    return Err("map_get: key not found".to_string());
                }
            }
            "map_has" => {
                let key = pop_str(&mut stack, "map_has")?;
                let map = pop_map(&mut stack, "map_has")?;
                stack.push(Value::Bool(map_find_idx(&map, &key).is_some()));
                ip += 1;
            }
            "map_remove" => {
                let key = pop_str(&mut stack, "map_remove")?;
                let mut map = pop_map(&mut stack, "map_remove")?;
                if let Some(idx) = map_find_idx(&map, &key) {
                    let _ = map.remove(idx);
                }
                stack.push(Value::Map(map));
                ip += 1;
            }
            "add" => {
                let b = pop_int(&mut stack, "add")?;
                let a = pop_int(&mut stack, "add")?;
                stack.push(Value::Int(a + b));
                ip += 1;
            }
            "sub" => {
                let b = pop_int(&mut stack, "sub")?;
                let a = pop_int(&mut stack, "sub")?;
                stack.push(Value::Int(a - b));
                ip += 1;
            }
            "mul" => {
                let b = pop_int(&mut stack, "mul")?;
                let a = pop_int(&mut stack, "mul")?;
                stack.push(Value::Int(a * b));
                ip += 1;
            }
            "div" => {
                let b = pop_int(&mut stack, "div")?;
                if b == 0 {
                    return Err("div: division by zero".to_string());
                }
                let a = pop_int(&mut stack, "div")?;
                stack.push(Value::Int(a / b));
                ip += 1;
            }
            "eq" => {
                let b = pop(&mut stack, "eq")?;
                let a = pop(&mut stack, "eq")?;
                stack.push(Value::Bool(a == b));
                ip += 1;
            }
            "ne" => {
                let b = pop(&mut stack, "ne")?;
                let a = pop(&mut stack, "ne")?;
                stack.push(Value::Bool(a != b));
                ip += 1;
            }
            "lt" => {
                let b = pop_int(&mut stack, "lt")?;
                let a = pop_int(&mut stack, "lt")?;
                stack.push(Value::Bool(a < b));
                ip += 1;
            }
            "le" => {
                let b = pop_int(&mut stack, "le")?;
                let a = pop_int(&mut stack, "le")?;
                stack.push(Value::Bool(a <= b));
                ip += 1;
            }
            "gt" => {
                let b = pop_int(&mut stack, "gt")?;
                let a = pop_int(&mut stack, "gt")?;
                stack.push(Value::Bool(a > b));
                ip += 1;
            }
            "ge" => {
                let b = pop_int(&mut stack, "ge")?;
                let a = pop_int(&mut stack, "ge")?;
                stack.push(Value::Bool(a >= b));
                ip += 1;
            }
            "and" => {
                let b = pop_bool(&mut stack, "and")?;
                let a = pop_bool(&mut stack, "and")?;
                stack.push(Value::Bool(a && b));
                ip += 1;
            }
            "or" => {
                let b = pop_bool(&mut stack, "or")?;
                let a = pop_bool(&mut stack, "or")?;
                stack.push(Value::Bool(a || b));
                ip += 1;
            }
            "not" => {
                let v = pop_bool(&mut stack, "not")?;
                stack.push(Value::Bool(!v));
                ip += 1;
            }
            "neg" => {
                let v = pop_int(&mut stack, "neg")?;
                stack.push(Value::Int(-v));
                ip += 1;
            }
            "strlen" => {
                let s = pop_str(&mut stack, "strlen")?;
                stack.push(Value::Int(s.chars().count() as i64));
                ip += 1;
            }
            "concat" => {
                let b = pop_str(&mut stack, "concat")?;
                let a = pop_str(&mut stack, "concat")?;
                let mut out = String::with_capacity(a.len() + b.len());
                out.push_str(&a);
                out.push_str(&b);
                stack.push(Value::Str(out));
                ip += 1;
            }
            "substr" => {
                let len = pop_int(&mut stack, "substr")?;
                let start = pop_int(&mut stack, "substr")?;
                let s = pop_str(&mut stack, "substr")?;
                let out = substr_chars(&s, start, len)?;
                stack.push(Value::Str(out));
                ip += 1;
            }
            "replace" => {
                let to = pop_str(&mut stack, "replace")?;
                let from = pop_str(&mut stack, "replace")?;
                let s = pop_str(&mut stack, "replace")?;
                stack.push(Value::Str(s.replace(&from, &to)));
                ip += 1;
            }
            "jump" => {
                ip = resolve_target(&arg, &labels)?;
            }
            "jump_if_false" => {
                let cond = pop_bool(&mut stack, "jump_if_false")?;
                if !cond {
                    ip = resolve_target(&arg, &labels)?;
                } else {
                    ip += 1;
                }
            }
            "call" => {
                if frames.len() >= max_call {
                    return Err(format!("limit: call depth exceeded {}", max_call));
                }
                let target = resolve_target(&arg, &labels)?;
                frames.push(Frame {
                    ret_ip: ip + 1,
                    locals: locals.clone(),
                });
                ip = target;
            }
            "ret" => {
                if let Some(frame) = frames.pop() {
                    locals = frame.locals;
                    ip = frame.ret_ip;
                } else {
                    return Ok(());
                }
            }
            "ret_val" => {
                let v = pop(&mut stack, "ret_val")?;
                if let Some(frame) = frames.pop() {
                    locals = frame.locals;
                    ip = frame.ret_ip;
                    stack.push(v);
                } else {
                    stack.push(v);
                    return Ok(());
                }
            }
            "println" => {
                let v = pop(&mut stack, "println")?;
                println!("{}", value_to_string(&v));
                ip += 1;
            }
            "readfile" => {
                let path = pop_str(&mut stack, "readfile")?;
                let content = fs::read_to_string(&path)
                    .map_err(|e| format!("readfile: {} ({})", path, e))?;
                stack.push(Value::Str(content));
                ip += 1;
            }
            "writefile" => {
                let content = pop_str(&mut stack, "writefile")?;
                let path = pop_str(&mut stack, "writefile")?;
                if let Some(parent) = Path::new(&path).parent() {
                    if !parent.as_os_str().is_empty() {
                        fs::create_dir_all(parent)
                            .map_err(|e| format!("writefile: create_dir failed ({})", e))?;
                    }
                }
                fs::write(&path, content).map_err(|e| format!("writefile: {} ({})", path, e))?;
                stack.push(Value::Bool(true));
                ip += 1;
            }
            _ => {
                return Err(format!("unsupported opcode in AOT backend: {}", op));
            }
        }

        if stack.len() > max_stack {
            return Err(format!("limit: stack exceeded {}", max_stack));
        }
    }

    Ok(())
}

fn pop(stack: &mut Vec<Value>, op: &str) -> Result<Value, String> {
    stack
        .pop()
        .ok_or_else(|| format!("{op}: stack underflow"))
}

fn pop_int(stack: &mut Vec<Value>, op: &str) -> Result<i64, String> {
    match pop(stack, op)? {
        Value::Int(v) => Ok(v),
        _ => Err(format!("{op}: expected int")),
    }
}

fn pop_bool(stack: &mut Vec<Value>, op: &str) -> Result<bool, String> {
    match pop(stack, op)? {
        Value::Bool(v) => Ok(v),
        _ => Err(format!("{op}: expected bool")),
    }
}

fn pop_str(stack: &mut Vec<Value>, op: &str) -> Result<String, String> {
    match pop(stack, op)? {
        Value::Str(v) => Ok(v),
        _ => Err(format!("{op}: expected string")),
    }
}

fn pop_vec(stack: &mut Vec<Value>, op: &str) -> Result<Vec<Value>, String> {
    match pop(stack, op)? {
        Value::Vec(v) => Ok(v),
        _ => Err(format!("{op}: expected vector")),
    }
}

fn pop_map(stack: &mut Vec<Value>, op: &str) -> Result<Vec<(String, String)>, String> {
    match pop(stack, op)? {
        Value::Map(v) => Ok(v),
        _ => Err(format!("{op}: expected map")),
    }
}

fn value_kind(v: &Value) -> u8 {
    match v {
        Value::Int(_) => 0,
        Value::Bool(_) => 1,
        Value::Str(_) => 2,
        Value::Vec(_) => 3,
        Value::Map(_) => 4,
    }
}

fn vec_accepts(vec: &[Value], value: &Value) -> bool {
    if vec.is_empty() {
        return true;
    }
    value_kind(&vec[0]) == value_kind(value)
}

fn map_find_idx(map: &[(String, String)], key: &str) -> Option<usize> {
    let mut i = 0usize;
    while i < map.len() {
        if map[i].0 == key {
            return Some(i);
        }
        i += 1;
    }
    None
}

fn value_to_string(v: &Value) -> String {
    match v {
        Value::Int(x) => x.to_string(),
        Value::Bool(x) => x.to_string(),
        Value::Str(x) => x.clone(),
        Value::Vec(items) => {
            let mut out = String::from("[");
            let mut i = 0usize;
            while i < items.len() {
                if i > 0 {
                    out.push_str(", ");
                }
                out.push_str(&value_to_string(&items[i]));
                i += 1;
            }
            out.push(']');
            out
        }
        Value::Map(entries) => {
            let mut out = String::from("{");
            let mut i = 0usize;
            while i < entries.len() {
                if i > 0 {
                    out.push_str(", ");
                }
                out.push_str(&entries[i].0);
                out.push(':');
                out.push_str(&entries[i].1);
                i += 1;
            }
            out.push('}');
            out
        }
    }
}

fn resolve_target(arg: &str, labels: &HashMap<String, usize>) -> Result<usize, String> {
    let trimmed = arg.trim();
    if trimmed.is_empty() {
        return Err("jump: missing target".to_string());
    }
    if let Ok(v) = trimmed.parse::<isize>() {
        if v < 0 {
            return Err(format!("jump: invalid negative target {}", v));
        }
        return Ok(v as usize);
    }
    labels
        .get(trimmed)
        .copied()
        .ok_or_else(|| format!("jump: unknown label '{}'", trimmed))
}

fn parse_positive_u64(arg: &str, name: &str) -> Result<u64, String> {
    let v = arg
        .trim()
        .parse::<u64>()
        .map_err(|_| format!("{name}: expected positive integer"))?;
    if v == 0 {
        return Err(format!("{name}: must be > 0"));
    }
    Ok(v)
}

fn parse_positive_usize(arg: &str, name: &str) -> Result<usize, String> {
    let v = parse_positive_u64(arg, name)?;
    if v > usize::MAX as u64 {
        return Err(format!("{name}: integer too large"));
    }
    Ok(v as usize)
}

fn parse_non_negative_usize(arg: &str, name: &str) -> Result<usize, String> {
    let v = arg
        .trim()
        .parse::<i64>()
        .map_err(|_| format!("{name}: expected integer"))?;
    if v < 0 {
        return Err(format!("{name}: negative index"));
    }
    Ok(v as usize)
}

fn make_locals(count: usize) -> Vec<Value> {
    let mut out = Vec::with_capacity(count);
    let mut i = 0usize;
    while i < count {
        out.push(Value::Int(0));
        i += 1;
    }
    out
}

fn substr_chars(s: &str, start: i64, len: i64) -> Result<String, String> {
    if start < 0 || len < 0 {
        return Err("substr: negative index".to_string());
    }
    let chars = s.chars().collect::<Vec<_>>();
    let start_u = start as usize;
    let len_u = len as usize;
    if start_u > chars.len() || start_u + len_u > chars.len() {
        return Err("substr: out of range".to_string());
    }
    Ok(chars[start_u..start_u + len_u].iter().collect())
}

fn parse_str_literal(raw: &str) -> String {
    let s = raw.trim();
    if s.len() >= 2 && s.starts_with('\'') && s.ends_with('\'') {
        let inner = &s[1..s.len() - 1];
        return inner.replace("''", "'");
    }
    s.to_string()
}

fn parse_op_arg(line: &str) -> (String, String) {
    let s = line.trim();
    if s.is_empty() {
        return (String::new(), String::new());
    }
    if s.starts_with('#') {
        return ("#".to_string(), String::new());
    }
    let mut split = None::<usize>;
    for (idx, ch) in s.char_indices() {
        if ch.is_whitespace() {
            split = Some(idx);
            break;
        }
    }
    match split {
        Some(idx) => {
            let op = s[..idx].trim().to_ascii_lowercase();
            let arg = s[idx..].trim().to_string();
            (op, arg)
        }
        None => (s.to_ascii_lowercase(), String::new()),
    }
}

fn strip_inline_comment(line: &str) -> String {
    let mut out = String::new();
    let mut chars = line.chars().peekable();
    let mut in_str = false;
    while let Some(ch) = chars.next() {
        if in_str {
            out.push(ch);
            if ch == '\'' {
                if chars.peek().copied() == Some('\'') {
                    out.push('\'');
                    let _ = chars.next();
                } else {
                    in_str = false;
                }
            }
            continue;
        }
        if ch == '\'' {
            in_str = true;
            out.push(ch);
            continue;
        }
        if ch == '#' {
            break;
        }
        out.push(ch);
    }
    out.trim().to_string()
}

fn split_lines(src: &str) -> Vec<String> {
    let mut out = Vec::<String>::new();
    let mut cur = String::new();
    let mut chars = src.chars().peekable();
    let mut in_str = false;

    while let Some(ch) = chars.next() {
        if in_str {
            cur.push(ch);
            if ch == '\'' {
                if chars.peek().copied() == Some('\'') {
                    cur.push('\'');
                    let _ = chars.next();
                } else {
                    in_str = false;
                }
            }
            continue;
        }

        if ch == '\'' {
            in_str = true;
            cur.push(ch);
            continue;
        }

        if ch == ';' || ch == '\n' || ch == '\r' {
            let line = strip_inline_comment(cur.trim());
            if !line.is_empty() {
                out.push(line);
            }
            cur.clear();
            continue;
        }

        cur.push(ch);
    }

    let line = strip_inline_comment(cur.trim());
    if !line.is_empty() {
        out.push(line);
    }
    out
}
"###,
    );
    out
}
