# Cerberus Programming: Beginner to Expert Guide

**Language:** Cerberus (Self-Hosted Compiler)  
**Audience:** Complete beginners to advanced compiler development  
**Last Updated:** 2026-04-29

---

## Table of Contents

### Beginner Level (1-2 weeks)
1. [Quick Start](#quick-start)
2. [Basic Syntax](#basic-syntax)
3. [Types & Variables](#types--variables)
4. [Control Flow](#control-flow)
5. [Functions](#functions)

### Intermediate Level (3-4 weeks)
6. [Collections](#collections)
7. [Error Handling](#error-handling)
8. [Pattern Matching](#pattern-matching)
9. [Modules & Organization](#modules--organization)

### Advanced Level (5-8 weeks)
10. [Type System Deep Dive](#type-system-deep-dive)
11. [Bytecode & VM Integration](#bytecode--vm-integration)
12. [Compiler Internals](#compiler-internals)
13. [Performance Optimization](#performance-optimization)

### Expert Level (8+ weeks)
14. [Contributing to Cerberus Core](#contributing-to-cerberus-core)
15. [AST Manipulation](#ast-manipulation)
16. [Custom Instruction Implementation](#custom-instruction-implementation)

---

## BEGINNER LEVEL

---

## Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/zulsyam23-dot/cerberus-compiler
cd cerberus-compiler

# Build with Rust (required for now)
cargo build --release

# Verify installation
./target/release/cerberus-compiler --version
```

### Your First Program

**File:** `hello.cer`
```cerberus
fn main() {
  println("Hello, Cerberus!");
}
```

**Compile and run:**
```bash
./target/release/cerberus-compiler compile hello.cer -o hello.crt
./target/release/cerberus-compiler run hello.crt
```

**Output:**
```
Hello, Cerberus!
```

### Program Structure

Every Cerberus program needs a `main` function as the entry point:

```cerberus
fn main() {
  // Your code here
}
```

---

## Basic Syntax

### Comments

```cerberus
// Single-line comment

/*
  Multi-line comment
  spans multiple lines
*/
```

### Printing Output

```cerberus
println("Simple string");
println(42);
println(true);
```

**Note:** `println` is the primary way to output values. There's no `print` without newline yet.

### Basic Operations

```cerberus
// Arithmetic
let sum = 5 + 3;
let diff = 10 - 4;
let product = 6 * 7;
let quotient = 20 / 4;

// Comparison
let is_greater = 5 > 3;        // true
let is_equal = 5 == 5;         // true
let is_not_equal = 5 != 3;     // true

// Logical
let and_result = true && false; // false
let or_result = true || false;  // true
let not_result = !true;         // false
```

---

## Types & Variables

### Declaring Variables

```cerberus
// ✅ Correct: Type must be explicit
let x: i64 = 42;
let name: string = "Alice";
let is_active: bool = true;

// ❌ Incorrect: Type inference not supported
let y = 42;  // Error: type not specified
```

### Supported Types

| Type | Example | Notes |
|------|---------|-------|
| `i64` | `42` | 64-bit signed integer |
| `bool` | `true`, `false` | Boolean |
| `string` | `"Hello"` | Text string |
| `vector[T]` | `[1, 2, 3]` | Dynamic array |
| `option[T]` | `Some(42)` or `None` | Nullable value |
| `result[T, E]` | `Ok(42)` or `Err("error")` | Result with error |

### Type Naming Convention

```cerberus
// Primitive types (lowercase)
let count: i64 = 10;
let message: string = "Hello";

// Generic types (lowercase with brackets)
let numbers: vector[i64] = [1, 2, 3];
let maybe_value: option[i64] = Some(42);
let computation: result[string, string] = Ok("success");
```

### Constants vs Variables

Currently, Cerberus doesn't support `const` declarations. All variables are:
```cerberus
let x: i64 = 42;  // Assignment works once per scope
```

---

## Control Flow

### If-Else Statements

```cerberus
fn main() {
  let age: i64 = 18;
  
  if age >= 18 {
    println("You are an adult");
  } else {
    println("You are a minor");
  }
}
```

### If-Else If-Else Chain

```cerberus
fn main() {
  let score: i64 = 85;
  
  if score >= 90 {
    println("Grade: A");
  } else if score >= 80 {
    println("Grade: B");
  } else if score >= 70 {
    println("Grade: C");
  } else {
    println("Grade: F");
  }
}
```

### While Loops

```cerberus
fn main() {
  let counter: i64 = 0;
  
  while counter < 5 {
    println(counter);
    counter = counter + 1;
  }
}
```

**Note:** Cerberus doesn't have `for` loops yet. Use `while` for iteration.

---

## Functions

### Basic Function Definition

```cerberus
fn greet(name: string) -> string {
  let greeting: string = "Hello, " + name;
  return greeting;
}

fn main() {
  let result: string = greet("Alice");
  println(result);  // Output: Hello, Alice
}
```

### Return Types

```cerberus
// Function returning i64
fn add(a: i64, b: i64) -> i64 {
  return a + b;
}

// Function returning bool
fn is_even(n: i64) -> bool {
  let remainder: i64 = n - (n / 2) * 2;
  return remainder == 0;
}

// Function returning string
fn full_name(first: string, last: string) -> string {
  return first + " " + last;
}
```

### Functions with No Return Value

```cerberus
fn announce() {
  println("Hello from announce!");
}

fn main() {
  announce();
}
```

### Function Parameters

```cerberus
// Multiple parameters
fn multiply(a: i64, b: i64) -> i64 {
  return a * b;
}

// No parameters
fn get_constant() -> i64 {
  return 42;
}

// String parameters
fn repeat(text: string, count: i64) -> string {
  // Placeholder: string repetition not yet implemented
  return text;
}
```

---

## INTERMEDIATE LEVEL

---

## Collections

### Vectors (Dynamic Arrays)

```cerberus
fn main() {
  // Create empty vector
  let numbers: vector[i64] = vector_new();
  
  // Or with initial values (using literals)
  let values: vector[i64] = [1, 2, 3, 4, 5];
  
  // Get length
  let len: i64 = vector_len(values);
  println(len);  // Output: 5
  
  // Get element
  let first: i64 = vector_get(values, 0);
  println(first);  // Output: 1
}
```

### Vector Operations

```cerberus
fn main() {
  let nums: vector[i64] = [10, 20, 30];
  
  // Push element
  vector_push(nums, 40);
  
  // Pop element
  let last: i64 = vector_pop(nums);
  
  // Set element
  vector_set(nums, 0, 99);
  
  // Clear all
  vector_clear(nums);
  
  // Get last element
  let end: i64 = vector_last(nums);
}
```

### Maps (Dictionaries)

```cerberus
fn main() {
  // Create empty map
  let person: map[string, string] = map_new();
  
  // Set values
  map_set(person, "name", "Alice");
  map_set(person, "age", "30");
  
  // Get value
  let name: string = map_get(person, "name");
  println(name);  // Output: Alice
  
  // Check if key exists
  let has_name: bool = map_has(person, "name");
  
  // Get length
  let size: i64 = map_len(person);
  
  // Remove key
  map_remove(person, "age");
}
```

### Sets

```cerberus
fn main() {
  let unique: set[string] = set_new();
  
  // Add elements
  set_add(unique, "apple");
  set_add(unique, "banana");
  set_add(unique, "apple");  // Duplicate, won't be added
  
  // Check membership
  let has_apple: bool = set_has(unique, "apple");
  
  // Get size
  let count: i64 = set_len(unique);
  
  // Remove element
  set_remove(unique, "banana");
}
```

### Stacks

```cerberus
fn main() {
  let stack: stack[i64] = stack_new();
  
  // Push items
  stack_push(stack, 10);
  stack_push(stack, 20);
  stack_push(stack, 30);
  
  // Check top
  let top: i64 = stack_top(stack);
  println(top);  // Output: 30
  
  // Pop item
  let popped: i64 = stack_pop(stack);
  
  // Get length
  let size: i64 = stack_len(stack);
}
```

---

## Error Handling

### Option Type (Nullable Values)

```cerberus
fn find_user(id: i64) -> option[string] {
  if id == 1 {
    return Some("Alice");
  } else {
    return None;
  }
}

fn main() {
  let user: option[string] = find_user(1);
  
  // Must explicitly handle Some/None
  match user {
    Some(name) => println("Found: " + name),
    None => println("User not found"),
  }
}
```

### Option Helper Functions

```cerberus
fn main() {
  let maybe_value: option[i64] = Some(42);
  
  // Check if Some
  let is_some: bool = opt_is_some(maybe_value);
  
  // Get value or default
  let value: i64 = opt_unwrap_or(maybe_value, 0);
  
  // Unwrap (crashes if None)
  let actual_value: i64 = opt_unwrap(maybe_value);
}
```

### Result Type (Error Handling)

```cerberus
fn divide(a: i64, b: i64) -> result[i64, string] {
  if b == 0 {
    return Err("Division by zero");
  } else {
    return Ok(a / b);
  }
}

fn main() {
  let result: result[i64, string] = divide(10, 2);
  
  // Handle success or error
  match result {
    Ok(value) => println("Result: " + value),
    Err(msg) => println("Error: " + msg),
  }
}
```

### Result Helper Functions

```cerberus
fn main() {
  let result: result[i64, string] = Ok(42);
  
  // Check if Ok
  let is_ok: bool = res_is_ok(result);
  
  // Get value or default
  let value: i64 = res_unwrap_or(result, 0);
  
  // Get error value
  let error: string = res_unwrap_err(result);
}
```

---

## Pattern Matching

### Basic Match Expressions

```cerberus
fn main() {
  let status: i64 = 200;
  
  match status {
    200 => println("OK"),
    404 => println("Not Found"),
    500 => println("Server Error"),
    _ => println("Unknown status"),  // Default case
  }
}
```

### Matching Options

```cerberus
fn main() {
  let maybe_name: option[string] = Some("Bob");
  
  match maybe_name {
    Some(name) => println("Name: " + name),
    None => println("No name provided"),
  }
}
```

### Matching Results

```cerberus
fn main() {
  let operation: result[i64, string] = Ok(100);
  
  match operation {
    Ok(value) => println("Success: " + value),
    Err(error) => println("Failed: " + error),
  }
}
```

---

## Modules & Organization

### Module Structure

```
my_project/
├── main.cer           # Entry point with fn main()
├── math.cer           # Math utilities
├── string_utils.cer   # String functions
└── types.cer          # Custom type definitions
```

### Importing Modules

```cerberus
// In main.cer
// Include other files (pseudo-syntax, implementation may vary)
fn add(a: i64, b: i64) -> i64 {
  return a + b;
}

fn subtract(a: i64, b: i64) -> i64 {
  return a - b;
}
```

### Standard Library Functions

Cerberus provides built-in functions:

**String Operations:**
```cerberus
str_len(s: string) -> i64
str_concat(s1: string, s2: string) -> string
str_substr(s: string, start: i64, len: i64) -> string
str_replace(s: string, old: string, new: string) -> string
```

**Environment & System:**
```cerberus
env_get(var: string) -> option[string]
env_has(var: string) -> bool
cwd() -> string                              // Current working directory
path_join(a: string, b: string) -> string
fs_exists(path: string) -> bool
fs_listdir(path: string) -> vector[string]
now_timestamp() -> i64                       // Unix timestamp
sleep_ms(ms: i64) -> unit                    // Sleep milliseconds
```

---

## ADVANCED LEVEL

---

## Type System Deep Dive

### Generic Types

```cerberus
// Vector is generic over its element type
let ints: vector[i64] = [1, 2, 3];
let strings: vector[string] = ["a", "b", "c"];

// Option is generic
let maybe_int: option[i64] = Some(42);
let maybe_str: option[string] = Some("hello");

// Result has two type parameters
let ok_result: result[i64, string] = Ok(100);
let err_result: result[i64, string] = Err("failed");
```

### Type Aliases (Pseudo-syntax)

```cerberus
// Define a type alias for clarity
// type UserId = i64;
// type UserName = string;
// type User = (UserId, UserName);

// Then use in functions
fn get_user_name(id: i64) -> option[string] {
  // Implementation
  return None;
}
```

### Union Types via Result/Option

```cerberus
// Instead of exceptions, use Result
fn parse_int(s: string) -> result[i64, string] {
  // Success case
  // Error case: Err("invalid number")
  return Ok(0);
}

// Instead of null, use Option
fn find_item(id: i64) -> option[string] {
  // Some(item)
  // None (not found)
  return None;
}
```

### Type Safety Benefits

```cerberus
// ❌ This is impossible
let x: i64 = "hello";  // Compiler error: type mismatch

// ❌ This is impossible
let y: option[string] = "hello";  // Must wrap in Some()
let y: option[string] = Some("hello");  // ✅ Correct

// ❌ This is impossible
let z: result[i64, string] = 42;  // Must wrap in Ok()
let z: result[i64, string] = Ok(42);  // ✅ Correct
```

---

## Bytecode & VM Integration

### Understanding Bytecode

Cerberus compiles to bytecode that runs on a VM:

```cerberus
// High-level source
fn double(x: i64) -> i64 {
  return x * 2;
}
```

Compiles to bytecode (simplified):
```
label double
load 0           # Load parameter x
const_int 2      # Load constant 2
mul              # Multiply
ret_val          # Return value
```

### Bytecode Artifact Format

```
cerberus_toolchain_v1;vm_text_script;entry=main;limit_steps=100000::code::
label main
const_int 42
println
halt
```

**Components:**
- `cerberus_toolchain_v1` - Format version
- `vm_text_script` - Artifact type
- `entry=main` - Entry point label
- `limit_steps=100000` - Execution limit
- `::code::` - Separator
- Instructions follow

### Viewing Generated Bytecode

```bash
# Compile to see bytecode
cerberus-compiler compile program.cer -o program.crt

# Inspect artifact
cat program.crt
```

### VM Instructions

Common instruction categories:

**Arithmetic:**
```
const_int <value>
const_bool <value>
const_str <value>
add, sub, mul, div
```

**Control Flow:**
```
jump <label>
jump_if_false <label>
label <name>
halt
```

**Stack Operations:**
```
load <var_index>
store <var_index>
println
```

**Collections:**
```
vec_new
vec_push
vec_pop
vec_get
vec_set
vec_len
```

---

## Compiler Internals

### Compilation Pipeline

```
Source Code (.cer)
    ↓
[Lexer] → Tokens
    ↓
[Parser] → AST (Abstract Syntax Tree)
    ↓
[Typechecker] → Typed AST
    ↓
[Codegen] → Bytecode Instructions
    ↓
[Artifact Writer] → Artifact (.crt)
```

### Standard Library Structure

```
stdlib/
├── compiler.cer           # Main entry point
├── lexer/
│   ├── lexer_core.cer     # Tokenization
│   └── tokens.cer         # Token definitions
├── parser/
│   ├── parser_core.cer    # Parse logic
│   └── ast.cer            # AST node types
├── typecheck/
│   ├── typecheck_core.cer # Type checking
│   └── builtins.cer       # Built-in functions
├── codegen/
│   ├── codegen_core.cer   # Code generation
│   └── instr.cer          # Instruction definitions
└── runtime/
    ├── runtime.cer        # Runtime entry point
    └── vm/                # VM implementation
```

### Understanding the Lexer

The lexer converts text into tokens:

```
Input:  "let x = 42;"
        ↓
Tokens: [Keyword("let"), Identifier("x"), Operator("="), Number("42"), Punctuation(";")]
```

Key token types in Cerberus:
- Keywords: `let`, `fn`, `match`, `if`, `else`, `while`, `return`
- Operators: `+`, `-`, `*`, `/`, `==`, `!=`, `<`, `>`, `&&`, `||`
- Literals: Numbers, strings, booleans
- Identifiers: Variable/function names
- Punctuation: `(`, `)`, `{`, `}`, `[`, `]`, `;`, `,`

---

## Performance Optimization

### Benchmarking Your Code

```bash
# Time execution
time ./target/release/cerberus-compiler run program.crt

# Profile with runtime limits
cerberus-compiler run program.crt --limit-steps 1000000
```

### Common Performance Bottlenecks

1. **String Concatenation in Loops**
   ```cerberus
   // ❌ Inefficient: creates new string each iteration
   let result: string = "";
   let i: i64 = 0;
   while i < 1000 {
     result = result + "a";
     i = i + 1;
   }
   ```

2. **Vector Lookups**
   ```cerberus
   // ✅ Cache frequently accessed elements
   let data: vector[i64] = [1, 2, 3, 4, 5];
   let first: i64 = vector_get(data, 0);
   let second: i64 = vector_get(data, 1);
   // Use first and second instead of repeatedly calling vector_get
   ```

3. **Repeated Calculations**
   ```cerberus
   // ❌ Recalculates same value
   let i: i64 = 0;
   while i < vector_len(my_vector) {
     i = i + 1;
   }
   
   // ✅ Cache the length
   let len: i64 = vector_len(my_vector);
   let i: i64 = 0;
   while i < len {
     i = i + 1;
   }
   ```

---

## EXPERT LEVEL

---

## Contributing to Cerberus Core

### Setting Up Development Environment

```bash
# Clone and setup
git clone https://github.com/zulsyam23-dot/cerberus-compiler
cd cerberus-compiler

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build development version
cargo build

# Run tests
cargo test

# Run integration tests
cargo test --test selfhost_cli
```

### Code Style Guidelines

**Naming Conventions:**

```cerberus
// Functions: snake_case
fn calculate_sum(a: i64, b: i64) -> i64 {
  return a + b;
}

// Variables: snake_case
let user_name: string = "Alice";
let is_active: bool = true;

// Types: PascalCase (if custom types supported)
// Currently use string representations
```

**Formatting:**

```cerberus
// Consistent indentation (2 spaces)
fn main() {
  if true {
    println("Hello");
  }
}

// Line length: ~100 characters
// Long function signatures on multiple lines
fn very_long_function_name(
  param1: i64,
  param2: string,
  param3: vector[i64]
) -> result[string, string] {
  return Ok("done");
}
```

### Creating a Pull Request

1. **Fork the repository**
2. **Create a feature branch:**
   ```bash
   git checkout -b feature/improve-error-messages
   ```

3. **Make changes and test:**
   ```bash
   cargo test
   cargo test --test selfhost_cli
   ```

4. **Write clear commit messages:**
   ```
   Improve error messages for type mismatch errors

   - Add source location tracking
   - Show expected vs actual types
   - Suggest common fixes
   ```

5. **Push and create PR:**
   ```bash
   git push origin feature/improve-error-messages
   ```

---

## AST Manipulation

### Understanding the AST

The Abstract Syntax Tree represents program structure:

```
Program
  ├── Function Declaration (main)
  │   └── Block
  │       ├── Let Statement
  │       │   ├── name: "x"
  │       │   └── value: IntLiteral(42)
  │       └── Call Expression
  │           ├── function: "println"
  │           └── arguments: [Variable("x")]
```

### Working with AST in Stdlib

In `stdlib/compiler/ast.cer`, you can inspect AST nodes:

```cerberus
// Pseudo-code for AST types
type Stmt =
  | LetStmt { name: string, value: Expr }
  | ReturnStmt { value: option[Expr] }
  | ExprStmt { expr: Expr }
  | Block { statements: vector[Stmt] }

type Expr =
  | IntLit { value: i64 }
  | StrLit { value: string }
  | BoolLit { value: bool }
  | Variable { name: string }
  | BinOp { op: string, left: Expr, right: Expr }
  | Call { name: string, args: vector[Expr] }
```

### AST Visitor Pattern

```cerberus
// Process AST nodes recursively
fn visit_expr(expr: Expr) -> ResultType {
  match expr {
    IntLit(val) => process_int(val),
    StrLit(val) => process_str(val),
    Variable(name) => lookup_variable(name),
    BinOp(op, left, right) => {
      let left_result = visit_expr(left);
      let right_result = visit_expr(right);
      apply_operator(op, left_result, right_result);
    },
    Call(name, args) => {
      // Process function call
    },
  }
}
```

---

## Custom Instruction Implementation

### Adding a New Instruction

**Step 1: Define in `src/bytecode/instr.rs`**

```rust
#[derive(Debug, Clone)]
pub enum Instr {
    // ... existing instructions ...
    MyCustomOp,  // Add new instruction
}

#[repr(u8)]
pub enum InstrOpcode {
    // ... existing opcodes ...
    MyCustomOpCode = 200,  // Assign opcode
}
```

**Step 2: Implement in `src/vm/runtime/handler.rs`**

```rust
fn execute_my_custom_op(vm: &mut VirtualMachine) -> Result<(), VmError> {
    // Get stack operands if needed
    let arg1 = vm.stack.pop()?;
    let arg2 = vm.stack.pop()?;
    
    // Perform operation
    let result = my_operation(arg1, arg2);
    
    // Push result
    vm.stack.push(result);
    
    Ok(())
}
```

**Step 3: Wire in dispatch**

```rust
Instr::MyCustomOp => execute_my_custom_op(vm)?,
```

**Step 4: Add codegen in `stdlib/codegen/codegen_core.cer`**

```cerberus
fn emit_my_custom_op() {
  // Emit MyCustomOp instruction
}
```

### Testing Custom Instructions

```bash
# Create test program
cat > test_custom.cer << 'EOF'
fn main() {
  // Use new instruction
}
EOF

# Compile and test
cargo test
./target/release/cerberus-compiler run test_custom.crt
```

---

## Common Patterns & Best Practices

### Error Handling Pattern

```cerberus
fn safe_operation(input: string) -> result[i64, string] {
  // Validate input
  if str_len(input) == 0 {
    return Err("Input cannot be empty");
  }
  
  // Attempt operation
  // On success
  return Ok(42);
  
  // On failure
  // return Err("Description of what went wrong");
}

fn main() {
  match safe_operation("test") {
    Ok(value) => println("Success: " + value),
    Err(msg) => println("Error: " + msg),
  }
}
```

### Option Handling Pattern

```cerberus
fn find_user(id: i64) -> option[string] {
  if id > 0 && id < 1000 {
    return Some("User" + id);
  }
  return None;
}

fn main() {
  let user: option[string] = find_user(5);
  
  // Pattern 1: Match
  match user {
    Some(name) => println(name),
    None => println("Not found"),
  }
  
  // Pattern 2: Unwrap with default
  let name: string = opt_unwrap_or(user, "Unknown");
  println(name);
}
```

### Collection Processing

```cerberus
fn sum_vector(numbers: vector[i64]) -> i64 {
  let sum: i64 = 0;
  let i: i64 = 0;
  let len: i64 = vector_len(numbers);
  
  while i < len {
    let num: i64 = vector_get(numbers, i);
    sum = sum + num;
    i = i + 1;
  }
  
  return sum;
}
```

---

## Troubleshooting

### Common Errors

**Error: "Type mismatch: expected i64, found string"**
```cerberus
// ❌ Wrong
let count: i64 = "5";

// ✅ Correct
let count: i64 = 5;

// Or if converting from string
// Implementation depends on available conversion functions
```

**Error: "Variable used before assignment"**
```cerberus
// ❌ Wrong
if true {
  let x: i64 = 42;
}
println(x);  // x not in scope

// ✅ Correct
let x: i64 = 0;
if true {
  x = 42;
}
println(x);  // x is in scope
```

**Error: "Function not found"**
```cerberus
// ❌ Wrong
result = my_undefined_function();

// ✅ Correct - define function first
fn my_function() -> i64 {
  return 42;
}

result = my_function();
```

---

## Resources & Further Learning

### Official Documentation
- [Architecture Guide](docs/ARCHITECTURE.md)
- [Runtime Status](docs/self-host-runtime.md)
- [Glossary](docs/GLOSSARY.md)

### Key Concepts Review
1. **Type Safety**: Why explicit types prevent bugs
2. **Ownership**: Understanding stack-only memory model
3. **Error Handling**: Result/Option instead of exceptions
4. **VM Model**: How bytecode execution works

### Next Steps for Experts

1. **Implement language feature**: Add new syntax or instruction
2. **Optimize pipeline**: Improve compiler performance
3. **Expand stdlib**: Add new built-in functions
4. **Create tools**: Build ecosystem tools (linter, formatter, etc.)

---

## Quick Reference Card

### Variable Declaration
```cerberus
let name: type = value;
```

### Function Definition
```cerberus
fn function_name(param: type, ...) -> return_type {
  // body
  return value;
}
```

### Control Flow
```cerberus
if condition { }
else { }

while condition {
  // body
}

match value {
  pattern1 => action1,
  pattern2 => action2,
  _ => default_action,
}
```

### Common Built-ins
```cerberus
println(value)
vector_new()
map_new()
opt_unwrap_or(option, default)
res_unwrap_or(result, default)
str_len(s)
env_get(var_name)
```

---

**Last Updated:** 2026-04-29  
**Status:** Comprehensive guide for all skill levels  
**Contributing:** See CONTRIBUTING.md for guidelines
