# Cerberus VM Professional Roadmap

Roadmap ini disusun dari status kode saat ini agar pengembangan VM tetap fokus: aman, bisa diprediksi, dan membawa Cerberus menuju toolchain yang benar-benar mandiri.

## Kondisi Saat Ini

VM host Rust sudah relatif matang untuk baseline produksi internal:

- validasi bytecode/script sebelum jalan
- runtime limits (`steps`, `stack`, `call depth`)
- operasi aritmetika kritis sudah hardened
- dispatch dipisah per domain (`instr_core`, `instr_io`, `instr_collections`, dll)
- ada unit/integration test dasar
- runtime stdlib sudah bisa load artifact package versioned `cerberus_toolchain_v1` (`vm_text_script`)

Namun, kemandirian total belum tercapai karena runtime utama tetap dieksekusi oleh binary Rust.

## Kelebihan Teknis yang Sudah Ada

- Safety baseline sudah baik:
  - invalid target / invalid index ditahan validator
  - limit eksekusi mencegah runaway workload
- Arsitektur modular:
  - pemisahan instruksi per file mengurangi kompleksitas perubahan
- Jalur self-host sudah terintegrasi:
  - compiler Cerberus bisa jadi jalur compile default
- Operability:
  - CLI sudah mendukung run/dump/selfhost/native fallback

## Kekurangan Teknis Kritis

- Mandiri penuh belum tercapai:
  - engine tetap host Rust, bukan runtime native Cerberus-only
- Kontrak artifact belum formal:
  - belum ada compatibility policy versi VM yang ketat dan terdokumentasi end-to-end
- Test strategy belum seimbang:
  - coverage CLI ada, tapi conformance suite lintas versi/runtime behavior masih minim
- Tooling observability belum lengkap:
  - belum ada profiler runtime dan analisis hot path yang sistematis

## Prioritas Implementasi

### Fase A - Contract Stabil (Wajib)

- Definisikan format artifact final:
  - `magic`, `version`, `feature flags`, metadata runtime
- Definisikan compatibility policy:
  - forward/backward rules untuk loader dan instruksi
- Definisikan ABI builtin yang versioned

### Fase B - Hardening + Conformance

- Tambah conformance suite:
  - parser script
  - validator
  - dispatch behavior
  - limit enforcement
- Tambah regression gates di CI untuk skenario self-host

### Fase C - Performance Baseline

- Tetapkan benchmark standar:
  - compile latency
  - run latency
  - peak memory
- Optimasi berdasarkan data:
  - dispatch hot path
  - alokasi string/value
  - call overhead

### Fase D - Standalone Capability

- Turunkan peran Rust menjadi fallback mode.
- Sediakan jalur distribusi toolchain yang menargetkan operasi harian tanpa ketergantungan host Rust penuh.

## Definition of Done (VM Production + Mandiri)

- Tidak ada panic dari input valid maupun invalid.
- Semua error kritis punya context yang cukup untuk debugging.
- Ada compatibility contract resmi lintas versi artifact.
- Ada conformance suite + benchmark suite yang jadi quality gate.
- Ada jalur operasi harian yang tidak mengandalkan runtime host Rust sebagai jalur utama.
