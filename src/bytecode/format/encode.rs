use crate::bytecode::{Function, Instr, OpCode};

use super::util::{write_i64, write_str, write_u32};

pub(super) fn write_function(buf: &mut Vec<u8>, func: &Function) {
    write_str(buf, &func.name);
    write_u32(buf, func.param_count);
    write_u32(buf, func.locals);
    write_u32(buf, func.code.len() as u32);
    for instr in &func.code {
        encode_instr(buf, instr);
    }
}

fn encode_instr(buf: &mut Vec<u8>, instr: &Instr) {
    match instr {
        Instr::ConstInt(v) => {
            buf.push(OpCode::ConstInt as u8);
            write_i64(buf, *v);
        }
        Instr::ConstBool(v) => {
            buf.push(OpCode::ConstBool as u8);
            buf.push(if *v { 1 } else { 0 });
        }
        Instr::ConstStr(s) => {
            buf.push(OpCode::ConstStr as u8);
            write_str(buf, s);
        }
        Instr::Load(i) => {
            buf.push(OpCode::Load as u8);
            write_u32(buf, *i);
        }
        Instr::Store(i) => {
            buf.push(OpCode::Store as u8);
            write_u32(buf, *i);
        }
        Instr::Add => buf.push(OpCode::Add as u8),
        Instr::Sub => buf.push(OpCode::Sub as u8),
        Instr::Mul => buf.push(OpCode::Mul as u8),
        Instr::Div => buf.push(OpCode::Div as u8),
        Instr::Eq => buf.push(OpCode::Eq as u8),
        Instr::Ne => buf.push(OpCode::Ne as u8),
        Instr::Lt => buf.push(OpCode::Lt as u8),
        Instr::Le => buf.push(OpCode::Le as u8),
        Instr::Gt => buf.push(OpCode::Gt as u8),
        Instr::Ge => buf.push(OpCode::Ge as u8),
        Instr::StrEq => buf.push(OpCode::StrEq as u8),
        Instr::StrNe => buf.push(OpCode::StrNe as u8),
        Instr::And => buf.push(OpCode::And as u8),
        Instr::Or => buf.push(OpCode::Or as u8),
        Instr::Not => buf.push(OpCode::Not as u8),
        Instr::Neg => buf.push(OpCode::Neg as u8),
        Instr::Jump(addr) => {
            buf.push(OpCode::Jump as u8);
            write_u32(buf, *addr);
        }
        Instr::JumpIfFalse(addr) => {
            buf.push(OpCode::JumpIfFalse as u8);
            write_u32(buf, *addr);
        }
        Instr::PrintLn => buf.push(OpCode::PrintLn as u8),
        Instr::ReadInt(i) => {
            buf.push(OpCode::ReadInt as u8);
            write_u32(buf, *i);
        }
        Instr::ReadBool(i) => {
            buf.push(OpCode::ReadBool as u8);
            write_u32(buf, *i);
        }
        Instr::ReadStr(i) => {
            buf.push(OpCode::ReadStr as u8);
            write_u32(buf, *i);
        }
        Instr::Call(i) => {
            buf.push(OpCode::Call as u8);
            write_u32(buf, *i);
        }
        Instr::Ret => buf.push(OpCode::Ret as u8),
        Instr::RetVal => buf.push(OpCode::RetVal as u8),
        Instr::AllocArray(len) => {
            buf.push(OpCode::AllocArray as u8);
            write_u32(buf, *len);
        }
        Instr::LoadIndex => buf.push(OpCode::LoadIndex as u8),
        Instr::StoreIndex => buf.push(OpCode::StoreIndex as u8),
        Instr::ReadFile => buf.push(OpCode::ReadFile as u8),
        Instr::WriteFile => buf.push(OpCode::WriteFile as u8),
        Instr::ArgCount => buf.push(OpCode::ArgCount as u8),
        Instr::Arg => buf.push(OpCode::Arg as u8),
        Instr::StrLen => buf.push(OpCode::StrLen as u8),
        Instr::StrConcat => buf.push(OpCode::StrConcat as u8),
        Instr::StrSubstr => buf.push(OpCode::StrSubstr as u8),
        Instr::StrReplace => buf.push(OpCode::StrReplace as u8),
        Instr::VecNew => buf.push(OpCode::VecNew as u8),
        Instr::VecLen => buf.push(OpCode::VecLen as u8),
        Instr::VecGet => buf.push(OpCode::VecGet as u8),
        Instr::VecSet => buf.push(OpCode::VecSet as u8),
        Instr::VecPush => buf.push(OpCode::VecPush as u8),
        Instr::VecRemove => buf.push(OpCode::VecRemove as u8),
        Instr::VecLast => buf.push(OpCode::VecLast as u8),
        Instr::VecPop => buf.push(OpCode::VecPop as u8),
        Instr::StackNew => buf.push(OpCode::StackNew as u8),
        Instr::StackLen => buf.push(OpCode::StackLen as u8),
        Instr::StackPush => buf.push(OpCode::StackPush as u8),
        Instr::StackTop => buf.push(OpCode::StackTop as u8),
        Instr::StackPop => buf.push(OpCode::StackPop as u8),
        Instr::MapNew => buf.push(OpCode::MapNew as u8),
        Instr::MapLen => buf.push(OpCode::MapLen as u8),
        Instr::MapSet => buf.push(OpCode::MapSet as u8),
        Instr::MapGet => buf.push(OpCode::MapGet as u8),
        Instr::MapHas => buf.push(OpCode::MapHas as u8),
        Instr::MapRemove => buf.push(OpCode::MapRemove as u8),
        Instr::SetNew => buf.push(OpCode::SetNew as u8),
        Instr::SetLen => buf.push(OpCode::SetLen as u8),
        Instr::SetAdd => buf.push(OpCode::SetAdd as u8),
        Instr::SetHas => buf.push(OpCode::SetHas as u8),
        Instr::SetRemove => buf.push(OpCode::SetRemove as u8),
        Instr::OptSomeInt => buf.push(OpCode::OptSomeInt as u8),
        Instr::OptSomeBool => buf.push(OpCode::OptSomeBool as u8),
        Instr::OptSomeStr => buf.push(OpCode::OptSomeStr as u8),
        Instr::OptNoneInt => buf.push(OpCode::OptNoneInt as u8),
        Instr::OptNoneBool => buf.push(OpCode::OptNoneBool as u8),
        Instr::OptNoneStr => buf.push(OpCode::OptNoneStr as u8),
        Instr::OptIsSomeInt => buf.push(OpCode::OptIsSomeInt as u8),
        Instr::OptIsSomeBool => buf.push(OpCode::OptIsSomeBool as u8),
        Instr::OptIsSomeStr => buf.push(OpCode::OptIsSomeStr as u8),
        Instr::OptUnwrapInt => buf.push(OpCode::OptUnwrapInt as u8),
        Instr::OptUnwrapBool => buf.push(OpCode::OptUnwrapBool as u8),
        Instr::OptUnwrapStr => buf.push(OpCode::OptUnwrapStr as u8),
        Instr::OptUnwrapOrInt => buf.push(OpCode::OptUnwrapOrInt as u8),
        Instr::OptUnwrapOrBool => buf.push(OpCode::OptUnwrapOrBool as u8),
        Instr::OptUnwrapOrStr => buf.push(OpCode::OptUnwrapOrStr as u8),
        Instr::ResOkInt => buf.push(OpCode::ResOkInt as u8),
        Instr::ResOkBool => buf.push(OpCode::ResOkBool as u8),
        Instr::ResOkStr => buf.push(OpCode::ResOkStr as u8),
        Instr::ResErrInt => buf.push(OpCode::ResErrInt as u8),
        Instr::ResErrBool => buf.push(OpCode::ResErrBool as u8),
        Instr::ResErrStr => buf.push(OpCode::ResErrStr as u8),
        Instr::ResIsOkInt => buf.push(OpCode::ResIsOkInt as u8),
        Instr::ResIsOkBool => buf.push(OpCode::ResIsOkBool as u8),
        Instr::ResIsOkStr => buf.push(OpCode::ResIsOkStr as u8),
        Instr::ResUnwrapInt => buf.push(OpCode::ResUnwrapInt as u8),
        Instr::ResUnwrapBool => buf.push(OpCode::ResUnwrapBool as u8),
        Instr::ResUnwrapStr => buf.push(OpCode::ResUnwrapStr as u8),
        Instr::ResUnwrapOrInt => buf.push(OpCode::ResUnwrapOrInt as u8),
        Instr::ResUnwrapOrBool => buf.push(OpCode::ResUnwrapOrBool as u8),
        Instr::ResUnwrapOrStr => buf.push(OpCode::ResUnwrapOrStr as u8),
        Instr::ResUnwrapErrInt => buf.push(OpCode::ResUnwrapErrInt as u8),
        Instr::ResUnwrapErrBool => buf.push(OpCode::ResUnwrapErrBool as u8),
        Instr::ResUnwrapErrStr => buf.push(OpCode::ResUnwrapErrStr as u8),
        Instr::StrClear => buf.push(OpCode::StrClear as u8),
        Instr::VecClear => buf.push(OpCode::VecClear as u8),
        Instr::StackClear => buf.push(OpCode::StackClear as u8),
        Instr::MapClear => buf.push(OpCode::MapClear as u8),
        Instr::SetClear => buf.push(OpCode::SetClear as u8),
        Instr::EnvGet => buf.push(OpCode::EnvGet as u8),
        Instr::EnvHas => buf.push(OpCode::EnvHas as u8),
        Instr::Cwd => buf.push(OpCode::Cwd as u8),
        Instr::PathJoin => buf.push(OpCode::PathJoin as u8),
        Instr::FsExists => buf.push(OpCode::FsExists as u8),
        Instr::FsListDir => buf.push(OpCode::FsListDir as u8),
        Instr::NowTimestamp => buf.push(OpCode::NowTimestamp as u8),
        Instr::SleepMs => buf.push(OpCode::SleepMs as u8),
        Instr::LogStr => buf.push(OpCode::LogStr as u8),
        Instr::LogInt => buf.push(OpCode::LogInt as u8),
        Instr::LogBool => buf.push(OpCode::LogBool as u8),
        Instr::BcNew => buf.push(OpCode::BcNew as u8),
        Instr::BcMain => buf.push(OpCode::BcMain as u8),
        Instr::BcEmitPrintStr => buf.push(OpCode::BcEmitPrintStr as u8),
        Instr::BcEmitHalt => buf.push(OpCode::BcEmitHalt as u8),
        Instr::BcWrite => buf.push(OpCode::BcWrite as u8),
        Instr::BcEmitConstInt => buf.push(OpCode::BcEmitConstInt as u8),
        Instr::BcEmitStore0 => buf.push(OpCode::BcEmitStore0 as u8),
        Instr::BcEmitLoad0 => buf.push(OpCode::BcEmitLoad0 as u8),
        Instr::BcEmitPrintLn => buf.push(OpCode::BcEmitPrintLn as u8),
        Instr::BcEmitConstBool => buf.push(OpCode::BcEmitConstBool as u8),
        Instr::BcLabel => buf.push(OpCode::BcLabel as u8),
        Instr::BcJump => buf.push(OpCode::BcJump as u8),
        Instr::BcJumpIfFalse => buf.push(OpCode::BcJumpIfFalse as u8),
        Instr::BcEmitConstStr => buf.push(OpCode::BcEmitConstStr as u8),
        Instr::BcEmitLoad => buf.push(OpCode::BcEmitLoad as u8),
        Instr::BcEmitStore => buf.push(OpCode::BcEmitStore as u8),
        Instr::BcEmitAdd => buf.push(OpCode::BcEmitAdd as u8),
        Instr::BcEmitSub => buf.push(OpCode::BcEmitSub as u8),
        Instr::BcEmitMul => buf.push(OpCode::BcEmitMul as u8),
        Instr::BcEmitDiv => buf.push(OpCode::BcEmitDiv as u8),
        Instr::BcEmitEq => buf.push(OpCode::BcEmitEq as u8),
        Instr::BcEmitNe => buf.push(OpCode::BcEmitNe as u8),
        Instr::BcEmitLt => buf.push(OpCode::BcEmitLt as u8),
        Instr::BcEmitLe => buf.push(OpCode::BcEmitLe as u8),
        Instr::BcEmitGt => buf.push(OpCode::BcEmitGt as u8),
        Instr::BcEmitGe => buf.push(OpCode::BcEmitGe as u8),
        Instr::BcEmitAnd => buf.push(OpCode::BcEmitAnd as u8),
        Instr::BcEmitOr => buf.push(OpCode::BcEmitOr as u8),
        Instr::BcEmitNot => buf.push(OpCode::BcEmitNot as u8),
        Instr::BcEmitNeg => buf.push(OpCode::BcEmitNeg as u8),
        Instr::BcEmitStrLen => buf.push(OpCode::BcEmitStrLen as u8),
        Instr::BcEmitStrConcat => buf.push(OpCode::BcEmitStrConcat as u8),
        Instr::BcEmitStrSubstr => buf.push(OpCode::BcEmitStrSubstr as u8),
        Instr::BcEmitStrReplace => buf.push(OpCode::BcEmitStrReplace as u8),
        Instr::BcEmitVecNew => buf.push(OpCode::BcEmitVecNew as u8),
        Instr::BcEmitVecLen => buf.push(OpCode::BcEmitVecLen as u8),
        Instr::BcEmitVecGet => buf.push(OpCode::BcEmitVecGet as u8),
        Instr::BcEmitVecSet => buf.push(OpCode::BcEmitVecSet as u8),
        Instr::BcEmitVecPush => buf.push(OpCode::BcEmitVecPush as u8),
        Instr::BcEmitVecRemove => buf.push(OpCode::BcEmitVecRemove as u8),
        Instr::BcEmitVecLast => buf.push(OpCode::BcEmitVecLast as u8),
        Instr::BcEmitVecPop => buf.push(OpCode::BcEmitVecPop as u8),
        Instr::BcEmitMapNew => buf.push(OpCode::BcEmitMapNew as u8),
        Instr::BcEmitMapLen => buf.push(OpCode::BcEmitMapLen as u8),
        Instr::BcEmitMapSet => buf.push(OpCode::BcEmitMapSet as u8),
        Instr::BcEmitMapGet => buf.push(OpCode::BcEmitMapGet as u8),
        Instr::BcEmitMapHas => buf.push(OpCode::BcEmitMapHas as u8),
        Instr::BcEmitMapRemove => buf.push(OpCode::BcEmitMapRemove as u8),
        Instr::BcEmitReadFile => buf.push(OpCode::BcEmitReadFile as u8),
        Instr::BcEmitWriteFile => buf.push(OpCode::BcEmitWriteFile as u8),
        Instr::BcEmitArgCount => buf.push(OpCode::BcEmitArgCount as u8),
        Instr::BcEmitArg => buf.push(OpCode::BcEmitArg as u8),
        Instr::BcEmitStrClear => buf.push(OpCode::BcEmitStrClear as u8),
        Instr::BcEmitVecClear => buf.push(OpCode::BcEmitVecClear as u8),
        Instr::BcEmitMapClear => buf.push(OpCode::BcEmitMapClear as u8),
        Instr::BcFuncBegin => buf.push(OpCode::BcFuncBegin as u8),
        Instr::BcFuncEnd => buf.push(OpCode::BcFuncEnd as u8),
        Instr::BcEmitCall => buf.push(OpCode::BcEmitCall as u8),
        Instr::BcEmitRet => buf.push(OpCode::BcEmitRet as u8),
        Instr::BcEmitRetVal => buf.push(OpCode::BcEmitRetVal as u8),
        Instr::BcEmitEnvGet => buf.push(OpCode::BcEmitEnvGet as u8),
        Instr::BcEmitEnvHas => buf.push(OpCode::BcEmitEnvHas as u8),
        Instr::BcEmitCwd => buf.push(OpCode::BcEmitCwd as u8),
        Instr::BcEmitPathJoin => buf.push(OpCode::BcEmitPathJoin as u8),
        Instr::BcEmitFsExists => buf.push(OpCode::BcEmitFsExists as u8),
        Instr::BcEmitFsListDir => buf.push(OpCode::BcEmitFsListDir as u8),
        Instr::BcEmitNowTimestamp => buf.push(OpCode::BcEmitNowTimestamp as u8),
        Instr::BcEmitConstIntOp => buf.push(OpCode::BcEmitConstIntOp as u8),
        Instr::BcEmitConstBoolOp => buf.push(OpCode::BcEmitConstBoolOp as u8),
        Instr::BcEmitConstStrOp => buf.push(OpCode::BcEmitConstStrOp as u8),
        Instr::BcEmitLoadOp => buf.push(OpCode::BcEmitLoadOp as u8),
        Instr::BcEmitStoreOp => buf.push(OpCode::BcEmitStoreOp as u8),
        Instr::BcEmitCallOp => buf.push(OpCode::BcEmitCallOp as u8),
        Instr::BcNewOp => buf.push(OpCode::BcNewOp as u8),
        Instr::BcWriteOp => buf.push(OpCode::BcWriteOp as u8),
        Instr::BcLabelOp => buf.push(OpCode::BcLabelOp as u8),
        Instr::BcJumpOp => buf.push(OpCode::BcJumpOp as u8),
        Instr::BcJumpIfFalseOp => buf.push(OpCode::BcJumpIfFalseOp as u8),
        Instr::BcFuncBeginOp => buf.push(OpCode::BcFuncBeginOp as u8),
        Instr::BcMainOp => buf.push(OpCode::BcMainOp as u8),
        Instr::BcFuncEndOp => buf.push(OpCode::BcFuncEndOp as u8),
        Instr::BcEmitHaltOp => buf.push(OpCode::BcEmitHaltOp as u8),
        Instr::BcEmitRetOp => buf.push(OpCode::BcEmitRetOp as u8),
        Instr::BcEmitRetValOp => buf.push(OpCode::BcEmitRetValOp as u8),
        Instr::BcEmitPrintLnOp => buf.push(OpCode::BcEmitPrintLnOp as u8),
        Instr::BcEmitWriteFileOp => buf.push(OpCode::BcEmitWriteFileOp as u8),
        Instr::BcEmitAddOp => buf.push(OpCode::BcEmitAddOp as u8),
        Instr::BcEmitSubOp => buf.push(OpCode::BcEmitSubOp as u8),
        Instr::BcEmitMulOp => buf.push(OpCode::BcEmitMulOp as u8),
        Instr::BcEmitDivOp => buf.push(OpCode::BcEmitDivOp as u8),
        Instr::BcEmitEqOp => buf.push(OpCode::BcEmitEqOp as u8),
        Instr::BcEmitNeOp => buf.push(OpCode::BcEmitNeOp as u8),
        Instr::BcEmitLtOp => buf.push(OpCode::BcEmitLtOp as u8),
        Instr::BcEmitLeOp => buf.push(OpCode::BcEmitLeOp as u8),
        Instr::BcEmitGtOp => buf.push(OpCode::BcEmitGtOp as u8),
        Instr::BcEmitGeOp => buf.push(OpCode::BcEmitGeOp as u8),
        Instr::BcEmitAndOp => buf.push(OpCode::BcEmitAndOp as u8),
        Instr::BcEmitOrOp => buf.push(OpCode::BcEmitOrOp as u8),
        Instr::BcEmitNotOp => buf.push(OpCode::BcEmitNotOp as u8),
        Instr::BcEmitNegOp => buf.push(OpCode::BcEmitNegOp as u8),
        Instr::BcEmitStrLenOp => buf.push(OpCode::BcEmitStrLenOp as u8),
        Instr::BcEmitStrConcatOp => buf.push(OpCode::BcEmitStrConcatOp as u8),
        Instr::BcEmitStrSubstrOp => buf.push(OpCode::BcEmitStrSubstrOp as u8),
        Instr::BcEmitStrReplaceOp => buf.push(OpCode::BcEmitStrReplaceOp as u8),
        Instr::BcEmitVecNewOp => buf.push(OpCode::BcEmitVecNewOp as u8),
        Instr::BcEmitVecLenOp => buf.push(OpCode::BcEmitVecLenOp as u8),
        Instr::BcEmitVecGetOp => buf.push(OpCode::BcEmitVecGetOp as u8),
        Instr::BcEmitVecSetOp => buf.push(OpCode::BcEmitVecSetOp as u8),
        Instr::BcEmitVecPushOp => buf.push(OpCode::BcEmitVecPushOp as u8),
        Instr::BcEmitVecRemoveOp => buf.push(OpCode::BcEmitVecRemoveOp as u8),
        Instr::BcEmitVecLastOp => buf.push(OpCode::BcEmitVecLastOp as u8),
        Instr::BcEmitVecPopOp => buf.push(OpCode::BcEmitVecPopOp as u8),
        Instr::BcEmitVecClearOp => buf.push(OpCode::BcEmitVecClearOp as u8),
        Instr::BcEmitMapNewOp => buf.push(OpCode::BcEmitMapNewOp as u8),
        Instr::BcEmitMapLenOp => buf.push(OpCode::BcEmitMapLenOp as u8),
        Instr::BcEmitMapSetOp => buf.push(OpCode::BcEmitMapSetOp as u8),
        Instr::BcEmitMapGetOp => buf.push(OpCode::BcEmitMapGetOp as u8),
        Instr::BcEmitMapHasOp => buf.push(OpCode::BcEmitMapHasOp as u8),
        Instr::BcEmitMapRemoveOp => buf.push(OpCode::BcEmitMapRemoveOp as u8),
        Instr::BcEmitMapClearOp => buf.push(OpCode::BcEmitMapClearOp as u8),
        Instr::BcEmitReadFileOp => buf.push(OpCode::BcEmitReadFileOp as u8),
        Instr::BcEmitArgCountOp => buf.push(OpCode::BcEmitArgCountOp as u8),
        Instr::BcEmitArgOp => buf.push(OpCode::BcEmitArgOp as u8),
        Instr::BcEmitEnvGetOp => buf.push(OpCode::BcEmitEnvGetOp as u8),
        Instr::BcEmitEnvHasOp => buf.push(OpCode::BcEmitEnvHasOp as u8),
        Instr::BcEmitCwdOp => buf.push(OpCode::BcEmitCwdOp as u8),
        Instr::BcEmitPathJoinOp => buf.push(OpCode::BcEmitPathJoinOp as u8),
        Instr::BcEmitFsExistsOp => buf.push(OpCode::BcEmitFsExistsOp as u8),
        Instr::BcEmitFsListDirOp => buf.push(OpCode::BcEmitFsListDirOp as u8),
        Instr::BcEmitNowTimestampOp => buf.push(OpCode::BcEmitNowTimestampOp as u8),
        Instr::MetaBcNew => buf.push(OpCode::MetaBcNew as u8),
        Instr::MetaBcMain => buf.push(OpCode::MetaBcMain as u8),
        Instr::MetaBcWrite => buf.push(OpCode::MetaBcWrite as u8),
        Instr::MetaBcFuncBegin => buf.push(OpCode::MetaBcFuncBegin as u8),
        Instr::MetaBcFuncEnd => buf.push(OpCode::MetaBcFuncEnd as u8),
        Instr::MetaBcEmitHalt => buf.push(OpCode::MetaBcEmitHalt as u8),
        Instr::MetaBcLabel => buf.push(OpCode::MetaBcLabel as u8),
        Instr::MetaBcJump => buf.push(OpCode::MetaBcJump as u8),
        Instr::MetaBcJumpIfFalse => buf.push(OpCode::MetaBcJumpIfFalse as u8),
        Instr::Halt => buf.push(OpCode::Halt as u8),
    }
}
