# Cerberus VM Professional Roadmap

Dokumen ini menjadi peta teknis agar VM Cerberus bisa mandiri, stabil, dan siap jadi fondasi bahasa yang berdiri sendiri.

## Tujuan Utama

- Menjalankan bytecode Cerberus secara aman, deterministik, dan bisa diprediksi.
- Menyediakan diagnostik runtime yang kuat untuk developer bahasa.
- Menjadi fondasi untuk self-hosting (compiler/tooling berjalan di Cerberus VM).
- Memiliki jalur evolusi jelas: interpreter stabil -> optimisasi -> JIT/Native backend.

## Prinsip Arsitektur

- Safety-first runtime: invalid bytecode tidak boleh menyebabkan panic/UB.
- Resource bounded: eksekusi dibatasi (step/stack/call depth) agar aman.
- Deterministic core: hasil eksekusi konsisten lintas mesin.
- Evolvable ISA: set instruksi bisa bertambah tanpa mematahkan kompatibilitas.
- Observability: error + stack trace + profil runtime harus jelas.

## Status Saat Ini (Sudah Diimplementasikan)

- Bytecode validation sebelum VM start:
  - validasi entry function
  - validasi local index (`Load/Store/Read*`)
  - validasi jump target (`Jump/JumpIfFalse`)
  - validasi function call target (`Call`)
  - validasi limit jumlah function/instruksi/local
- Runtime limits bawaan:
  - `max_steps`
  - `max_stack_size`
  - `max_call_depth`
  - `max_locals_per_function`
- Arithmetic hardening:
  - `Add/Sub/Mul/Neg` pakai checked arithmetic
  - `Div` aman dari division-by-zero + overflow
- Runtime stack trace pada error.
- Unit tests untuk validator, step limit, call-depth limit, dan division-by-zero.

## Fase Berikutnya

### Fase 1 - Runtime Contract dan ABI

- Definisikan versi bytecode (`magic + version + feature flags`).
- Definisikan ABI fungsi builtin dan konvensi call stack.
- Tambah verifier untuk stack effect (tinggi stack per basic block).

### Fase 2 - Object Model dan Memory Management

- Migrasi `Value` ke object model yang lebih seragam.
- Implementasi heap arena/GC (awal: mark-sweep sederhana).
- Pisahkan immutable/mutable structure untuk efisiensi copy.

### Fase 3 - Eksekusi dan Performa

- Dispatch optimization (threaded dispatch / direct threaded style).
- Constant pool untuk string literal agar mengurangi clone.
- Inline cache untuk operasi koleksi dan call site panas.
- Siapkan IR menengah untuk jalur JIT di masa depan.

### Fase 4 - Standalone Runtime Capability

- Modul stdlib runtime resmi (io/fs/time/env) dengan contract jelas.
- Sandbox mode opsional untuk batas akses file/env.
- Packaging bytecode + manifest dependensi.

### Fase 5 - Tooling Profesional

- Disassembler dan inspector yang lebih kaya (CFG, stack effect).
- Profiling hooks (instr count, hot function report).
- Conformance test suite untuk kompatibilitas versi VM.

## Definition of Done (VM Production Ready)

- Tidak ada panic runtime dari input bytecode valid maupun invalid.
- Semua error runtime punya stack trace yang terbaca.
- Ada compatibility policy untuk versi bytecode.
- Ada benchmark suite + regression gate di CI.
- Ada test matrix lintas OS untuk determinism.