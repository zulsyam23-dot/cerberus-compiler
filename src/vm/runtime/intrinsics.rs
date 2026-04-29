pub(super) fn c_intrinsic_arity(name: &str) -> Option<u32> {
    match name {
        "c_open" => Some(1),
        "c_close" => Some(1),
        "c_symbol" => Some(2),
        "c_str_ptr" => Some(1),
        "c_call_i64_0" => Some(1),
        "c_call_i64_1" => Some(2),
        "c_call_i64_2" => Some(3),
        "c_call_i64_3" => Some(4),
        "c_call_i64_4" => Some(5),
        _ => None,
    }
}

pub(super) fn is_c_intrinsic(name: &str) -> bool {
    c_intrinsic_arity(name).is_some()
}
