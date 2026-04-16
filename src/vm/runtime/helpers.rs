use crate::error::CompileError;

use super::super::value::Value;

pub(super) fn ensure_vector_value(vec: &[Value], val: &Value) -> Result<(), CompileError> {
    match val {
        Value::Int(_) | Value::Bool(_) | Value::Str(_) => {}
        _ => {
            return Err(CompileError::new_simple(
                "vector element must be int/bool/string",
            ));
        }
    }
    if let Some(first) = vec.first() {
        if std::mem::discriminant(first) != std::mem::discriminant(val) {
            return Err(CompileError::new_simple("vector element type mismatch"));
        }
    }
    Ok(())
}
