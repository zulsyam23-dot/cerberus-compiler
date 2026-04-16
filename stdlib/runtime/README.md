# Runtime Module Layout

Dokumen ini menjelaskan layout runtime yang aktif setelah restrukturisasi stdlib.

## Struktur Modul Saat Ini

- `stdlib/runtime/runtime.cer`
  - entrypoint runtime mode script.
- `stdlib/vm/mod.cer`
  - facade VM level stdlib.
- `stdlib/vm/runtime/mod.cer`
  - agregator runtime (`builder`, `dispatch`, `validate`, `instr_*`, `tests`).
- `stdlib/vm/runtime/validate.cer`
  - validasi directive/opcode/arg/target.
- `stdlib/vm/runtime/dispatch.cer`
  - routing eksekusi instruksi.
- `stdlib/vm/runtime/instr_core.cer`
  - instruksi arithmetic/logic/load-store.
- `stdlib/vm/runtime/instr_control.cer`
  - instruksi control-flow.
- `stdlib/vm/runtime/instr_collections.cer`
  - vector/map/option/result.
- `stdlib/vm/runtime/instr_io.cer`
  - read/write/arg/println.
- `stdlib/vm/runtime/instr_env_fs.cer`
  - env + filesystem helper ops.
- `stdlib/vm/runtime/instr_time_log.cer`
  - waktu dan logging runtime.
- `stdlib/vm/runtime/tests.cer`
  - self-test runtime script.

## Kelebihan Runtime Saat Ini

- Komponen sudah terpecah rapi sesuai fungsi.
- Validasi runtime tersedia sebelum dispatch.
- Ada runtime limits untuk keamanan.
- Builtin modern (`option_*`, `result_*`, collections) sudah masuk jalur runtime.

## Kekurangan Runtime Saat Ini

- Runtime masih dieksekusi melalui host VM Rust.
- Belum ada paket artifact runtime yang versioned secara formal.
- Self-test stdlib runtime belum menjadi satu-satunya quality gate; masih perlu conformance suite yang lebih luas.

## Catatan Operasional

- Script opcode tekstual tetap didukung.
- Directive runtime harus berada di bagian awal script.
- Untuk target standalone penuh, dependency pada host runtime perlu diturunkan bertahap hingga fallback-only.

## Artifact Package v1

Runtime juga menerima artifact package format:

- `cerberus_toolchain_v1;vm_text_script;...::code::<payload>`
- metadata header opsional:
  - `entry=...`
  - `limit_steps=...`
  - `limit_stack=...`
  - `limit_call=...`

Contoh singkat:

`cerberus_toolchain_v1;vm_text_script;entry=main::code::label main; const_int 9; println; halt;`

## Related Commands

- `compiler --package <vm_script.cer> [out.crt]`
  - membungkus script VM menjadi artifact v1.
- `toolchain pack <vm_script.cer> [out.crt]`
  - launcher stdlib untuk packaging.
- `toolchain run <vm_script.cer|out.crt>`
  - menjalankan script atau artifact package melalui VM runtime stdlib.
