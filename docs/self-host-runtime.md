# Cerberus Self-Host Runtime: Status Nyata

Dokumen ini menjelaskan kondisi runtime dan compiler self-host Cerberus saat ini, dengan fokus pada batas kemampuan dan gap menuju toolchain yang benar-benar mandiri.

## Ringkasan Eksekutif

Cerberus **sudah self-hosted di level compiler pipeline** (lexer, parser, typecheck, codegen ditulis dalam Cerberus), tetapi **belum standalone penuh** karena eksekusi masih membutuhkan host Rust (`cerberus-compiler`) sebagai VM engine dan orchestrator CLI.

## Peta Modul Runtime Saat Ini

- `stdlib/runtime/runtime.cer`
  - entrypoint runtime mode script teks.
- `stdlib/vm/mod.cer`
  - facade VM untuk stdlib.
- `stdlib/vm/runtime/mod.cer`
  - agregator runtime VM (`builder`, `dispatch`, `validate`, `instr_*`, `tests`).
- `stdlib/vm/runtime/validate.cer`
  - preflight validasi directive/opcode/arg/target.
- `stdlib/vm/runtime/dispatch.cer`
  - loop dispatch instruksi.
- `stdlib/vm/runtime/instr_*.cer`
  - implementasi kategori instruksi (core, io, collections, control, env/fs, time/log).
- `stdlib/vm/runtime/tests.cer`
  - self-test runtime level script.

## Kontrak Script Runtime

- Directive:
  - `@cerberus_vm 1`
  - `@entry <label|ip>`
  - `@limit_steps <n>`
  - `@limit_stack <n>`
  - `@limit_call <n>`
- Separator instruksi:
  - newline (direkomendasikan)
  - `;` (kompatibilitas)
- Inline comment: `# ...`
- Validasi dilakukan sebelum eksekusi (`rt_validate_text`).

## Kontrak Artifact Toolchain v1 (Baru)

Runtime sekarang juga menerima paket artifact versioned (bukan hanya script polos):

- magic: `cerberus_toolchain_v1`
- format: `vm_text_script`
- marker payload: `::code::`
- header metadata opsional:
  - `entry=<label|ip>`
  - `limit_steps=<n>`
  - `limit_stack=<n>`
  - `limit_call=<n>`

Contoh:

`cerberus_toolchain_v1;vm_text_script;entry=main;limit_steps=100000::code::label main; const_int 1; println; halt;`

Runtime akan me-resolve artifact ini menjadi script VM valid (`@cerberus_vm 1` + directive runtime) sebelum eksekusi.

## Launcher Tooling (Baru)

Untuk mengurangi ketergantungan langsung pada host workflow, stdlib sekarang punya launcher:

- `stdlib/toolchain.cer`
  - `pack <vm_script.cer> [out.crt]`
  - `validate <vm_script.cer|out.crt>`
  - `run <vm_script.cer|out.crt>`
  - `resolve <out.crt> [resolved.cer]`
  - `selftest`

Compiler juga mendukung mode package:

- `compiler --package <vm_script.cer> [out.crt]`

## Kelebihan Runtime Saat Ini

- Ada lapisan validasi script sebelum jalan.
- Ada limit eksekusi untuk mencegah loop tak terbatas atau stack abuse.
- Struktur runtime dipisah per domain instruksi, tidak lagi monolitik.
- Error runtime sudah membawa konteks yang cukup untuk debugging dasar.
- Runtime self-test tersedia di stdlib (`vm_runtime_tests_run`).

## Kekurangan Runtime Saat Ini

- Masih jalan di atas host Rust (belum ada engine native Cerberus yang bisa boot sendiri tanpa binary host).
- Boundary host I/O (`readfile`, `writefile`, `env`, `fs`) masih tergantung implementasi VM host.
- Format utama masih script opcode teks; belum menjadi paket runtime mandiri dengan bytecode container + manifest yang stabil.
- Kontrak ABI antar versi runtime belum dibakukan sebagai compatibility policy formal.

## Status Kemandirian (Reality Check)

- Compiler self-host: **ya**
- Runtime logic ditulis Cerberus: **ya (sebagian besar di stdlib/vm/runtime)**
- Toolchain tanpa binary Rust: **belum**
- Distribusi portable murni Cerberus: **belum**

## Target Minimum Agar Bisa Disebut Standalone

- Ada runtime launcher native Cerberus (atau bootstrap binary minimal non-Rust) yang bukan host VM Rust penuh.
- Ada format artifact stabil (`magic`, `version`, `feature flags`) untuk eksekusi lintas versi.
- Ada paket stdlib + compiler + runtime yang bisa di-deploy sebagai toolchain tunggal.
- Rust path menjadi fallback/legacy, bukan jalur default produksi.
