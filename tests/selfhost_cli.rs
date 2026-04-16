use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn compiler_command(root: &PathBuf) -> Command {
    let mut cmd = Command::new("cargo");
    cmd.current_dir(root).arg("run").arg("-q").arg("--");
    cmd
}

fn command_exists(name: &str) -> bool {
    Command::new(name).arg("--version").output().is_ok()
        || Command::new(name).arg("-v").output().is_ok()
}

fn has_aot_asm_toolchain() -> bool {
    if !command_exists("nasm") {
        return false;
    }
    command_exists("cc") || command_exists("gcc") || command_exists("clang")
}

fn temp_out(name: &str) -> PathBuf {
    temp_file(name, "cerb")
}

fn temp_file(name: &str, ext: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before UNIX_EPOCH")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "cerberus_{name}_{}_{}.{}",
        std::process::id(), nanos, ext
    ))
}

fn temp_dir(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before UNIX_EPOCH")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "cerberus_{name}_{}_{}",
        std::process::id(),
        nanos
    ))
}

#[test]
fn default_compile_path_is_selfhosted() {
    let root = repo_root();
    let input = root.join("stdlib").join("main.cer");
    let output = temp_out("default_selfhost");

    let run = compiler_command(&root)
        .arg(&input)
        .arg(&output)
        .output()
        .expect("failed to run default compile path");

    assert!(
        run.status.success(),
        "default compile failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );

    let stdout = String::from_utf8_lossy(&run.stdout);
    assert!(
        stdout.contains("ok: wrote "),
        "expected selfhost compiler output marker\nstdout:\n{}",
        stdout
    );
    assert!(
        output.exists(),
        "missing compiled output: {}",
        output.display()
    );

    let _ = std::fs::remove_file(&output);
}

#[test]
fn selfhost_compiles_program() {
    let root = repo_root();
    let input = root.join("stdlib").join("main.cer");
    let output = temp_out("selfhost_compile");

    let run = compiler_command(&root)
        .arg("selfhost")
        .arg("--no-runtime-cache")
        .arg(&input)
        .arg(&output)
        .output()
        .expect("failed to run selfhost compile");

    assert!(
        run.status.success(),
        "selfhost compile failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
    assert!(
        output.exists(),
        "missing compiled output: {}",
        output.display()
    );

    let _ = std::fs::remove_file(&output);
}

#[test]
fn selfhost_limit_steps_is_enforced() {
    let root = repo_root();
    let input = root.join("stdlib").join("main.cer");
    let output = temp_out("selfhost_limit");

    let run = compiler_command(&root)
        .arg("selfhost")
        .arg("--no-runtime-cache")
        .arg("--limit-steps")
        .arg("1")
        .arg(&input)
        .arg(&output)
        .output()
        .expect("failed to run selfhost with step limit");

    assert!(
        !run.status.success(),
        "expected failure with tiny step limit\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
    let stderr = String::from_utf8_lossy(&run.stderr);
    assert!(
        stderr.contains("execution step limit exceeded"),
        "unexpected stderr:\n{}",
        stderr
    );

    let _ = std::fs::remove_file(&output);
}

#[test]
fn selfhost_run_mode_executes_output() {
    let root = repo_root();
    let input = root.join("stdlib").join("main.cer");
    let output = temp_out("selfhost_run");

    let run = compiler_command(&root)
        .arg("selfhost")
        .arg("--no-runtime-cache")
        .arg("--run")
        .arg(&input)
        .arg(&output)
        .output()
        .expect("failed to run selfhost --run");

    assert!(
        run.status.success(),
        "selfhost --run failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );

    let _ = std::fs::remove_file(&output);
}

#[test]
fn vm_package_input_runs_directly() {
    let root = repo_root();
    let pkg = temp_file("vm_package_direct", "crt");
    std::fs::write(
        &pkg,
        "cerberus_toolchain_v1;vm_text_script::code::const_int 7;println;halt;",
    )
    .expect("failed to write temporary package");

    let run = compiler_command(&root)
        .arg(&pkg)
        .output()
        .expect("failed to run packaged artifact directly");

    assert!(
        run.status.success(),
        "direct package run failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
    let stdout = String::from_utf8_lossy(&run.stdout);
    assert!(
        stdout.contains('7'),
        "expected package output marker\nstdout:\n{}",
        stdout
    );

    let _ = std::fs::remove_file(&pkg);
}

#[test]
fn aot_builds_and_runs_native_exe() {
    if Command::new("rustc").arg("--version").output().is_err() {
        return;
    }

    let root = repo_root();
    let vm_script = temp_file("aot_vm_script", "cer");
    let exe_ext = if cfg!(windows) { "exe" } else { "bin" };
    let native_out = temp_file("aot_native", exe_ext);

    std::fs::write(
        &vm_script,
        "@cerberus_vm 1; const_int 5; const_int 6; add; println; halt;",
    )
    .expect("failed to write temporary vm script");

    let build = compiler_command(&root)
        .arg("aot")
        .arg(&vm_script)
        .arg(&native_out)
        .output()
        .expect("failed to run aot build");

    assert!(
        build.status.success(),
        "aot build failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );
    assert!(native_out.exists(), "missing native exe: {}", native_out.display());

    let run = Command::new(&native_out)
        .output()
        .expect("failed to run generated native exe");

    assert!(
        run.status.success(),
        "generated native exe failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
    let stdout = String::from_utf8_lossy(&run.stdout);
    assert!(
        stdout.contains("11"),
        "unexpected native exe output\nstdout:\n{}",
        stdout
    );

    let _ = std::fs::remove_file(&vm_script);
    let _ = std::fs::remove_file(&native_out);
}

#[test]
fn aot_asm_backend_builds_and_runs_simple_program() {
    if !has_aot_asm_toolchain() {
        return;
    }

    let root = repo_root();
    let vm_script = temp_file("aot_vm_script_asm", "cer");
    let exe_ext = if cfg!(windows) { "exe" } else { "bin" };
    let native_out = temp_file("aot_native_asm", exe_ext);

    std::fs::write(&vm_script, "@cerberus_vm 1; const_int 42; println; halt;")
        .expect("failed to write temporary vm script");

    let build = compiler_command(&root)
        .arg("aot")
        .arg("--backend")
        .arg("asm")
        .arg(&vm_script)
        .arg(&native_out)
        .output()
        .expect("failed to run aot build (asm backend)");

    assert!(
        build.status.success(),
        "aot asm build failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );
    assert!(native_out.exists(), "missing native exe: {}", native_out.display());

    let run = Command::new(&native_out)
        .output()
        .expect("failed to run generated native exe (asm)");

    assert!(
        run.status.success(),
        "generated asm native exe failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
    let stdout = String::from_utf8_lossy(&run.stdout);
    assert!(
        stdout.contains("42"),
        "unexpected asm native output\nstdout:\n{}",
        stdout
    );

    let _ = std::fs::remove_file(&vm_script);
    let _ = std::fs::remove_file(&native_out);
}

#[test]
fn aot_supports_call_locals_and_string_ops() {
    if Command::new("rustc").arg("--version").output().is_err() {
        return;
    }

    let root = repo_root();
    let vm_script = temp_file("aot_vm_call_string", "cer");
    let exe_ext = if cfg!(windows) { "exe" } else { "bin" };
    let native_out = temp_file("aot_native_call_string", exe_ext);

    std::fs::write(
        &vm_script,
        "@cerberus_vm 1; jump main; label func; load 0; const_str ' world'; concat; ret_val; label main; locals 1; const_str 'hello'; store 0; call func; println; halt;",
    )
    .expect("failed to write temporary vm script");

    let build = compiler_command(&root)
        .arg("aot")
        .arg(&vm_script)
        .arg(&native_out)
        .output()
        .expect("failed to run aot build for call/string");

    assert!(
        build.status.success(),
        "aot build failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );
    assert!(native_out.exists(), "missing native exe: {}", native_out.display());

    let run = Command::new(&native_out)
        .output()
        .expect("failed to run generated native exe");

    assert!(
        run.status.success(),
        "generated native exe failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
    let stdout = String::from_utf8_lossy(&run.stdout);
    assert!(
        stdout.contains("hello world"),
        "unexpected native exe output\nstdout:\n{}",
        stdout
    );

    let _ = std::fs::remove_file(&vm_script);
    let _ = std::fs::remove_file(&native_out);
}

#[test]
fn aot_supports_vector_and_map_ops() {
    if Command::new("rustc").arg("--version").output().is_err() {
        return;
    }

    let root = repo_root();
    let vm_script = temp_file("aot_vm_collections", "cer");
    let exe_ext = if cfg!(windows) { "exe" } else { "bin" };
    let native_out = temp_file("aot_native_collections", exe_ext);

    std::fs::write(
        &vm_script,
        "@cerberus_vm 1; vec_new; const_int 1; vec_push; const_int 2; vec_push; const_int 1; vec_get; println; map_new; const_str 'k'; const_str 'v'; map_set; const_str 'k'; map_get; println; halt;",
    )
    .expect("failed to write temporary vm script");

    let build = compiler_command(&root)
        .arg("aot")
        .arg(&vm_script)
        .arg(&native_out)
        .output()
        .expect("failed to run aot build for collections");

    assert!(
        build.status.success(),
        "aot build failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );
    assert!(native_out.exists(), "missing native exe: {}", native_out.display());

    let run = Command::new(&native_out)
        .output()
        .expect("failed to run generated native exe");

    assert!(
        run.status.success(),
        "generated native exe failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
    let stdout = String::from_utf8_lossy(&run.stdout);
    assert!(
        stdout.contains("2") && stdout.contains("v"),
        "unexpected native exe output\nstdout:\n{}",
        stdout
    );

    let _ = std::fs::remove_file(&vm_script);
    let _ = std::fs::remove_file(&native_out);
}

#[test]
fn aot_rejects_bad_package_checksum() {
    if Command::new("rustc").arg("--version").output().is_err() {
        return;
    }

    let root = repo_root();
    let pkg = temp_file("aot_bad_checksum", "crt");
    let native_out = temp_file("aot_bad_checksum_out", if cfg!(windows) { "exe" } else { "bin" });

    std::fs::write(
        &pkg,
        "cerberus_toolchain_v1;vm_text_script;vm=1;abi=1;features=core,io,collections,fs,time,strings;artifact=vm_package;toolchain=cerberus_selfhost;checksum=123::code::halt;",
    )
    .expect("failed to write bad checksum package");

    let build = compiler_command(&root)
        .arg("aot")
        .arg(&pkg)
        .arg(&native_out)
        .output()
        .expect("failed to run aot on bad package");

    assert!(
        !build.status.success(),
        "aot build should fail for bad checksum\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );
    let stderr = String::from_utf8_lossy(&build.stderr);
    assert!(
        stderr.contains("checksum mismatch"),
        "unexpected aot error for bad checksum\nstderr:\n{}",
        stderr
    );

    let _ = std::fs::remove_file(&pkg);
    let _ = std::fs::remove_file(&native_out);
}

#[test]
fn pack_command_delegates_to_toolchain() {
    let root = repo_root();
    let vm_script = temp_file("pack_vm_script", "cer");
    let out_pkg = temp_file("pack_vm_script", "crt");

    std::fs::write(&vm_script, "@cerberus_vm 1; const_int 9; println; halt;")
        .expect("failed to write temporary vm script");

    let build = compiler_command(&root)
        .arg("pack")
        .arg(&vm_script)
        .arg(&out_pkg)
        .output()
        .expect("failed to run pack command");

    assert!(
        build.status.success(),
        "pack command failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );
    assert!(out_pkg.exists(), "missing package output: {}", out_pkg.display());

    let pkg_text = std::fs::read_to_string(&out_pkg).expect("failed to read package output");
    assert!(
        pkg_text.contains("cerberus_toolchain_v1") && pkg_text.contains("::code::"),
        "unexpected package contents:\n{}",
        pkg_text
    );

    let _ = std::fs::remove_file(&vm_script);
    let _ = std::fs::remove_file(&out_pkg);
}

#[test]
fn build_command_delegates_to_toolchain() {
    let root = repo_root();
    let vm_script = temp_file("build_vm_script", "cer");
    let target_dir = temp_dir("build_target");
    let app_name = "build_demo";

    std::fs::write(&vm_script, "@cerberus_vm 1; const_int 12; println; halt;")
        .expect("failed to write temporary vm script");

    let build = compiler_command(&root)
        .env("CERBERUS_TARGET_DIR", &target_dir)
        .arg("build")
        .arg(&vm_script)
        .arg(app_name)
        .output()
        .expect("failed to run build command");

    assert!(
        build.status.success(),
        "build command failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );

    let app_dir = target_dir.join("app").join(app_name);
    let crt = app_dir.join(format!("{app_name}.crt"));
    let cmd = app_dir.join(format!("{app_name}.cmd"));
    let exe = app_dir.join(format!("{app_name}.exe"));

    assert!(crt.exists(), "missing crt: {}", crt.display());
    assert!(cmd.exists(), "missing cmd: {}", cmd.display());
    assert!(
        !exe.exists(),
        "build command should not emit exe directly: {}",
        exe.display()
    );

    let _ = std::fs::remove_file(&vm_script);
    let _ = std::fs::remove_dir_all(&target_dir);
}

#[test]
fn bundle_outputs_crt_and_cmd() {
    let root = repo_root();
    let vm_script = temp_file("bundle_vm_script", "cer");
    let target_dir = temp_dir("bundle_target");
    let app_name = "bundle_demo";

    std::fs::write(&vm_script, "@cerberus_vm 1; const_int 13; println; halt;")
        .expect("failed to write temporary vm script");

    let build = compiler_command(&root)
        .arg("bundle")
        .arg(&vm_script)
        .arg(app_name)
        .arg("--target-dir")
        .arg(&target_dir)
        .output()
        .expect("failed to run bundle command");

    assert!(
        build.status.success(),
        "bundle build failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );

    let app_dir = target_dir.join("app").join(app_name);
    let crt = app_dir.join(format!("{app_name}.crt"));
    let cmd = app_dir.join(format!("{app_name}.cmd"));
    let exe = app_dir.join(format!("{app_name}.exe"));

    assert!(crt.exists(), "missing crt: {}", crt.display());
    assert!(cmd.exists(), "missing cmd: {}", cmd.display());
    assert!(
        !exe.exists(),
        "bundle command should not emit exe directly: {}",
        exe.display()
    );

    let _ = std::fs::remove_file(&vm_script);
    let _ = std::fs::remove_dir_all(&target_dir);
}

#[test]
fn aot_selfhost_emits_build_script() {
    let root = repo_root();
    let vm_script = temp_file("aot_selfhost_vm_script", "cer");
    let out_asm = temp_file("aot_selfhost_out", "asm");
    let out_bin = temp_file(
        "aot_selfhost_bin",
        if cfg!(windows) { "exe" } else { "bin" },
    );
    let script_path = temp_file(
        "aot_selfhost_script",
        if cfg!(windows) { "cmd" } else { "sh" },
    );
    let platform = if cfg!(windows) { "win64" } else { "linux64" };

    std::fs::write(&vm_script, "@cerberus_vm 1; const_int 77; println; halt;")
        .expect("failed to write temporary vm script");

    let run = compiler_command(&root)
        .arg("aot-selfhost")
        .arg(&vm_script)
        .arg(&out_asm)
        .arg("--platform")
        .arg(platform)
        .arg("--out-bin")
        .arg(&out_bin)
        .arg("--script")
        .arg(&script_path)
        .output()
        .expect("failed to run aot-selfhost");

    assert!(
        run.status.success(),
        "aot-selfhost failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
    assert!(
        script_path.exists(),
        "missing selfhost build script: {}",
        script_path.display()
    );

    let script = std::fs::read_to_string(&script_path).expect("failed to read selfhost build script");
    if cfg!(windows) {
        assert!(
            script.contains("nasm -f win64") && script.contains("gcc "),
            "unexpected windows selfhost build script:\n{}",
            script
        );
    } else {
        assert!(
            script.contains("nasm -f elf64") && script.contains("cc "),
            "unexpected linux selfhost build script:\n{}",
            script
        );
    }
    assert!(
        script.contains(out_asm.to_string_lossy().as_ref())
            && script.contains(out_bin.to_string_lossy().as_ref()),
        "selfhost build script does not reference output paths\nscript:\n{}",
        script
    );

    let _ = std::fs::remove_file(&vm_script);
    let _ = std::fs::remove_file(&out_asm);
    let _ = std::fs::remove_file(&out_bin);
    let _ = std::fs::remove_file(&script_path);
}

#[test]
fn aot_selfhost_no_script_skips_script_output() {
    let root = repo_root();
    let vm_script = temp_file("aot_selfhost_no_script_vm", "cer");
    let out_asm = temp_file("aot_selfhost_no_script_out", "asm");
    let out_bin = temp_file(
        "aot_selfhost_no_script_bin",
        if cfg!(windows) { "exe" } else { "bin" },
    );
    let script_path = temp_file(
        "aot_selfhost_no_script_script",
        if cfg!(windows) { "cmd" } else { "sh" },
    );
    let platform = if cfg!(windows) { "win64" } else { "linux64" };

    std::fs::write(&vm_script, "@cerberus_vm 1; const_int 77; println; halt;")
        .expect("failed to write temporary vm script");

    let run = compiler_command(&root)
        .arg("aot-selfhost")
        .arg(&vm_script)
        .arg(&out_asm)
        .arg("--platform")
        .arg(platform)
        .arg("--out-bin")
        .arg(&out_bin)
        .arg("--script")
        .arg(&script_path)
        .arg("--no-script")
        .output()
        .expect("failed to run aot-selfhost --no-script");

    assert!(
        run.status.success(),
        "aot-selfhost --no-script failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
    assert!(
        !script_path.exists(),
        "script should not exist in --no-script mode: {}",
        script_path.display()
    );

    let _ = std::fs::remove_file(&vm_script);
    let _ = std::fs::remove_file(&out_asm);
    let _ = std::fs::remove_file(&out_bin);
    let _ = std::fs::remove_file(&script_path);
}
