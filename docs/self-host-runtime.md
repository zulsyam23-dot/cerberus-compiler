# Cerberus Self-Hosted Runtime (Stage 0)

Dokumen ini menjelaskan runtime Cerberus-native awal yang ditulis dalam bahasa Cerberus, sebagai langkah menuju runtime yang benar-benar lepas dari Rust.

## File Utama

- `stdlib/runtime/runtime.cer`
  - entrypoint runtime
  - menerima argumen path script VM
- `stdlib/runtime/vm/runtime_vm.cer`
  - facade module untuk VM runtime
- `stdlib/runtime/vm/runtime_vm_text.cer`
  - utility parsing/lexing script opcode teks
- `stdlib/runtime/vm/runtime_vm_value.cer`
  - value envelope typed `v1|kind|len|payload` (kompatibel legacy `i:`, `b:`, `s:`)
- `stdlib/runtime/vm/runtime_vm_validate.cer`
  - validasi statis directive/opcode/arg/target sebelum eksekusi
- `stdlib/runtime/vm/runtime/runtime_vm_exec.cer`
  - interpreter opcode tekstual dan loop eksekusi VM
- `stdlib/runtime/vm/runtime_io.cer`
  - wrapper I/O host (`readfile`, `writefile`, `fs_exists`, `cwd`)

## Format Script VM

Untuk tahap ini, script ditulis sebagai opcode teks dengan separator:

- newline (direkomendasikan)
- atau `;` (kompatibilitas)

Format profesional (disarankan):

- `@cerberus_vm 1;`
- `@entry <label|ip>;`
- `@limit_steps <n>;` (opsional)
- `@limit_stack <n>;` (opsional)
- `@limit_call <n>;` (opsional)
- opcode runtime (`label`, `const_int`, dst)

Contoh:

`@cerberus_vm 1
@entry main
label main
locals 2
const_int 40
store 0
const_int 2
store 1
load 0
load 1
add
println
halt`

Catatan:

- Directive wajib berada di bagian awal file (sebelum opcode pertama).
- Runtime lama tanpa directive tetap didukung untuk kompatibilitas.
- Inline comment didukung dengan prefix `#`.
- String literal aman berisi `;` tanpa memecah instruksi.

## Opcode yang Tersedia (Stage 0)

- Core: `nop`, `halt`, `locals`, `const_int`, `const_bool`, `const_str`, `load`, `store`
- Arithmetic: `add`, `sub`, `mul`, `div`, `neg`
- Compare/logic: `eq`, `ne`, `lt`, `le`, `gt`, `ge`, `and`, `or`, `not`
- Control: `label`, `jump`, `jump_if_false`, `call`, `ret`, `ret_val`
- I/O: `println`, `readfile`, `writefile`
- Runtime directives: `limit_steps`, `limit_stack`, `limit_call`
- Preflight validation: struktur script, arg opcode, duplicate label, unresolved target

## Cara Menjalankan

1. Compile runtime Cerberus:
   - `cargo run -- stdlib/runtime/runtime.cer out_runtime.cerb`
2. Jalankan runtime dengan script:
   - `cargo run -- run out_runtime.cerb path/to/script_vm.cer`

Catatan:
- Folder `examples/` sudah dihapus karena tidak dipakai build/test.
- Gunakan script VM milik proyek kamu sendiri (`path/to/script_vm.cer`).

## Catatan Stage 0

- Ini sudah runtime yang ditulis di Cerberus, namun masih dibootstrap via VM Rust.
- Parsing script mendukung newline + `;`, namun masih berbasis opcode teks (belum bytecode biner native).
- Value runtime sudah pakai envelope typed `v1|kind|len|payload` (tetap kompatibel baca legacy `i:`, `b:`, `s:`).
- Call frame menyimpan `ip` + snapshot `locals` (encoded frame).

## Next Step Wajib Agar Lepas Total dari Rust

- Stage 1: format modul bytecode tekstual resmi + loader multiline yang stabil.
- Stage 2: transisi ke value model non-string (typed runtime value).
- Stage 3: pindahkan compiler stdlib untuk emit format yang langsung dikonsumsi runtime VM.
- Stage 4: build bootstrap chain (compiler.cer + runtime.cer) sebagai toolchain default.
- Stage 5: Rust host diturunkan menjadi fallback/legacy runner, bukan runtime utama.
