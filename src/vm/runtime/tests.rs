use crate::bytecode::{Bytecode, Function, Instr};

use super::{Vm, VmConfig, VmLimits};

fn function(name: &str, locals: u32, code: Vec<Instr>) -> Function {
    Function {
        name: name.to_string(),
        param_count: 0,
        locals,
        code,
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
