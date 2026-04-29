# Cerberus Independence Roadmap: Technical Challenges & Solutions

**Status:** Phase A - Contract Finalization  
**Target:** Full standalone operation without Rust dependency  
**Timeline:** 12 months to independence  
**Last Updated:** 2026-04-29

---

## Executive Summary

Cerberus is NOT yet a standalone toolchain because:

1. **Rust Execution Dependency** - Compiler runs through Rust VM
2. **Bootstrap Problem** - Can't compile itself without existing runtime
3. **Artifact Format Instability** - Format not finalized, breaking changes risk
4. **Internal Representation Overhead** - String-based AST adds complexity
5. **Module Resolver Complexity** - Difficult to maintain and optimize

This document explains each blocker, why it exists, and the precise steps to eliminate it.

---

## Table of Contents

1. [Challenge 1: Rust Execution Dependency](#challenge-1-rust-execution-dependency)
2. [Challenge 2: Bootstrap Paradox](#challenge-2-bootstrap-paradox)
3. [Challenge 3: Artifact Format Instability](#challenge-3-artifact-format-instability)
4. [Challenge 4: Internal Representation Overhead](#challenge-4-internal-representation-overhead)
5. [Challenge 5: Module Resolver Complexity](#challenge-5-module-resolver-complexity)
6. [Cross-Cutting Issues](#cross-cutting-issues)
7. [Independence Roadmap](#independence-roadmap)
8. [Success Metrics](#success-metrics)

---

## Challenge 1: Rust Execution Dependency

### The Problem

**Current State:**
```
Cerberus Source Code (.cer)
    ↓
[Cerberus Compiler in Cerberus] ← Running inside Rust VM
    ↓
Bytecode (.crt)
    ↓
[Rust VM Executor] ← Direct dependency
    ↓
Output
```

**Why This Blocks Independence:**
- Every Cerberus program execution depends on Rust binary
- Cannot distribute "pure Cerberus" without including Rust toolchain
- Users must have Rust installed to use Cerberus
- Defeats the purpose of a self-hosted compiler

### Root Cause Analysis

#### Why Rust is Still Required

**1. VM Implementation**
```rust
// src/vm/runtime/executor.rs
pub struct VirtualMachine {
    stack: Vec<Value>,
    memory: Vec<u8>,
    pc: u32,  // Program counter
    ...
}

impl VirtualMachine {
    fn execute_instruction(&mut self, instr: Instr) -> Result<(), VmError> {
        match instr {
            Instr::Add => self.stack.push(...),
            Instr::Call => self.call_function(...),
            ...
        }
    }
}
```

The VM is implemented in Rust because:
- **Performance**: Native code execution faster than interpreted VM
- **Memory Safety**: Rust guarantees no buffer overflows
- **FFI Support**: Easy to call C/system libraries

**2. System Integration**
```rust
// File I/O
fn fs_read_file(path: &str) -> Result<String, IoError> {
    std::fs::read_to_string(path)
}

// Environment access
fn env_get(var: &str) -> Option<String> {
    std::env::var(var).ok()
}

// Timing
fn now_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}
```

Rust provides direct OS access that's hard to replicate from pure bytecode.

**3. CLI & Tooling**
```rust
// src/main.rs
fn main() -> anyhow::Result<()> {
    let args = std::env::args();
    // ... parse CLI arguments
    // ... resolve sidecar packages
    // ... execute compiler or VM
}
```

The Rust binary is the entry point that bootstraps everything.

### The Vicious Cycle

```
To run Cerberus code:
  Need: Cerberus runtime (VM)
  
To run the Cerberus runtime:
  Need: Compiled bytecode or native executable
  
To compile to bytecode:
  Need: Cerberus compiler
  
To run the Cerberus compiler:
  Need: Cerberus runtime (VM)  ← Back to start!
```

### Solution: Staged Bootstrap

#### Stage 1: Current State (Months 1-2)

Keep Rust VM but make it **non-critical**:

```
Cerberus Source
    ↓
[Cerberus Compiler in Cerberus via Rust VM]
    ↓
Bytecode
    ↓
[Rust VM (only fallback)]
```

**Actions:**
- Stabilize self-hosted compiler path
- Ensure bytecode is reproducible
- Create comprehensive conformance tests

#### Stage 2: Precompiled Artifact (Months 3-5)

Package Cerberus runtime as artifact:

```
Pre-built Artifact:
  cerberus_runtime.crt (contains runtime bytecode)
  
To bootstrap:
  1. Minimal C launcher loads runtime artifact
  2. Runtime executes compiler bytecode
  3. Compiler compiles user code
```

**Actions:**
- Compile Cerberus runtime to bytecode artifact
- Create C launcher (500 LOC) to load artifact
- Eliminate Rust from critical path

#### Stage 3: Minimal Native Launcher (Months 6-8)

Replace Rust binary with C/ASM launcher:

```
launcher.c (200 LOC)
  ├── Load cerberus_runtime.crt
  ├── Initialize VM stack/memory
  ├── Execute runtime bytecode
  └── Return to user

[No Rust in production path]
```

**Actions:**
- Write C launcher for each platform (Linux, macOS, Windows)
- Implement minimal VM in C (instruction dispatch only)
- Create build pipeline for cross-compilation

#### Stage 4: Full Independence (Months 9-12)

Pure Cerberus distribution:

```
Distribution Package:
  ├── launcher (C binary, platform-specific)
  ├── cerberus_runtime.crt (pre-compiled)
  ├── stdlib/ (source for reference)
  └── README

No Rust dependency!
```

**Actions:**
- Remove Rust from build requirements
- Create source-free distribution
- Establish backward compatibility guarantee

### Detailed Implementation Plan

#### Phase 1: Stabilize Self-Host Path

**Objective:** Ensure `stdlib/compiler.cer` produces valid bytecode 100% of time

**Tasks:**

1. **Add Conformance Tests**
   ```rust
   // tests/conformance.rs
   #[test]
   fn test_self_host_deterministic() {
       let source = "let x = 42; println(x);";
       let output1 = compile_with_self_host(source);
       let output2 = compile_with_self_host(source);
       assert_eq!(output1, output2);  // Bytecode must be identical
   }
   
   #[test]
   fn test_self_host_produces_valid_bytecode() {
       let source = "fn main() { println(42); }";
       let bytecode = compile_with_self_host(source);
       let result = execute_bytecode(bytecode);
       assert_eq!(result.output, "42\n");
   }
   ```

2. **Implement CI Gate**
   ```yaml
   # .github/workflows/conformance.yml
   on: [push, pull_request]
   jobs:
     conformance:
       runs-on: ubuntu-latest
       steps:
         - uses: actions/checkout@v2
         - run: cargo build --release
         - run: cargo test --test conformance
         - run: ./tests/regression_suite.sh
   ```

3. **Establish Regression Testing**
   ```bash
   #!/bin/bash
   # tests/regression_suite.sh
   
   for test_file in tests/programs/*.cer; do
       echo "Testing $test_file"
       ./target/release/cerberus-compiler compile "$test_file" -o /tmp/out.crt
       ./target/release/cerberus-compiler run /tmp/out.crt > /tmp/actual.txt
       
       expected_file="${test_file%.cer}.expected"
       if ! diff /tmp/actual.txt "$expected_file"; then
           echo "FAIL: $test_file output mismatch"
           exit 1
       fi
   done
   
   echo "All regression tests passed!"
   ```

#### Phase 2: Precompiled Runtime Artifact

**Objective:** Package Cerberus runtime as `cerberus_runtime.crt`

**Steps:**

1. **Compile Runtime to Bytecode**
   ```bash
   # Build with self-host
   ./target/release/cerberus-compiler compile stdlib/runtime.cer -o cerberus_runtime.crt
   
   # Verify it works
   ./target/release/cerberus-compiler run cerberus_runtime.crt --selftest
   ```

2. **Create Artifact Wrapper**
   ```
   cerberus_toolchain_v1;vm_bytecode;entry=_runtime_main;limit_steps=unlimited::code::
   [bytecode payload of runtime]
   ```

3. **Embed in Distribution**
   ```rust
   // src/sidecar.rs
   const RUNTIME_ARTIFACT: &[u8] = include_bytes!("../cerberus_runtime.crt");
   
   pub fn get_embedded_runtime() -> &'static [u8] {
       RUNTIME_ARTIFACT
   }
   ```

#### Phase 3: Minimal C Launcher

**Objective:** Create standalone launcher in C (< 500 lines)

**File:** `launcher/main.c`

```c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Embedded runtime artifact
extern const unsigned char RUNTIME_ARTIFACT[];
extern const size_t RUNTIME_ARTIFACT_SIZE;

// Minimal VM state
typedef struct {
    int32_t stack[65536];
    size_t sp;
    int32_t memory[1048576];
} VirtualMachine;

// Core instruction dispatch
void execute_bytecode(VirtualMachine* vm, const unsigned char* bytecode, size_t size) {
    size_t pc = 0;
    
    while (pc < size) {
        uint8_t opcode = bytecode[pc++];
        
        switch (opcode) {
            case 0x00:  // HALT
                return;
            case 0x01:  // CONST_INT
                {
                    int32_t val = *(int32_t*)&bytecode[pc];
                    pc += 4;
                    vm->stack[vm->sp++] = val;
                }
                break;
            case 0x02:  // ADD
                {
                    int32_t b = vm->stack[--vm->sp];
                    int32_t a = vm->stack[--vm->sp];
                    vm->stack[vm->sp++] = a + b;
                }
                break;
            case 0x03:  // PRINTLN
                {
                    int32_t val = vm->stack[--vm->sp];
                    printf("%d\n", val);
                }
                break;
            // ... more opcodes
        }
    }
}

int main(int argc, char* argv[]) {
    if (argc < 2) {
        fprintf(stderr, "usage: cerberus <program.crt>\n");
        return 1;
    }
    
    // Load bytecode file
    FILE* f = fopen(argv[1], "rb");
    if (!f) {
        perror("fopen");
        return 1;
    }
    
    fseek(f, 0, SEEK_END);
    size_t size = ftell(f);
    rewind(f);
    
    unsigned char* bytecode = malloc(size);
    fread(bytecode, 1, size, f);
    fclose(f);
    
    // Execute
    VirtualMachine vm = {0};
    execute_bytecode(&vm, bytecode, size);
    
    free(bytecode);
    return 0;
}
```

**Build:**
```bash
gcc -O2 -o cerberus launcher/main.c
./cerberus program.crt
```

---

## Challenge 2: Bootstrap Paradox

### The Problem

> "To compile Cerberus, you need Cerberus. But to get Cerberus, you need to compile Cerberus."

This is the classic bootstrapping problem faced by self-hosting compilers.

### Why It Matters

**Current Workaround:**
```
Rust Compiler (trusted, external)
    ↓
[Rust implementation of compilation]
    ↓
Bytecode artifact
    ↓
[Run Cerberus compiler inside VM]
    ↓
Final bytecode
```

The Rust "bootstrap" is necessary because we can't compile the Cerberus compiler without it.

### Root Cause

1. **Circular Dependency**
   ```
   To run Cerberus code:
     Need runtime (VM bytecode)
   
   To compile runtime:
     Need compiler
   
   To run compiler:
     Need runtime ← Circular!
   ```

2. **Compiler Stages**
   ```
   Cerberus Compiler consists of:
   ├── Lexer (tokenize source)
   ├── Parser (build AST)
   ├── Type Checker (validate types)
   └── Codegen (emit bytecode)
   
   All stages written in Cerberus
   But need runtime to execute them!
   ```

### The Bootstrap Hierarchy

```
Level 0: Machine Code (CPU understands directly)
         ↑
         [Rust compiler - trusted base]
         ↑
Level 1: Rust-compiled Cerberus VM (can execute bytecode)
         ↑
         [Bytecode of compiler stages]
         ↑
Level 2: Cerberus Compiler (lexer, parser, typecheck, codegen)
         ↑
         [Bytecode of runtime]
         ↑
Level 3: User Cerberus Programs (any .cer file)
```

### Solution: Staged Bootstrap

#### Bootstrap Strategy: Precompile the Compiler

**Idea:** Compile the Cerberus compiler once using Rust, then use the compiled bytecode forever.

**Steps:**

1. **Initial Compilation** (Done in Rust, one-time)
   ```bash
   # Using Rust compiler
   rustc compile cerberus_compiler.cer into compiler_bootstrap.crt
   ```

2. **Archive as Artifact**
   ```
   cerberus_toolchain_v1;vm_bytecode;entry=compiler_main::code::
   [full bytecode of compiler pipeline]
   ```

3. **Distribute with Runtime**
   ```
   cerberus-1.0.tar.gz
   ├── launcher (C binary)
   ├── cerberus_runtime.crt (runtime bytecode)
   ├── compiler.crt (compiler bytecode)
   └── stdlib/ (source files for reference)
   ```

4. **Usage**
   ```bash
   # To compile a program:
   ./launcher compiler.crt myprogram.cer > output.crt
   
   # To run it:
   ./launcher output.crt
   ```

#### One-Time Trusted Build

```
Bootstrap Chain (One-Time):

1. Write Rust implementation of VM
2. Write Cerberus compiler in Cerberus
3. Compile compiler with Rust VM → compiler.crt
4. Archive compiler.crt
5. Forever: Use launcher + archived compiler.crt
```

**Trust Model:**
```
Trust Rust compiler once.
        ↓
Produce cerberus_runtime.crt
Produce compiler.crt
        ↓
Distribute launcher + .crt files
        ↓
Never need Rust again
```

#### Implementation Timeline

**Month 1-2: Prepare**
- [ ] Ensure `stdlib/compiler.cer` compiles successfully
- [ ] Create automated bootstrap build
- [ ] Add integrity checks (hash verification)

**Month 3-4: Archive**
- [ ] Compile all stdlib components to bytecode
- [ ] Package as artifacts
- [ ] Create distribution bundle

**Month 5-6: Distribute**
- [ ] Release bootstrap toolchain
- [ ] Test with real users
- [ ] Document bootstrap process

---

## Challenge 3: Artifact Format Instability

### The Problem

**Current Artifact Format:**
```
cerberus_toolchain_v1;vm_text_script;entry=main;limit_steps=100000::code::
label main
const_int 42
println
halt
```

**Issues:**

1. **No Version Tied to Instructions**
   - v1 format doesn't specify which instruction set version
   - Adding new instructions breaks compatibility
   - No migration path for old artifacts

2. **No Compatibility Policy**
   - What happens to v1 artifacts when v2 released?
   - Can old artifacts run on new runtimes?
   - Can new artifacts run on old runtimes?

3. **No Deprecation Process**
   - Instructions can't be safely removed
   - No way to mark instructions as deprecated
   - Breaking changes accumulate

### Root Cause Analysis

#### Instruction Set Churn

Currently, instruction set evolves:
```rust
// Old v1 instructions
Instr::Add,
Instr::Sub,
Instr::Mul,

// New instructions added (breaks compatibility)
Instr::VecNew,
Instr::OptSome,
Instr::ResOk,
```

**Problem:** Old bytecode expects specific opcode numbers. Adding new instructions shifts them:
```
Before: Add = 10, Sub = 11, Mul = 12
After:  Add = 10, Sub = 11, Mul = 12, VecNew = 13 ← OK

But if we insert:
After:  NewOp = 10, Add = 11, Sub = 12, Mul = 13 ← BREAKS!
```

#### Missing Specification

No formal document defines:
- Opcode assignment (is 255 reserved?)
- Bytecode layout (big-endian? little-endian?)
- String encoding (UTF-8? length-prefixed?)
- Metadata format (version, limits, entry point)

### Solution: Formal Artifact Specification

#### Create `docs/ARTIFACT_SPEC_v1.md`

```markdown
# Cerberus Artifact Format v1 Specification

## Format Overview

```
cerberus_toolchain_v1;vm_text_script;[metadata]::code::[payload]
```

## Version Scheme

- **Major Version (v1, v2, ...)**: Breaking changes to instruction set
- **Minor Version (1.0, 1.1, ...)**: Backward-compatible changes
- **Patch Version (1.0.0, 1.0.1, ...)**: Bug fixes, no semantic changes

## Instruction Set v1 (Frozen)

Opcodes 0-200 are reserved and frozen:

| Code | Instruction | Encoding | Notes |
|------|-------------|----------|-------|
| 0x00 | HALT | 1 byte | Terminates execution |
| 0x01 | CONST_INT | 1 + 8 bytes | Value as i64 |
| 0x02 | CONST_BOOL | 1 + 1 byte | 0=false, 1=true |
| 0x03 | CONST_STR | 1 + 4 + N bytes | Length + UTF-8 |
| 0x04 | LOAD | 1 + 4 bytes | Variable index |
| 0x05 | STORE | 1 + 4 bytes | Variable index |
| ... | ... | ... | ... |
| 0xC8 | RESERVED | - | 200-255 reserved for v1 extensions |

## Compatibility Guarantee

- A v1 artifact runs unchanged on any v1+ runtime
- Unknown opcodes (> 200) trigger error with helpful message
- Old instruction encodings never change meaning

## Metadata Header

```
entry=<label_or_ip>
limit_steps=<n>          # Optional: execution step limit
limit_stack=<n>          # Optional: max stack depth
limit_call=<n>           # Optional: max call depth
```

## String Encoding

All strings are UTF-8:
```
[length:u32][utf8_bytes:N]
```

Example: "hello" (5 bytes)
```
0x00 0x00 0x00 0x05 'h' 'e' 'l' 'l' 'o'
```

## Bytecode Example

```
Artifact:
cerberus_toolchain_v1;vm_bytecode;entry=main::code::
[opcode=0x01][value=0x2A 0x00 0x00 0x00 0x00 0x00 0x00 0x00]  // CONST_INT 42
[opcode=0x03]                                                   // PRINTLN
[opcode=0x00]                                                   // HALT
```

## Forward Compatibility

- Runtimes must ignore unknown opcodes (>200) gracefully
- Runtimes should log warning if unknown opcode encountered
- Runtimes may support extension opcodes in ranges:
  - 200-240: Experimental extensions (may change)
  - 241-254: Stable extensions (won't change)
  - 255: Reserved

## Testing

All conformance tests verify:
1. v1 artifact can run on any v1+ runtime
2. Output is deterministic
3. Limits are enforced correctly
4. Unknown opcodes handled gracefully
```

#### Conformance Test Suite

**File:** `tests/artifact_conformance.rs`

```rust
#[test]
fn test_artifact_v1_forward_compat() {
    // v1 artifact should run on current runtime
    let artifact = load_artifact_v1("tests/artifacts/v1_simple.crt");
    let result = execute_artifact(artifact);
    assert_eq!(result.output, "42\n");
}

#[test]
fn test_artifact_v1_unknown_opcode() {
    // Unknown opcode should not crash
    let artifact = create_artifact_with_unknown_opcode(0xFF);
    let result = execute_artifact(artifact);
    assert!(result.error.contains("Unknown opcode"));
}

#[test]
fn test_artifact_deterministic() {
    // Same input → same output always
    let source = "fn main() { println(42); }";
    let output1 = compile_and_run(source);
    let output2 = compile_and_run(source);
    assert_eq!(output1, output2);
}

#[test]
fn test_artifact_execution_limits() {
    // Limits enforced
    let artifact = create_infinite_loop_artifact();
    let result = execute_with_limit(artifact, 1000);
    assert!(result.error.contains("execution limit exceeded"));
}
```

#### Opcode Registry

**File:** `stdlib/runtime/opcode_registry.cer`

```cerberus
// Authoritative opcode definitions in Cerberus
// This is the source of truth for instruction set v1

type OpcodeInfo = {
  code: i64,
  name: string,
  encoding: string,      // "1" | "1+4" | "1+8" | ...
  description: string,
  introduced_in: string, // "v1.0" | "v1.1" | ...
}

fn get_opcode_info(code: i64) -> option[OpcodeInfo] {
  match code {
    0x00 => Some({ code: 0x00, name: "HALT", ... }),
    0x01 => Some({ code: 0x01, name: "CONST_INT", ... }),
    0x02 => Some({ code: 0x02, name: "CONST_BOOL", ... }),
    // ... all 200 v1 instructions
    _ => None,  // Unknown opcode
  }
}

fn is_opcode_valid(code: i64) -> bool {
  match get_opcode_info(code) {
    Some(_) => true,
    None => false,
  }
}
```

---

## Challenge 4: Internal Representation Overhead

### The Problem

**Current Implementation:**

The compiler uses string-encoded AST nodes:

```cerberus
// In stdlib/compiler.cer
fn parse_expression(tokens: vector[string]) -> string {
  // Returns AST as string like:
  // "BinOp{op:+,left:IntLit{value:42},right:Variable{name:x}}"
  
  // Later, this string is re-parsed multiple times:
  // - In type checker: parse string → extract type info
  // - In codegen: parse string → emit bytecode
  // - In optimizer: parse string → analyze pattern
}
```

**Consequences:**

1. **Repeated Parsing Overhead**
   ```
   Source Code
       ↓
   [Parser: string → string]
       ↓
   String-encoded AST
       ↓
   [Type Checker: parse string]
       ↓
   [Codegen: parse string]
       ↓
   [Optimizer: parse string]
       
   Each stage re-parses the same string!
   ```

2. **No Type Safety**
   ```cerberus
   // AST represented as string
   ast = "FnDecl{name:foo,params:[],body:[]}"
   
   // Typo in parsing breaks silently
   if string_contains(ast, "name:") {
     // Extract name
   }
   // What if the format changes? No compiler help!
   ```

3. **Fragile Refactoring**
   ```cerberus
   // Current: "name:foo"
   // Future: "id:123,name:foo"
   
   // All code that parses strings must be updated
   // Hard to find all places that need changes
   // No compiler verification
   ```

### Root Cause: Lack of Structured Types

Cerberus needs a proper AST type system. Currently, the language doesn't support:
- Algebraic data types (discriminated unions)
- Pattern matching on structured data
- Recursive type definitions

### Solution: Implement Structured AST

#### Step 1: Design AST Type Hierarchy

**File:** `stdlib/compiler/ast.cer`

```cerberus
// Structured AST representation

// Program is a collection of declarations
type Program = {
  items: vector[TopLevelItem],
}

type TopLevelItem =
  | FunctionDecl { fn_decl: FnDecl }
  | ConstantDecl { const_decl: ConstDecl }

type FnDecl = {
  name: string,
  params: vector[Param],
  return_type: string,  // Type name
  body: vector[Statement],
}

type Param = {
  name: string,
  param_type: string,
}

type Statement =
  | LetStatement { let_stmt: LetStmt }
  | ExprStatement { expr_stmt: ExprStmt }
  | ReturnStatement { ret_stmt: RetStmt }
  | IfStatement { if_stmt: IfStmt }
  | WhileStatement { while_stmt: WhileStmt }
  | Block { statements: vector[Statement] }

type LetStmt = {
  name: string,
  value_type: string,
  initial_value: Expr,
}

type ExprStmt = {
  expr: Expr,
}

type RetStmt = {
  value: option[Expr],
}

type IfStmt = {
  condition: Expr,
  then_body: vector[Statement],
  else_body: option[vector[Statement]],
}

type WhileStmt = {
  condition: Expr,
  body: vector[Statement],
}

type Expr =
  | IntLiteral { value: i64 }
  | BoolLiteral { value: bool }
  | StringLiteral { value: string }
  | Variable { name: string }
  | BinaryOp { op: string, left: Expr, right: Expr }
  | UnaryOp { op: string, operand: Expr }
  | Call { name: string, args: vector[Expr] }
  | FieldAccess { object: Expr, field: string }
```

#### Step 2: Update Parser to Emit Structured AST

**Before (Current):**
```cerberus
fn parse_expression() -> string {
  let token = next_token();
  if is_number(token) {
    return "IntLit{" + "value:" + token + "}";
  }
  // Returns string
}
```

**After (Proposed):**
```cerberus
fn parse_expression() -> Expr {
  let token = next_token();
  if is_number(token) {
    // Convert string to i64 safely
    let value: i64 = parse_int(token);
    return IntLiteral { value: value };
  }
  // Returns structured type
}

fn parse_binary_op(left: Expr, op: string) -> Expr {
  let right: Expr = parse_expression();
  return BinaryOp {
    op: op,
    left: left,
    right: right,
  };
}
```

#### Step 3: Update Type Checker

**Before:**
```cerberus
fn check_type(ast_string: string) -> result[string, string] {
  // Parse string to extract type info
  if string_contains(ast_string, "IntLit") {
    return Ok("i64");
  }
  // Fragile string matching
}
```

**After:**
```cerberus
fn check_expr_type(expr: Expr) -> result[string, TypeError] {
  match expr {
    IntLiteral { value: _ } => Ok("i64"),
    BoolLiteral { value: _ } => Ok("bool"),
    StringLiteral { value: _ } => Ok("string"),
    Variable { name: name } => {
      // Look up symbol safely
      match symbol_table.lookup(name) {
        Some(info) => Ok(info.var_type),
        None => Err(error_unknown_symbol(name)),
      }
    },
    BinaryOp { op: op, left: left, right: right } => {
      let left_type: string = check_expr_type(left)?;
      let right_type: string = check_expr_type(right)?;
      
      // Validate operator for types
      match (op, left_type, right_type) {
        ("+", "i64", "i64") => Ok("i64"),
        ("+", "string", "string") => Ok("string"),
        _ => Err(error_type_mismatch(...)),
      }
    },
    // ... more patterns
  }
}
```

#### Step 4: Update Codegen

**Before:**
```cerberus
fn codegen(ast_string: string) -> vector[string] {
  // Parse string again!
  if string_contains(ast_string, "IntLit{value:42}") {
    return vector["const_int 42"];
  }
}
```

**After:**
```cerberus
fn codegen_expr(expr: Expr) -> vector[Instr] {
  match expr {
    IntLiteral { value: val } => vector[ConstInt { value: val }],
    BoolLiteral { value: val } => vector[ConstBool { value: val }],
    StringLiteral { value: val } => vector[ConstStr { value: val }],
    BinaryOp { op: op, left: left, right: right } => {
      let left_code: vector[Instr] = codegen_expr(left);
      let right_code: vector[Instr] = codegen_expr(right);
      
      let op_instr: Instr = match op {
        "+" => Add,
        "-" => Sub,
        "*" => Mul,
        "/" => Div,
        _ => error("Unknown operator"),
      };
      
      return left_code + right_code + vector[op_instr];
    },
    // ... more patterns
  }
}
```

#### Benefits

```
Before (String-based):
  - 3 full string parses per compilation
  - Error handling through string matching
  - Refactoring dangerous (no compiler checks)
  - Performance: O(n) for each parse

After (Structured):
  - 1 parse in parser, direct object passing
  - Compile-time type checking
  - Refactoring safe (compiler catches errors)
  - Performance: O(1) object passing
```

#### Timeline

- **Weeks 1-2**: Design AST types
- **Weeks 3-4**: Update parser to emit AST
- **Weeks 5-6**: Update type checker for AST
- **Weeks 7-8**: Update codegen for AST
- **Weeks 9-10**: Testing and optimization
- **Weeks 11-12**: Documentation

---

## Challenge 5: Module Resolver Complexity

### The Problem

**Current State:**

The module resolver in `stdlib/compiler.cer` is verbose and repetitive:

```cerberus
fn resolve_function(name: string) -> option[FunctionDef] {
  // Search built-in functions
  if name == "println" {
    return Some(builtin_println);
  }
  if name == "vector_new" {
    return Some(builtin_vector_new);
  }
  // ... 50 more built-ins
  
  // Search user-defined functions
  for fn_def in user_functions {
    if fn_def.name == name {
      return Some(fn_def);
    }
  }
  
  // Search stdlib functions
  for fn_def in stdlib_functions {
    if fn_def.name == name {
      return Some(fn_def);
    }
  }
  
  return None;
}

fn resolve_type(name: string) -> option[TypeDef] {
  // Same three-step search pattern!
  // Check built-ins
  if name == "i64" {
    return Some(type_i64);
  }
  // ... etc
  
  // Check user-defined
  for type_def in user_types {
    if type_def.name == name {
      return Some(type_def);
    }
  }
  
  // Check stdlib
  for type_def in stdlib_types {
    if type_def.name == name {
      return Some(type_def);
    }
  }
  
  return None;
}

// And repeat for constants, modules, etc...
fn resolve_constant(name: string) -> option[ConstantDef] { ... }
fn resolve_variable(name: string) -> option[VariableDef] { ... }
fn resolve_module(name: string) -> option[ModuleDef] { ... }
```

**Problems:**

1. **Code Duplication**: Same lookup logic repeated 5+ times
2. **Maintenance Burden**: Change lookup strategy → update all 5 places
3. **Bug Risk**: Easy to miss an update somewhere
4. **Performance**: Inefficient lookups without caching
5. **Extensibility**: Hard to add new symbol types

### Root Cause: No Generic Functions

Cerberus doesn't support:
- Generic/polymorphic functions
- Higher-order functions
- Function parameters as values

### Solution: Generic Resolver Factory

#### Design Pattern: Strategy Pattern

```cerberus
// Define generic symbol resolver

type SymbolTable[T] = {
  builtins: map[string, T],
  user_defined: vector[T],
  stdlib: vector[T],
  get_name: fn(T) -> string,  // How to extract name from T
  cache: map[string, option[T]],
}

fn symbol_table_new[T](
  builtins: map[string, T],
  get_name_fn: fn(T) -> string
) -> SymbolTable[T] {
  return {
    builtins: builtins,
    user_defined: vector_new(),
    stdlib: vector_new(),
    get_name: get_name_fn,
    cache: map_new(),
  };
}

fn symbol_table_lookup[T](
  table: SymbolTable[T],
  name: string
) -> option[T] {
  // Check cache first
  match table.cache.get(name) {
    Some(cached) => return cached,
    None => { /* continue */ }
  }
  
  // Search builtins
  match table.builtins.get(name) {
    Some(item) => {
      table.cache.set(name, Some(item));
      return Some(item);
    },
    None => { /* continue */ }
  }
  
  // Search user-defined
  for item in table.user_defined {
    if table.get_name(item) == name {
      table.cache.set(name, Some(item));
      return Some(item);
    }
  }
  
  // Search stdlib
  for item in table.stdlib {
    if table.get_name(item) == name {
      table.cache.set(name, Some(item));
      return Some(item);
    }
  }
  
  // Not found
  table.cache.set(name, None);
  return None;
}

fn symbol_table_add_user[T](
  table: SymbolTable[T],
  item: T
) {
  vector_push(table.user_defined, item);
  // Invalidate cache
  table.cache.clear();
}
```

#### Usage Example

**Before (50 lines for each symbol type):**
```cerberus
fn resolve_function(name: string) -> option[FunctionDef] {
  // Explicit code...
}

fn resolve_type(name: string) -> option[TypeDef] {
  // Same code...
}
```

**After (Generic, 10 lines total):**
```cerberus
// Create specialized resolvers
let fn_table: SymbolTable[FunctionDef] = symbol_table_new(
  builtin_functions,
  fn(def: FunctionDef) -> string { def.name }
);

let type_table: SymbolTable[TypeDef] = symbol_table_new(
  builtin_types,
  fn(def: TypeDef) -> string { def.name }
);

// Use uniform interface
fn resolve_symbol[T](
  table: SymbolTable[T],
  name: string
) -> option[T] {
  return symbol_table_lookup(table, name);
}

// In type checker:
fn check_variable(name: string) -> result[string, TypeError] {
  match resolve_symbol(var_table, name) {
    Some(var_def) => Ok(var_def.var_type),
    None => Err(error_unknown_symbol(name)),
  }
}
```

#### Benefits

| Aspect | Before | After |
|--------|--------|-------|
| **Lines of Code** | 200+ | 50 |
| **Maintenance** | 5 functions to update | 1 generic function |
| **Caching** | Manual per-function | Automatic |
| **Consistency** | Prone to errors | Guaranteed |
| **Extensibility** | Add new type → 50 lines | Just provide name extractor |

#### Implementation Steps

**Phase 1: Design (Days 1-2)**
- [ ] Create `SymbolTable[T]` type
- [ ] Implement generic lookup function
- [ ] Design name extractor interface

**Phase 2: Refactor (Days 3-7)**
- [ ] Update `resolve_function()`
- [ ] Update `resolve_type()`
- [ ] Update `resolve_variable()`
- [ ] Update `resolve_constant()`
- [ ] Add caching layer

**Phase 3: Testing (Days 8-9)**
- [ ] Unit tests for generic resolver
- [ ] Integration tests with compiler
- [ ] Performance benchmarks

**Phase 4: Documentation (Days 10)**
- [ ] Update stdlib documentation
- [ ] Add code examples
- [ ] Performance analysis

---

## Cross-Cutting Issues

These issues affect multiple challenges:

### Issue A: Error Handling & Reporting

**Problem:** Errors scattered throughout pipeline, hard to track source.

**Solution:** Unified error type with source location:
```cerberus
type CompilerError = {
  message: string,
  location: SourceLocation,
  kind: ErrorKind,
  suggestions: vector[string],
}
```

See `ERROR_MESSAGE_SYSTEM.md` for details.

### Issue B: Performance Measurements

**Problem:** Don't know which parts are slow.

**Solution:** Add benchmarking infrastructure:
```bash
# Time each pipeline stage
cerberus-compiler compile --benchmark program.cer

# Output:
# Lexer:      12.5ms
# Parser:     45.3ms
# TypeCheck:  28.1ms
# Codegen:    15.2ms
# Total:     101.1ms
```

### Issue C: Testing Strategy

**Problem:** Hard to test self-hosted compiler.

**Solution:** Multi-level testing:
```
1. Unit tests (test individual functions)
2. Integration tests (test compilation pipeline)
3. Conformance tests (test bytecode compatibility)
4. Regression tests (catch performance degradation)
5. Bootstrap tests (verify self-hosting works)
```

---

## Independence Roadmap

### Phase A: Contract Finalization (Months 1-2)

**Goal:** Freeze artifact format and ensure self-host path is solid

**Deliverables:**
- ✅ `docs/ARTIFACT_SPEC_v1.md` - Formal specification
- ✅ `docs/CONFORMANCE_SUITE.md` - Test strategy
- ✅ Conformance test suite (5+ tests)
- ✅ Simplified module resolver
- ✅ Error message system design

**Success Criteria:**
- Self-host compilation deterministic
- Artifact format stable
- All conformance tests pass

### Phase B: Hardening & Optimization (Months 3-5)

**Goal:** Ensure compiler robustness and performance

**Deliverables:**
- [ ] Structured AST implementation
- [ ] Performance benchmarks
- [ ] Regression test suite
- [ ] Documentation updates

**Success Criteria:**
- Compiler handles all test cases
- Performance within target (compile 1000 LOC in <100ms)
- No regressions vs Phase A

### Phase C: Minimal Launcher (Months 6-8)

**Goal:** Create standalone distribution

**Deliverables:**
- [ ] C launcher (< 500 lines)
- [ ] Precompiled runtime artifact
- [ ] Distribution package
- [ ] Installation guide

**Success Criteria:**
- Launcher compiles on Linux/macOS/Windows
- All tests pass with launcher
- Distribution < 10MB

### Phase D: Independence (Months 9-12)

**Goal:** Remove Rust dependency

**Deliverables:**
- [ ] Rust-free build pipeline
- [ ] v1.0 standalone release
- [ ] Community contribution guide
- [ ] Compatibility guarantee

**Success Criteria:**
- No Rust in critical path
- Users don't need Rust installed
- Can compile Cerberus with Cerberus

---

## Success Metrics

### Metric 1: Compilation Determinism

```bash
for i in {1..10}; do
  ./cerberus compile test.cer -o /tmp/out$i.crt
done

# All /tmp/out*.crt files are identical
md5sum /tmp/out*.crt | sort | uniq | wc -l
# Should output: 1
```

**Target:** 100% determinism in all tests

### Metric 2: Conformance Test Coverage

```rust
#[test]
fn conformance_suite_comprehensive() {
    let tests = load_conformance_tests();
    assert!(tests.len() > 50);  // At least 50 test cases
    
    for test in tests {
        let result = run_test(test);
        assert!(result.passed, "Test failed: {}", test.name);
    }
}
```

**Target:** 50+ conformance tests, 100% pass rate

### Metric 3: Bootstrap Self-Hosting

```bash
# Can compile compiler with compiler?
./cerberus compile stdlib/compiler.cer -o compiler2.crt

# Do they produce same output?
./cerberus run compiler2.crt test.cer > out1.crt
./cerberus run ./compiler.crt test.cer > out2.crt

md5sum out1.crt out2.crt
# Hashes should match
```

**Target:** Fully bootstrapping compiler

### Metric 4: Distribution Size

```bash
ls -lh cerberus-1.0-linux.tar.gz
# cerberus-1.0-linux.tar.gz: < 10MB

# Contents:
# - launcher: ~50KB
# - cerberus_runtime.crt: ~100KB
# - stdlib/: ~200KB
# - docs/: ~500KB
```

**Target:** < 10MB total distribution

### Metric 5: Dependency Freedom

```bash
# Run without Rust
which rustc
# Should output: not found

./cerberus compile hello.cer -o hello.crt
./cerberus run hello.crt
# Works!
```

**Target:** Zero Rust dependencies in production

---

## Conclusion

Cerberus faces 5 major technical challenges preventing independence:

| Challenge | Root Cause | Solution | Timeline |
|-----------|-----------|----------|----------|
| Rust Dependency | VM in Rust | Minimal C launcher | 8 months |
| Bootstrap Paradox | Circular deps | Precompiled artifacts | 6 months |
| Artifact Format | No spec | Formal v1 specification | 1 month |
| String-based AST | No structured types | Implement AST hierarchy | 3 months |
| Resolver Complexity | Code duplication | Generic factory pattern | 1 month |

**Overall Timeline:** 12 months to full independence

**Key Insight:** Challenges are well-understood and solvable. No fundamental blockers. Just disciplined engineering work following the roadmap.

**Next Step:** Execute Phase A (Contract Finalization) in next 2 months to establish solid foundation.

---

**Document Version:** 1.0  
**Status:** Ready for team review and implementation planning  
**Last Updated:** 2026-04-29
