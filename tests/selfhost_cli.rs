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

fn temp_out(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before UNIX_EPOCH")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "cerberus_{name}_{}_{}.cerb",
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
