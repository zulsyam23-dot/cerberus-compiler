# Penilaian Detail: Compiler Cerberus yang Self-Host

Dokumen ini menjawab pertanyaan utama: seberapa jauh compiler Cerberus sudah berdiri sendiri, apa kelebihannya, dan apa kekurangannya.

## Definisi yang Dipakai

- **Self-hosted compiler**: compiler ditulis dalam bahasa Cerberus.
- **Standalone toolchain**: compiler + runtime + runner bisa dipakai tanpa binary host Rust sebagai jalur utama.

## Kesimpulan Singkat

Cerberus saat ini berada di level **self-hosted compiler (tercapai)**, tetapi **belum standalone penuh (belum tercapai)**.

## Kelebihan Detail

### 1) Arsitektur Modul Sudah Profesional

- `stdlib/` sudah mengikuti domain jelas:
  - `lexer/`, `parser/`, `typecheck/`, `codegen/`, `vm/`, `bytecode/`
- Struktur ini memudahkan onboarding kontributor dan mengurangi coupling.

### 2) Jalur Self-Host Sudah Menjadi Default

- CLI Rust menjadikan jalur selfhost sebagai default compile path.
- Integration test (`tests/selfhost_cli.rs`) memverifikasi:
  - default path selfhost
  - mode `selfhost`
  - limit enforcement
  - selfhost run mode

### 3) VM Host Memiliki Hardening Penting

- Ada validator sebelum eksekusi.
- Ada limit runtime (`steps`, `stack`, `call depth`).
- Ada hardening operasi aritmetika kritis.
- Error context relatif jelas untuk debugging.

### 4) Fitur Type Modern Sudah Mulai Tersambung

- Builtin `option_*` dan `result_*` typed sudah ada lintas typecheck/codegen/runtime.
- Koleksi (`vector`, `map`, dst) sudah terintegrasi di jalur utama.

## Kekurangan Detail

### 1) Ketergantungan Rust Masih Fundamental

- Eksekusi compiler self-host tetap lewat VM Rust.
- Runner/CLI/tooling harian tetap bertumpu pada binary Rust.
- Artinya, secara operasional Cerberus belum lepas total.

### 2) Kontrak Artifact dan Versi Belum Terkunci Penuh

- Belum ada dokumen compatibility policy yang tegas untuk versi artifact/toolchain.
- Risiko: perubahan instruksi/format bisa mempengaruhi stabilitas lintas versi.

### 3) Sebagian API Masih Transisi

- API text codegen `stdlib/codegen/codegen_core.cer` masih berupa placeholder (`bytecode:` + ast).
- Jalur compile utama memang memakai builder/emit path, tapi API publik perlu dirapikan agar tidak membingungkan kontributor.

### 4) Representasi Internal Masih Berat di String Encoding

- Banyak parsing/splitting AST berbasis string encoded node.
- Dampak:
  - overhead runtime lebih tinggi
  - debugging lebih rapuh jika format node berubah
  - refactor besar jadi lebih berisiko

### 5) Resolver Modul Belum Elegan

- Resolver di `stdlib/compiler.cer` panjang dan repetitif.
- Dampak:
  - maintainability rendah
  - peluang bug fallback path meningkat
  - biaya perubahan struktur folder lebih mahal

### 6) Kualitas Test Belum Menutup Semua Permukaan

- Ada test penting, tetapi conformance test matrix lintas fitur belum lengkap.
- Belum ada benchmark gate formal untuk performa self-host.

## Matriks Status per Sub-sistem

| Sub-sistem | Status Saat Ini | Nilai |
|---|---|---|
| Lexer self-host | Aktif dan terpakai | Kuat |
| Parser self-host | Aktif dan terpakai | Kuat |
| Typecheck self-host | Aktif dan terpakai | Kuat |
| Codegen self-host | Aktif untuk jalur compile utama | Menengah-Kuat |
| Runtime stdlib Cerberus | Ada dan modular | Menengah |
| VM engine tanpa Rust | Belum | Lemah |
| CLI/toolchain tanpa Rust | Belum | Lemah |
| Compatibility policy artifact | Belum formal penuh | Menengah-Lemah |
| Conformance + benchmark gate | Parsial | Menengah |

## Prioritas Tertinggi (Agar Benar-Benar Mandiri)

1. Turunkan Rust dari jalur utama menjadi fallback mode.
2. Bekukan kontrak artifact/version + policy kompatibilitas.
3. Rapikan API codegen yang masih placeholder agar tidak ambigu.
4. Sederhanakan resolver modul agar maintainable.
5. Tambah conformance suite dan benchmark gate sebagai syarat rilis.

## Pernyataan Status yang Aman Dipakai Tim

Kalimat yang akurat saat ini:

> "Cerberus sudah self-hosted di level compiler pipeline, namun belum standalone penuh karena eksekusi dan operasi toolchain utama masih bergantung pada runtime host Rust." 
