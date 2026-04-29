# Cerberus Compiler

Cerberus saat ini sudah berada pada fase **self-hosted compiler pipeline**: jalur kompilasi default memakai compiler yang ditulis dalam bahasa Cerberus (`stdlib/compiler.cer`) dan dijalankan di VM.

## Status Singkat

- Frontend (`lexer` + `parser`) self-host: **ya**
- Typecheck self-host: **ya**
- Codegen self-host: **ya**
- VM engine host-independent: **belum**
- CLI/toolchain tanpa binary Rust: **belum**

## Kelebihan Saat Ini

- Struktur `stdlib/` sudah dipisah per domain (`lexer`, `parser`, `typecheck`, `codegen`, `vm`) sehingga mudah diikuti kontributor.
- Default flow CLI sudah mengutamakan self-host (`tests/selfhost_cli.rs` memverifikasi ini).
- VM Rust sudah punya hardening penting: validator bytecode, runtime limits, checked arithmetic, dan stack trace error.
- Builtins koleksi/runtime plus `option_*`/`result_*` typed sudah tersambung lintas typecheck, codegen, dan runtime.

## Kekurangan Utama Saat Ini

- Masih bergantung pada binary Rust untuk mengeksekusi compiler self-host dan runtime bytecode.
- Distribusi final belum berupa toolchain native Cerberus (masih host-driven).
- Beberapa API/komponen masih transisi (contoh API text codegen di `stdlib/codegen/codegen_core.cer` masih placeholder).
- Coverage test untuk perilaku self-host level modul masih perlu diperluas.

## Dokumen Teknis

- Arsitektur runtime dan gap kemandirian: `docs/self-host-runtime.md`
- Roadmap profesional VM: `docs/vm-professional-roadmap.md`
- Penilaian detail kelebihan/kekurangan compiler mandiri: `docs/compiler-standalone-assessment.md`
- Interop C (FFI) milestone awal: `docs/c-interop.md`
- Layout runtime stdlib: `stdlib/runtime/README.md`

## Command Baru (Milestone Standalone)

- `compiler --package <vm_script.cer> [out.crt]`
  - membungkus VM script ke artifact `cerberus_toolchain_v1`.
- `toolchain pack|run|validate|resolve|selftest`
  - launcher stdlib untuk workflow artifact package.
