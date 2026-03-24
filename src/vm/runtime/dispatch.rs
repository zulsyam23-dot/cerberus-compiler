use crate::bytecode::Instr;
use crate::error::CompileError;

use super::{Step, Vm};

pub(super) fn dispatch(vm: &mut Vm) -> Result<Step, CompileError> {
    let code = &vm.functions[vm.current_func].code;
    if vm.ip >= code.len() {
        return Err(CompileError::new_simple("instruction pointer out of bounds"));
    }
    let ip = vm.ip;
    let instr = code[vm.ip].clone();
    let instr_dbg = instr.clone();
    let func_id = vm.current_func;
    let func_name = vm.functions[func_id].name.clone();
    vm.ip += 1;

    let res = match instr {
        Instr::ConstInt(_)
        | Instr::ConstBool(_)
        | Instr::ConstStr(_)
        | Instr::Load(_)
        | Instr::Store(_)
        | Instr::Add
        | Instr::Sub
        | Instr::Mul
        | Instr::Div
        | Instr::Eq
        | Instr::Ne
        | Instr::Lt
        | Instr::Le
        | Instr::Gt
        | Instr::Ge
        | Instr::And
        | Instr::Or
        | Instr::Not
        | Instr::Neg => {
            vm.exec_core(instr).map(|_| Step::Continue)
        }
        Instr::StrEq
        | Instr::StrNe
        | Instr::StrLen
        | Instr::StrConcat
        | Instr::StrSubstr
        | Instr::StrReplace
        | Instr::StrClear => {
            vm.exec_strings(instr).map(|_| Step::Continue)
        }
        Instr::AllocArray(_)
        | Instr::LoadIndex
        | Instr::StoreIndex
        | Instr::VecNew
        | Instr::VecLen
        | Instr::VecGet
        | Instr::VecSet
        | Instr::VecPush
        | Instr::VecRemove
        | Instr::VecLast
        | Instr::VecPop
        | Instr::StackNew
        | Instr::StackLen
        | Instr::StackPush
        | Instr::StackTop
        | Instr::StackPop
        | Instr::MapNew
        | Instr::MapLen
        | Instr::MapSet
        | Instr::MapGet
        | Instr::MapHas
        | Instr::MapRemove
        | Instr::SetNew
        | Instr::SetLen
        | Instr::SetAdd
        | Instr::SetHas
        | Instr::SetRemove
        | Instr::OptSomeInt
        | Instr::OptSomeBool
        | Instr::OptSomeStr
        | Instr::OptNoneInt
        | Instr::OptNoneBool
        | Instr::OptNoneStr
        | Instr::OptIsSomeInt
        | Instr::OptIsSomeBool
        | Instr::OptIsSomeStr
        | Instr::OptUnwrapInt
        | Instr::OptUnwrapBool
        | Instr::OptUnwrapStr
        | Instr::OptUnwrapOrInt
        | Instr::OptUnwrapOrBool
        | Instr::OptUnwrapOrStr
        | Instr::ResOkInt
        | Instr::ResOkBool
        | Instr::ResOkStr
        | Instr::ResErrInt
        | Instr::ResErrBool
        | Instr::ResErrStr
        | Instr::ResIsOkInt
        | Instr::ResIsOkBool
        | Instr::ResIsOkStr
        | Instr::ResUnwrapInt
        | Instr::ResUnwrapBool
        | Instr::ResUnwrapStr
        | Instr::ResUnwrapOrInt
        | Instr::ResUnwrapOrBool
        | Instr::ResUnwrapOrStr
        | Instr::ResUnwrapErrInt
        | Instr::ResUnwrapErrBool
        | Instr::ResUnwrapErrStr
        | Instr::VecClear
        | Instr::StackClear
        | Instr::MapClear
        | Instr::SetClear => {
            vm.exec_collections(instr).map(|_| Step::Continue)
        }
        Instr::ReadFile
        | Instr::WriteFile
        | Instr::ArgCount
        | Instr::Arg
        | Instr::PrintLn
        | Instr::ReadInt(_)
        | Instr::ReadBool(_)
        | Instr::ReadStr(_) => {
            vm.exec_io(instr).map(|_| Step::Continue)
        }
        Instr::EnvGet
        | Instr::EnvHas
        | Instr::Cwd
        | Instr::PathJoin
        | Instr::FsExists
        | Instr::FsListDir => {
            vm.exec_env_fs(instr).map(|_| Step::Continue)
        }
        Instr::NowTimestamp | Instr::SleepMs | Instr::LogStr | Instr::LogInt | Instr::LogBool => {
            vm.exec_time_log(instr).map(|_| Step::Continue)
        }
        Instr::BcNew
        | Instr::BcMain
        | Instr::BcEmitPrintStr
        | Instr::BcEmitHalt
        | Instr::BcWrite
        | Instr::BcEmitConstInt
        | Instr::BcEmitStore0
        | Instr::BcEmitLoad0
        | Instr::BcEmitPrintLn
        | Instr::BcEmitConstBool
        | Instr::BcLabel
        | Instr::BcJump
        | Instr::BcJumpIfFalse
        | Instr::BcEmitConstStr
        | Instr::BcEmitLoad
        | Instr::BcEmitStore
        | Instr::BcEmitAdd
        | Instr::BcEmitSub
        | Instr::BcEmitMul
        | Instr::BcEmitDiv
        | Instr::BcEmitEq
        | Instr::BcEmitNe
        | Instr::BcEmitLt
        | Instr::BcEmitLe
        | Instr::BcEmitGt
        | Instr::BcEmitGe
        | Instr::BcEmitAnd
        | Instr::BcEmitOr
        | Instr::BcEmitNot
        | Instr::BcEmitNeg
        | Instr::BcEmitStrLen
        | Instr::BcEmitStrConcat
        | Instr::BcEmitStrSubstr
        | Instr::BcEmitStrReplace
        | Instr::BcEmitVecNew
        | Instr::BcEmitVecLen
        | Instr::BcEmitVecGet
        | Instr::BcEmitVecSet
        | Instr::BcEmitVecPush
        | Instr::BcEmitVecRemove
        | Instr::BcEmitVecLast
        | Instr::BcEmitVecPop
        | Instr::BcEmitMapNew
        | Instr::BcEmitMapLen
        | Instr::BcEmitMapSet
        | Instr::BcEmitMapGet
        | Instr::BcEmitMapHas
        | Instr::BcEmitMapRemove
        | Instr::BcEmitReadFile
        | Instr::BcEmitWriteFile
        | Instr::BcEmitArgCount
        | Instr::BcEmitArg
        | Instr::BcEmitStrClear
        | Instr::BcEmitVecClear
        | Instr::BcEmitMapClear
        | Instr::BcFuncBegin
        | Instr::BcFuncEnd
        | Instr::BcEmitCall
        | Instr::BcEmitRet
        | Instr::BcEmitRetVal
        | Instr::BcEmitEnvGet
        | Instr::BcEmitEnvHas
        | Instr::BcEmitCwd
        | Instr::BcEmitPathJoin
        | Instr::BcEmitFsExists
        | Instr::BcEmitFsListDir
        | Instr::BcEmitNowTimestamp
        | Instr::BcEmitConstIntOp
        | Instr::BcEmitConstBoolOp
        | Instr::BcEmitConstStrOp
        | Instr::BcEmitLoadOp
        | Instr::BcEmitStoreOp
        | Instr::BcEmitCallOp
        | Instr::BcNewOp
        | Instr::BcWriteOp
        | Instr::BcLabelOp
        | Instr::BcJumpOp
        | Instr::BcJumpIfFalseOp
        | Instr::BcFuncBeginOp
        | Instr::BcMainOp
        | Instr::BcFuncEndOp
        | Instr::BcEmitHaltOp
        | Instr::BcEmitRetOp
        | Instr::BcEmitRetValOp
        | Instr::BcEmitPrintLnOp
        | Instr::BcEmitWriteFileOp
        | Instr::BcEmitAddOp
        | Instr::BcEmitSubOp
        | Instr::BcEmitMulOp
        | Instr::BcEmitDivOp
        | Instr::BcEmitEqOp
        | Instr::BcEmitNeOp
        | Instr::BcEmitLtOp
        | Instr::BcEmitLeOp
        | Instr::BcEmitGtOp
        | Instr::BcEmitGeOp
        | Instr::BcEmitAndOp
        | Instr::BcEmitOrOp
        | Instr::BcEmitNotOp
        | Instr::BcEmitNegOp
        | Instr::BcEmitStrLenOp
        | Instr::BcEmitStrConcatOp
        | Instr::BcEmitStrSubstrOp
        | Instr::BcEmitStrReplaceOp
        | Instr::BcEmitVecNewOp
        | Instr::BcEmitVecLenOp
        | Instr::BcEmitVecGetOp
        | Instr::BcEmitVecSetOp
        | Instr::BcEmitVecPushOp
        | Instr::BcEmitVecRemoveOp
        | Instr::BcEmitVecLastOp
        | Instr::BcEmitVecPopOp
        | Instr::BcEmitVecClearOp
        | Instr::BcEmitMapNewOp
        | Instr::BcEmitMapLenOp
        | Instr::BcEmitMapSetOp
        | Instr::BcEmitMapGetOp
        | Instr::BcEmitMapHasOp
        | Instr::BcEmitMapRemoveOp
        | Instr::BcEmitMapClearOp
        | Instr::BcEmitReadFileOp
        | Instr::BcEmitArgCountOp
        | Instr::BcEmitArgOp
        | Instr::BcEmitEnvGetOp
        | Instr::BcEmitEnvHasOp
        | Instr::BcEmitCwdOp
        | Instr::BcEmitPathJoinOp
        | Instr::BcEmitFsExistsOp
        | Instr::BcEmitFsListDirOp
        | Instr::BcEmitNowTimestampOp
        | Instr::MetaBcNew
        | Instr::MetaBcMain
        | Instr::MetaBcWrite
        | Instr::MetaBcFuncBegin
        | Instr::MetaBcFuncEnd
        | Instr::MetaBcEmitHalt
        | Instr::MetaBcLabel
        | Instr::MetaBcJump
        | Instr::MetaBcJumpIfFalse => {
            vm.exec_builder(instr).map(|_| Step::Continue)
        }
        Instr::Jump(_)
        | Instr::JumpIfFalse(_)
        | Instr::Call(_)
        | Instr::Ret
        | Instr::RetVal
        | Instr::Halt => vm.exec_control(instr),
    };

    res.map_err(|e| {
        CompileError::new_simple(format!(
            "{} (at {}#{} ip {} instr {:?})",
            e.message, func_name, func_id, ip, instr_dbg
        ))
    })
}
