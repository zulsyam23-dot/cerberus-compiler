# Cerberus Compiler

A self-hosted compiler written in Cerberus, currently executing through a Rust VM host. Cerberus is progressing toward complete independence from the Rust host, with compiler pipelines (lexer, parser, typecheck, codegen) now written in the Cerberus language itself.

## Quick Status

| Component | Status | Notes |
|-----------|--------|-------|
| **Lexer (self-hosted)** | ✅ Complete | Running in Cerberus |
| **Parser (self-hosted)** | ✅ Complete | Running in Cerberus |
| **Typecheck (self-hosted)** | ✅ Complete | Running in Cerberus |
| **Codegen (self-hosted)** | ✅ Complete | Running in Cerberus for main pipeline |
| **VM Engine (host-independent)** | ❌ In Progress | Currently requires Rust VM host |
| **Standalone Toolchain** | ❌ In Progress | CLI still depends on Rust binary |

## Key Achievements

✅ **Professional Module Architecture**
- `stdlib/` organized by domain: `lexer/`, `parser/`, `typecheck/`, `codegen/`, `vm/`, `runtime/`
- Easy for contributors to navigate and understand module boundaries

✅ **Self-Host Default Path**
- CLI defaults to self-hosted compiler (`stdlib/compiler.cer`)
- Integration test validates self-host path (`tests/selfhost_cli.rs`)

✅ **VM Hardening & Safety**
- Bytecode/script validation before execution
- Runtime limits: `steps`, `stack depth`, `call depth`
- Checked arithmetic on critical operations
- Comprehensive error context for debugging

✅ **Modern Type System**
- `option_*` and `result_*` types integrated across typecheck→codegen→runtime
- Collections (`vector`, `map`) fully wired in the pipeline

✅ **Artifact Versioning System**
- New `cerberus_toolchain_v1` format for versioned artifacts
- Supports metadata headers: `entry`, `limit_steps`, `limit_stack`, `limit_call`

## Current Limitations

❌ **Rust Dependency**
- Compiler self-host still executes through Rust VM
- CLI toolchain still requires Rust binary
- Blocks full operational independence

❌ **Artifact Contract Not Finalized**
- Compatibility policy across versions not yet formal
- Risk of breaking changes in instruction set or format

❌ **Partial API Stability**
- Text codegen API (`stdlib/codegen/codegen_core.cer`) still placeholder
- Main compile path uses builder/emit, but public API needs cleanup

❌ **Internal Representation Overhead**
- Heavy reliance on string-encoded AST nodes
- Higher runtime parsing overhead
- More fragile during refactoring

❌ **Module Resolver Needs Simplification**
- Resolver in `stdlib/compiler.cer` is verbose and repetitive
- High maintenance burden and bug risk

## Command Reference

### Compilation
```bash
# Compile with default self-host path
cerberus-compiler compile input.cer -o output.crt

# Compile using native Rust fallback (when self-host unavailable)
cerberus-compiler compile input.cer --native

# Package VM script as versioned artifact
cerberus-compiler --package script.cer -o artifact.crt
```

### Runtime Tooling
```bash
# Run toolchain launcher
toolchain pack script.cer [output.crt]        # Package artifact
toolchain run script.cer|artifact.crt         # Execute code
toolchain validate script.cer|artifact.crt    # Validate without execution
toolchain resolve artifact.crt [output.cer]   # Extract and resolve artifact
toolchain selftest                            # Self-test runtime
```

### Artifact Format
```
cerberus_toolchain_v1;vm_text_script;entry=main;limit_steps=100000::code::
label main
const_int 42
println
halt
```

**Metadata Options:**
- `entry=<label|ip>` - Entry point label or instruction pointer
- `limit_steps=<n>` - Maximum execution steps
- `limit_stack=<n>` - Maximum stack depth
- `limit_call=<n>` - Maximum call depth

## Technical Documentation

- **[Architecture Guide](docs/ARCHITECTURE.md)** - System design and module relationships
- **[Self-Host Runtime Status](docs/self-host-runtime.md)** - Detailed runtime capabilities and gaps
- **[Professional Roadmap](docs/vm-professional-roadmap.md)** - Development priorities and milestones
- **[Compiler Assessment](docs/compiler-standalone-assessment.md)** - Self-hosted achievement analysis
- **[Glossary](docs/GLOSSARY.md)** - Technical terminology reference
- **[Module Layout](stdlib/runtime/README.md)** - Runtime stdlib structure

## Getting Started

### Prerequisites
- Rust 1.70+ (for building the compiler)
- Standard C compiler (for linking)

### Build
```bash
cargo build --release
./target/release/cerberus-compiler --version
```

### Run Tests
```bash
cargo test
cargo test --test selfhost_cli  # Verify self-host pipeline
```

### Write Your First Program
```cerberus
fn main() {
  let x = 42;
  println(x);
}
```

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for:
- Code style and conventions
- Testing requirements
- PR process
- Module modification guidelines

## Project Structure

```
├── src/                        # Rust host implementation
│   ├── main.rs                 # CLI entry point
│   ├── vm.rs                   # VM execution engine
│   └── ...
├── stdlib/                     # Cerberus standard library (compiler + runtime)
│   ├── compiler.cer            # Main self-hosted compiler
│   ├── lexer/                  # Lexical analysis (Cerberus)
│   ├── parser/                 # Parsing (Cerberus)
│   ├── typecheck/              # Type checking (Cerberus)
│   ├── codegen/                # Code generation (Cerberus)
│   ├── runtime/                # VM runtime (Cerberus)
│   ├── vm/                     # VM runtime modules
│   └── toolchain.cer           # Artifact tooling
├── tests/                      # Integration tests
├── docs/                       # Technical documentation
└── Cargo.toml
```

## Next Milestones (Toward Independence)

1. **Phase A - Contract Finalization** (Current)
   - Freeze artifact format and ABI
   - Document compatibility policy
   - Formalize runtime contract

2. **Phase B - Hardening & Conformance**
   - Comprehensive conformance suite
   - Regression gates in CI
   - Performance benchmarks

3. **Phase C - Performance Baseline**
   - Establish compile/runtime latency targets
   - Memory usage optimization
   - Hot path analysis

4. **Phase D - Standalone Distribution**
   - Bootstrap minimal native launcher
   - Move Rust to fallback-only mode
   - Distribute pure Cerberus toolchain

## Status Statement

> **"Cerberus is a self-hosted compiler where the pipeline components are written in Cerberus itself. However, it is not yet a standalone toolchain because compiler execution and daily toolchain operations still require a Rust VM host."**

## License

[LICENSE file to be added]

## Support

- Open an [issue](https://github.com/zulsyam23-dot/cerberus-compiler/issues) for bugs or questions
- Check [ARCHITECTURE.md](docs/ARCHITECTURE.md) for design questions
- See [GLOSSARY.md](docs/GLOSSARY.md) for terminology

---

**Last Updated:** 2026-04-29  
**Compiler Version:** Self-hosted (Cerberus + Rust VM host)
