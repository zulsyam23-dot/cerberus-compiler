# Cerberus Compiler: Problem Diagnosis & Solutions Guide

**Last Updated:** 2026-04-29  
**Status:** Active Development Phase A

---

## Table of Contents

1. [Current Critical Issues](#current-critical-issues)
2. [Root Cause Analysis](#root-cause-analysis)
3. [Solution Roadmap](#solution-roadmap)
4. [Implementation Priorities](#implementation-priorities)
5. [Technical Debt](#technical-debt)

---

## Current Critical Issues

### Issue 1: Rust Dependency - The Primary Blocker

**Severity:** 🔴 CRITICAL  
**Impact:** Blocks full operational independence

#### Problem Statement
Cerberus is a self-hosted compiler, but its execution still depends on a Rust VM host. This means:
- Compiler pipeline (lexer, parser, typecheck, codegen) runs **in Cerberus** but executes **through Rust**
- CLI toolchain remains a Rust binary
- Cannot achieve true standalone distribution without Rust ecosystem

#### Current Architecture
```
Input Code (.cer)
    ↓
[Self-Hosted Compiler in Cerberus] ← executes via Rust VM host
    ↓
VM Script / Artifact (.crt)
    ↓
[VM Engine in Rust] ← execution dependency
    ↓
Output
```

#### Why This Is Hard
1. **Bootstrapping Paradox**: To run a Cerberus compiler without Rust, you need a Cerberus runtime first
2. **Chicken-and-Egg**: VM implementation (Rust) provides bytecode interpretation that Cerberus code depends on
3. **Instruction Set Churn**: Every change to VM semantics requires synchronizing both Rust and Cerberus implementations

#### Solutions

**Phase B1: Fallback-Only Strategy** (Recommended Immediate)
```
Input → Self-Hosted Compiler → 
  ✅ Use Cerberus/VM path if available
  ❌ Fall back to Rust native only if self-host fails
```

**Milestones:**
- [ ] Stabilize artifact format `cerberus_toolchain_v1`
- [ ] Ensure all self-host paths produce valid bytecode
- [ ] Make Rust toolchain optional for distribution
- [ ] Build minimal native launcher (C/ASM only)

**Phase C: Standalone Launcher** (6-12 months)
```
Minimal launcher (compiled C/ASM) 
  → loads pre-compiled Cerberus runtime from artifact
  → executes Cerberus compiler from stdlib
```

---

### Issue 2: Internal Representation Overhead

**Severity:** 🟠 HIGH  
**Impact:** Runtime performance, maintenance burden

#### Problem Statement
The compiler pipeline heavily relies on string-encoded AST nodes:

```rust
// Current approach (inefficient)
ast_node = "fn main() { let x = 42; println(x); }"  // parsing as string
parsed = parse_string(ast_node)                      // repeated parsing
codegen = generate_code(parsed)                      // multiple re-parses
```

#### Consequences
- **Higher Runtime Overhead**: String parsing happens multiple times in the pipeline
- **Fragile Refactoring**: Changes to string format break downstream code
- **Harder Debugging**: No type safety, errors appear late

#### Solution: Structured AST Representation

**Create Internal AST Types** in `stdlib/compiler/ast.cer`:
```cerberus
// Pseudocode for Cerberus AST module
type AstNode = 
  | FnDecl { name: string, params: vector[Param], body: vector[Statement] }
  | LetStmt { name: string, value: AstExpr }
  | ExprStmt { expr: AstExpr }
  | ...

type AstExpr =
  | IntLit { value: i64 }
  | StrLit { value: string }
  | BinOp { op: string, left: AstExpr, right: AstExpr }
  | Call { func: string, args: vector[AstExpr] }
  | ...
```

**Benefits:**
- ✅ Type-safe representation
- ✅ Zero-copy passing between pipeline stages
- ✅ Easier optimization and refactoring
- ✅ Better error messages with source location tracking

**Timeline:** Implement incrementally in Phase B

---

### Issue 3: Module Resolver Complexity

**Severity:** 🟠 HIGH  
**Impact:** Maintenance burden, bug risk

#### Problem Statement
The module resolver in `stdlib/compiler.cer` is verbose and repetitive:
- Same resolution logic duplicated for different symbol types
- Limited error context for missing symbols
- No caching of resolution results

#### Current Pattern
```cerberus
// Repetitive pattern across resolver
fn resolve_fn(name: string) -> option[FnDecl] {
  // search built-ins
  // search user-defined
  // search stdlib
  // return option
}

fn resolve_type(name: string) -> option[TypeDef] {
  // same three-step search
  // same boilerplate
}
```

#### Solution: Generic Resolver Factory

```cerberus
// DRY resolver pattern
fn make_resolver[T](name: string, tables: vector[Table[T]]) -> option[T] {
  for table in tables {
    match table.get(name) {
      Some(item) => return Some(item),
      None => continue,
    }
  }
  return None;
}

// Usage
built_in_fns = load_builtin_functions();
user_fns = symbol_table.functions;
stdlib_fns = stdlib.functions;

resolve_fn = |name| make_resolver(name, vector[built_in_fns, user_fns, stdlib_fns]);
```

**Benefits:**
- ✅ Single source of truth for resolution logic
- ✅ Easier to add new symbol tables (builtins, imports)
- ✅ Better error reporting with lookup path
- ✅ Caching friendly

**Timeline:** Implement in Phase A (short-term win)

---

### Issue 4: Artifact Contract Fragility

**Severity:** 🟠 HIGH  
**Impact:** Breaking changes, version hell

#### Problem Statement
The `cerberus_toolchain_v1` format exists but compatibility policy is not formal:
```
Current: cerberus_toolchain_v1;vm_text_script;entry=main::code::...
Unknown: What happens when we add new instructions?
Risk: v1 artifacts break with next instruction set change
```

#### Issues
1. **No versioning for instruction sets**: Is `v1` tied to a specific opcode set?
2. **No deprecation path**: Old instructions become obsolete with no migration
3. **No compatibility testing**: No conformance suite validates artifact loading

#### Solution: Formal Artifact Specification

**Create:** `docs/ARTIFACT_SPEC.md`

```markdown
# Cerberus Artifact Format v1 Specification

## Format
cerberus_toolchain_v1;vm_text_script;[metadata]::code::[payload]

## Versioning Policy
- Major version (v1, v2, ...): Breaking changes to instruction set
- Instruction opcodes FROZEN for v1 (0-255 reserved)
- New instructions in v2 start at opcode 256

## Compatibility Guarantee
- v1 artifacts execute unchanged on v1+ runtimes
- Fallback behavior if unknown instruction encountered

## Metadata
- entry=<label>: Entry point
- limit_steps=<n>: Max execution steps
- limit_stack=<n>: Max stack depth
- limit_call=<n>: Max call depth

## Opcode Registry v1
[Complete list with semantics]
```

**Conformance Testing:**

```rust
#[test]
fn test_artifact_v1_forward_compat() {
    // Load v1 artifact on current runtime
    // Assert: executes with same output
}

#[test]
fn test_unknown_instruction_handling() {
    // Create artifact with unknown opcode
    // Assert: graceful error or deprecation message
}
```

---

## Root Cause Analysis

### Why is Cerberus "Rigid"?

The language feels strict/rigid because of design decisions made for **correctness and safety**:

#### 1. Static Type System (No Type Inference)
```cerberus
let x = 42;           // ❌ Type inferred? No explicit type
let x: i64 = 42;      // ✅ Explicit type required
let y: vector[i64] = [1, 2, 3];  // ✅ Generic types explicit
```

**Why:** 
- Eliminates ambiguity during type checking
- Makes compilation deterministic
- Enables bytecode generation without runtime type checks

**Trade-off:**
- More verbose than Python/JavaScript
- But catches errors at compile-time vs runtime

#### 2. No Implicit Type Coercion
```cerberus
let x: i64 = 42;
let s: string = x;    // ❌ Error: cannot convert i64 to string
let s: string = int_to_string(x);  // ✅ Explicit conversion
```

**Why:**
- Prevents hidden bugs (e.g., `"5" + 3` should not equal "8")
- Makes performance predictable
- Forces developers to understand data flow

#### 3. Mandatory Error Handling with Option/Result
```cerberus
fn read_file(path: string) -> result[string, string] {
    // Must return result, never null
}

match read_file("data.txt") {
    Ok(content) => { ... },
    Err(msg) => { ... },  // ❌ Cannot ignore error
}
```

**Why:**
- No null pointer exceptions
- Errors are explicit in function signature
- Forces caller to handle failures

#### 4. No Operator Overloading
```cerberus
let s1 = "Hello";
let s2 = "World";
let result = s1 + s2;     // ❌ String concat not overloaded
let result = string_concat(s1, s2);  // ✅ Explicit function call
```

**Why:**
- Avoids confusion (does `+` mean numeric add or string concat?)
- Makes code intent explicit
- Simplifies compiler implementation

#### 5. Memory Model (Stack-Only, No GC)
```cerberus
let x = 42;        // Stack allocation
let v = [1,2,3];   // Vector (stack, size inline)
// No heap allocation, no garbage collection
// ❌ Cannot create memory cycles
// ✅ Predictable cleanup
```

**Why:**
- Embedded systems compatibility
- Deterministic performance (no GC pauses)
- Simpler runtime

#### 6. No Global Mutable State
```cerberus
// ❌ Not allowed
let mut COUNTER = 0;
fn increment() {
    COUNTER = COUNTER + 1;  // Global mutation = hard to reason about
}

// ✅ Recommended pattern
fn increment(counter: i64) -> i64 {
    counter + 1
}
```

**Why:**
- Easier to reason about program behavior
- Facilitates parallelization
- Reduces debugging complexity

---

## Perceived "Rigidity" vs. Safety

| Feature | Cerberus | Python | JavaScript |
|---------|----------|--------|------------|
| Type Safety | Static (strict) | Dynamic | Dynamic |
| Null Safety | No nulls (Result/Option) | Nulls exist | Nulls exist |
| Error Handling | Explicit (Result) | Exceptions | Try/Catch |
| Type Coercion | None | Implicit | Implicit |
| Memory Management | Stack-only | GC | GC |
| Verbosity | High | Low | Low |

**Rigidity is a Feature**, not a Bug:
- ✅ Catches errors at compile-time
- ✅ Enables optimization
- ✅ Makes code reviews easier
- ✅ Supports embedded and systems programming

---

## Solution Roadmap

### Timeline: 12-Month Plan to Independence

#### **Phase A: Contract Finalization** (Months 1-2, Current)
**Goal:** Stabilize artifact format and runtime contract

- [ ] Freeze `cerberus_toolchain_v1` format
- [ ] Document compatibility policy
- [ ] Create conformance test suite
- [ ] Implement module resolver simplification
- [ ] Add structured AST types (start)

**Deliverables:**
- `docs/ARTIFACT_SPEC.md` (formal specification)
- `docs/CONFORMANCE_SUITE.md` (test strategy)
- Simplified resolver in `stdlib/compiler.cer`

---

#### **Phase B: Hardening & Optimization** (Months 3-5)
**Goal:** Ensure runtime reliability and performance

- [ ] Complete structured AST in Cerberus
- [ ] Add runtime performance benchmarks
- [ ] Implement instruction-level caching
- [ ] Create comprehensive regression test suite
- [ ] Optimize hot paths in codegen

**Deliverables:**
- Structured AST module
- Benchmark suite with baseline metrics
- Regression test automation

---

#### **Phase C: Minimal Fallback Mode** (Months 6-8)
**Goal:** Make Rust toolchain optional

- [ ] Build minimal C/ASM launcher (1000 LOC)
- [ ] Package pre-compiled Cerberus runtime
- [ ] Create distribution bundle
- [ ] Switch default to self-host mode
- [ ] Deprecate Rust compiler (fallback-only)

**Deliverables:**
- Standalone launcher
- Installation/distribution docs
- Backward compatibility layer

---

#### **Phase D: Standalone Toolchain** (Months 9-12)
**Goal:** Pure Cerberus distribution

- [ ] Remove Rust build from critical path
- [ ] Create Cerberus-only distribution
- [ ] Establish compatibility guarantee
- [ ] Build community contribution model

**Deliverables:**
- v1.0 standalone release
- Community contribution guidelines

---

## Implementation Priorities

### Quick Wins (Weeks 1-4)

1. **Module Resolver Refactor** (~2 days)
   - Create `make_resolver` generic function
   - Reduce `stdlib/compiler.cer` by 30%
   - Add resolver caching

2. **Artifact Specification** (~3 days)
   - Write `docs/ARTIFACT_SPEC.md`
   - Create conformance test suite
   - Document opcodes and semantics

3. **Error Message Improvement** (~2 days)
   - Add source location tracking to errors
   - Include resolver lookup path in diagnostics

### Medium-Term (Weeks 5-12)

4. **Structured AST Implementation** (~4 weeks)
   - Design type hierarchy in Cerberus
   - Update parser to emit structured AST
   - Refactor codegen to use AST
   - Add AST validation pass

5. **Performance Baseline** (~2 weeks)
   - Create benchmark suite
   - Establish compile-time targets
   - Establish runtime performance targets
   - Document optimization opportunities

### Long-Term (Months 4+)

6. **Minimal Launcher** (~3 weeks)
   - Design C/ASM launcher
   - Package Cerberus runtime artifact
   - Create automated build pipeline

---

## Technical Debt

### High Priority

| Item | Impact | Effort | Blocker? |
|------|--------|--------|----------|
| Structured AST | Performance, maintainability | Medium | No |
| Module resolver | Maintainability | Low | No |
| Artifact spec | Compatibility | Low | No |
| Benchmark suite | Optimization | Low | No |

### Medium Priority

| Item | Impact | Effort | Blocker? |
|------|--------|--------|----------|
| Rust elimination | Distribution | High | No |
| Conformance tests | Reliability | Medium | No |
| Documentation gaps | Usability | Medium | No |

### Low Priority (Can defer)

| Item | Impact | Effort | Blocker? |
|------|--------|--------|----------|
| Operator overloading | Nice-to-have | High | No |
| GC support | Nice-to-have | High | No |
| JIT compilation | Performance | High | No |

---

## Addressing the "Rigidity" Perception

### What Developers Actually Want

**"Why is Cerberus so strict?"**

Answer:
> Cerberus prioritizes **correctness** over convenience. The strict type system, mandatory error handling, and explicit conversions mean:
> - 90% fewer runtime surprises
> - Compile-time error detection
> - Easier code review and maintenance
> - Predictable performance

### Mitigation Strategies

1. **Better Error Messages**
   ```
   ❌ Error: Cannot add string to int
   
   ✅ Error: Type mismatch in binary operation
      Left operand: string = "hello"
      Right operand: i64 = 42
      Suggestion: use string_concat() for strings or int_to_string() before +
   ```

2. **Smart Defaults with Explicit Escape Hatches**
   ```cerberus
   // Recommended: explicit
   let s: string = int_to_string(x);
   
   // Also allowed: built-in coercion for println (UI only)
   println(x);  // Automatically converts to string for display
   ```

3. **Comprehensive Standard Library**
   ```cerberus
   // Make common patterns easy
   int_to_string(x)
   string_to_int(s)
   vector_map(v, f)
   vector_filter(v, predicate)
   string_split(s, delimiter)
   ```

4. **Clear Documentation and Tutorials**
   - Explain the *why* behind design choices
   - Show migration paths from dynamic languages
   - Provide cookbook of common patterns

---

## Next Steps

1. **Review & Prioritize** (This week)
   - [ ] Team consensus on Phase A priorities
   - [ ] Assign owners to each workstream
   - [ ] Create tracking issues

2. **Begin Phase A** (Next 2 weeks)
   - [ ] Start module resolver refactor
   - [ ] Draft artifact specification
   - [ ] Create conformance test skeleton

3. **Establish Metrics** (Ongoing)
   - [ ] Track artifact spec completeness
   - [ ] Monitor test coverage growth
   - [ ] Measure compilation time trends

---

**Document Status:** Ready for team review and implementation planning
