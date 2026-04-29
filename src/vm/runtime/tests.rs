use crate::bytecode::{Bytecode, Function, Instr};

use super::super::value::Value;
use super::{Vm, VmConfig, VmLimits};

fn function(name: &str, locals: u32, code: Vec<Instr>) -> Function {
    Function {
        name: name.to_string(),
        param_count: 0,
        locals,
        code,
    }
}

fn intrinsic(name: &str, param_count: u32) -> Function {
    Function {
        name: name.to_string(),
        param_count,
        locals: param_count.max(1),
        code: vec![Instr::Halt],
    }
}

#[test]
fn rejects_invalid_jump_target() {
    let bc = Bytecode {
        name: "test".to_string(),
        entry: 0,
        functions: vec![function("main", 0, vec![Instr::Jump(99), Instr::Halt])],
    };

    let err = match Vm::new(&bc, Vec::new()) {
        Ok(_) => panic!("expected VM construction to fail for invalid jump"),
        Err(err) => err,
    };
    assert!(err.message.contains("jumps to"));
}

#[test]
fn enforces_execution_step_limit() {
    let bc = Bytecode {
        name: "test".to_string(),
        entry: 0,
        functions: vec![function("main", 0, vec![Instr::Jump(0)])],
    };

    let mut limits = VmLimits::default();
    limits.max_steps = 16;
    let config = VmConfig::with_limits(limits);

    let mut vm = Vm::with_config(&bc, Vec::new(), config).unwrap();
    let err = vm.run().unwrap_err();
    assert!(err.message.contains("execution step limit exceeded"));
}

#[test]
fn reports_division_by_zero_without_panicking() {
    let bc = Bytecode {
        name: "test".to_string(),
        entry: 0,
        functions: vec![function(
            "main",
            0,
            vec![
                Instr::ConstInt(1),
                Instr::ConstInt(0),
                Instr::Div,
                Instr::Halt,
            ],
        )],
    };

    let mut vm = Vm::new(&bc, Vec::new()).unwrap();
    let err = vm.run().unwrap_err();
    assert!(err.message.contains("division by zero"));
}

#[test]
fn enforces_call_depth_limit() {
    let bc = Bytecode {
        name: "test".to_string(),
        entry: 0,
        functions: vec![
            function("main", 0, vec![Instr::Call(1), Instr::Halt]),
            function("recur", 0, vec![Instr::Call(1), Instr::Ret]),
        ],
    };

    let mut limits = VmLimits::default();
    limits.max_call_depth = 4;
    let config = VmConfig::with_limits(limits);

    let mut vm = Vm::with_config(&bc, Vec::new(), config).unwrap();
    let err = vm.run().unwrap_err();
    assert!(err.message.contains("call depth exceeded"));
}

#[test]
fn c_intrinsic_can_call_zero_arg_c_symbol() {
    let (lib_name, symbol_name) = if cfg!(windows) {
        ("kernel32.dll", "GetCurrentProcessId")
    } else if cfg!(target_os = "macos") {
        ("libSystem.B.dylib", "getpid")
    } else {
        ("libc.so.6", "getpid")
    };

    let bc = Bytecode {
        name: "ffi".to_string(),
        entry: 0,
        functions: vec![
            function(
                "main",
                2,
                vec![
                    Instr::ConstStr(lib_name.to_string()),
                    Instr::Call(1),
                    Instr::Store(0),
                    Instr::Load(0),
                    Instr::ConstStr(symbol_name.to_string()),
                    Instr::Call(3),
                    Instr::Store(1),
                    Instr::Load(1),
                    Instr::Call(5),
                    Instr::Halt,
                ],
            ),
            intrinsic("c_open", 1),
            intrinsic("c_close", 1),
            intrinsic("c_symbol", 2),
            intrinsic("c_str_ptr", 1),
            intrinsic("c_call_i64_0", 1),
            intrinsic("c_call_i64_1", 2),
            intrinsic("c_call_i64_2", 3),
            intrinsic("c_call_i64_3", 4),
            intrinsic("c_call_i64_4", 5),
        ],
    };

    let mut vm = Vm::new(&bc, Vec::new()).unwrap();
    vm.run().unwrap();
    match vm.stack.last() {
        Some(Value::Int(v)) => assert!(*v > 0),
        _ => panic!("expected integer return value from c_call_i64_0"),
    }
}

#[test]
fn c_intrinsic_can_pass_c_string_pointer() {
    let lib_name = if cfg!(windows) {
        "msvcrt.dll"
    } else if cfg!(target_os = "macos") {
        "libSystem.B.dylib"
    } else {
        "libc.so.6"
    };

    let bc = Bytecode {
        name: "ffi".to_string(),
        entry: 0,
        functions: vec![
            function(
                "main",
                3,
                vec![
                    Instr::ConstStr(lib_name.to_string()),
                    Instr::Call(1),
                    Instr::Store(0),
                    Instr::Load(0),
                    Instr::ConstStr("strlen".to_string()),
                    Instr::Call(3),
                    Instr::Store(1),
                    Instr::ConstStr("cerberus".to_string()),
                    Instr::Call(4),
                    Instr::Store(2),
                    Instr::Load(1),
                    Instr::Load(2),
                    Instr::Call(6),
                    Instr::Halt,
                ],
            ),
            intrinsic("c_open", 1),
            intrinsic("c_close", 1),
            intrinsic("c_symbol", 2),
            intrinsic("c_str_ptr", 1),
            intrinsic("c_call_i64_0", 1),
            intrinsic("c_call_i64_1", 2),
            intrinsic("c_call_i64_2", 3),
            intrinsic("c_call_i64_3", 4),
            intrinsic("c_call_i64_4", 5),
        ],
    };

    let mut vm = Vm::new(&bc, Vec::new()).unwrap();
    vm.run().unwrap();
    match vm.stack.last() {
        Some(Value::Int(v)) => assert_eq!(*v, 8),
        _ => panic!("expected integer return value from c_call_i64_1"),
    }
}
