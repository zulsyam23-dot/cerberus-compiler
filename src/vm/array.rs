use crate::error::CompileError;

pub fn set_index(mut arr: Vec<i64>, idx: usize, val: i64) -> Result<Vec<i64>, CompileError> {
    if idx >= arr.len() {
        return Err(CompileError::new_simple("array index out of bounds"));
    }
    arr[idx] = val;
    Ok(arr)
}
