# Cerberus C Interop (FFI) - Milestone Awal

Cerberus sekarang punya primitive FFI berbasis dynamic library untuk memanggil fungsi C dari program Cerberus.

## Builtin Baru

- `c_open(path: string): integer`
- `c_close(handle: integer): integer`
- `c_symbol(handle: integer, name: string): integer`
- `c_str_ptr(text: string): integer`
- `c_call_i64_0(symbol: integer): integer`
- `c_call_i64_1(symbol: integer, a0: integer): integer`
- `c_call_i64_2(symbol: integer, a0: integer, a1: integer): integer`
- `c_call_i64_3(symbol: integer, a0: integer, a1: integer, a2: integer): integer`
- `c_call_i64_4(symbol: integer, a0: integer, a1: integer, a2: integer, a3: integer): integer`

Semua nilai pointer/handle/symbol diperlakukan sebagai `integer`.

## Contoh: Memanggil `strlen`

```cerberus
program cffi_strlen;
var
  lib: integer;
  sym: integer;
  p: integer;
  n: integer;
begin
  lib := c_open('msvcrt.dll');      { Linux: libc.so.6, macOS: libSystem.B.dylib }
  sym := c_symbol(lib, 'strlen');
  p := c_str_ptr('cerberus');
  n := c_call_i64_1(sym, p);
  writeln(n);
  n := c_close(lib);
end.
```

## Catatan Penting

- ABI saat ini diasumsikan cocok dengan signature `extern "C" fn(...i64) -> i64`.
- Primitive ini adalah milestone awal untuk membuka akses ke ekosistem library C.
- Untuk API C yang lebih kompleks (struct, float, callback, varargs), tahap berikutnya perlu layer ABI/type bridge yang lebih kaya.
