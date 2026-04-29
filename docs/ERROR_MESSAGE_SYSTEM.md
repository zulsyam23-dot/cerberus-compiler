# Cerberus Error Message System Design

**Location:** `stdlib/error_handler/`  
**Language:** Cerberus (Self-Hosted)  
**Purpose:** Professional error messaging implementation in Cerberus stdlib  
**Last Updated:** 2026-04-29

---

## Table of Contents

1. [Overview](#overview)
2. [Error Categories](#error-categories)
3. [Error Message Architecture](#error-message-architecture)
4. [Implementation in Cerberus](#implementation-in-cerberus)
5. [Usage Examples](#usage-examples)
6. [Error Recovery Strategies](#error-recovery-strategies)
7. [Testing Error Messages](#testing-error-messages)

---

## Overview

### Design Goals

The Cerberus error messaging system aims to provide:

- ✅ **Clear Context**: Show exactly where and why an error occurred
- ✅ **Helpful Suggestions**: Offer actionable fixes for common mistakes
- ✅ **Professional Output**: Format messages for easy reading and debugging
- ✅ **Type Safety**: Use structured error types instead of strings
- ✅ **Source Location Tracking**: Reference line and column numbers
- ✅ **Cascade Prevention**: Avoid reporting multiple errors for single root cause

### Current State

Currently, errors are often simple strings:

```
Error: Type mismatch
```

### Desired State

```
Error [TYPE_MISMATCH]: Type mismatch in binary operation
  Location: main.cer:5:12
  Left operand: string = "hello"
  Right operand: i64 = 42
  Suggestion: Use string_concat("hello", int_to_string(42))
  See: https://docs.cerberus.dev/type-safety
```

---

## Error Categories

### 1. Lexer Errors

**Source Location:** `stdlib/lexer/`  
**Scope:** Tokenization phase

```cerberus
// Examples:
- Unclosed string: "hello world
- Invalid escape: "hello\x"
- Unknown character: @ # $ (when not allowed)
- Numeric overflow: 99999999999999999999999999999
```

**Error Structure:**
```cerberus
type LexerError = {
  kind: string,                    // "UNCLOSED_STRING" | "INVALID_ESCAPE" | ...
  message: string,                 // Human-readable description
  position: SourceLocation,        // Line, column, offset
  snippet: string,                 // Code context
  suggestion: option[string],      // Helpful hint
}
```

### 2. Parser Errors

**Source Location:** `stdlib/parser/`  
**Scope:** Syntax analysis phase

```cerberus
// Examples:
- Expected token: fn foo() { println("hi" ]  // Expected } not ]
- Unexpected EOF: fn incomplete() {
- Invalid syntax: let x 42;  // Missing =
- Duplicate definition: fn foo() { } fn foo() { }
```

**Error Structure:**
```cerberus
type ParserError = {
  kind: string,                    // "EXPECTED_TOKEN" | "UNEXPECTED_EOF" | ...
  message: string,
  expected: string,                // What was expected
  found: string,                    // What was actually found
  position: SourceLocation,
  snippet: string,
  suggestion: option[string],
}
```

### 3. Type Checker Errors

**Source Location:** `stdlib/typecheck/`  
**Scope:** Type checking phase

```cerberus
// Examples:
- Type mismatch: let x: i64 = "hello";
- Unknown symbol: println(undefined_var);
- Wrong argument count: add(1);  // Expected 2, got 1
- Function not found: call_nonexistent();
- Option/Result not handled: let x = maybe_get_value();
```

**Error Structure:**
```cerberus
type TypeCheckError = {
  kind: string,                    // "TYPE_MISMATCH" | "UNKNOWN_SYMBOL" | ...
  message: string,
  expected_type: string,           // For type mismatch
  actual_type: string,             // For type mismatch
  symbol_name: option[string],     // For resolution errors
  position: SourceLocation,
  snippet: string,
  suggestions: vector[string],     // Multiple helpful hints
  related_locations: vector[SourceLocation],  // Show previous definition
}
```

### 4. Codegen Errors

**Source Location:** `stdlib/codegen/`  
**Scope:** Code generation phase

```cerberus
// Examples:
- Stack overflow in compile: deeply nested expressions
- Instruction encoding failed: invalid bytecode operation
- Symbol not resolved at codegen time
- Unrecoverable type error
```

**Error Structure:**
```cerberus
type CodegenError = {
  kind: string,
  message: string,
  context: string,                 // What was being generated
  position: SourceLocation,
  snippet: string,
  suggestion: option[string],
}
```

### 5. Runtime Errors

**Source Location:** `stdlib/runtime/`  
**Scope:** VM execution phase

```cerberus
// Examples:
- Stack overflow: infinite recursion
- Out of memory: too many allocations
- Division by zero
- Array index out of bounds
- Execution limit exceeded
- Null pointer (from Option/Result unwrap)
```

**Error Structure:**
```cerberus
type RuntimeError = {
  kind: string,                    // "STACK_OVERFLOW" | "DIV_BY_ZERO" | ...
  message: string,
  context: string,                 // Instruction or function name
  value_info: option[string],      // Related values
  stack_trace: vector[StackFrame], // Call stack
  suggestion: option[string],
}
```

---

## Error Message Architecture

### Source Location Tracking

```cerberus
type SourceLocation = {
  filename: string,                // "main.cer"
  line: i64,                       // 1-indexed
  column: i64,                     // 1-indexed
  offset: i64,                     // Byte offset in file
  length: i64,                     // Error span length
}

type SourceContext = {
  filename: string,
  lines: vector[string],           // Full source file lines
  location: SourceLocation,
}
```

### Error Formatter

```cerberus
// In stdlib/error_handler/formatter.cer
fn format_error(error: ErrorType) -> string {
  // Returns formatted error message
  // Example output:
  // Error [TYPE_MISMATCH]: Type mismatch in binary operation
  //   File: main.cer
  //   Location: 5:12
  //   
  //   5 | let result = x + y;
  //       |             ^ ^
  //       |             | i64
  //       |             string
  //   
  //   Suggestion: Use string_concat("x", int_to_string(y))
}

fn format_with_context(error: ErrorType, source: SourceContext) -> string {
  // Includes code snippet
}

fn format_error_chain(errors: vector[ErrorType]) -> string {
  // Formats multiple errors with deduplication
}
```

### Error Registry

```cerberus
// In stdlib/error_handler/registry.cer
// Central error code definitions

type ErrorCode = {
  code: string,                    // "E0001" | "E0002" | ...
  category: string,                // "LEXER" | "PARSER" | "TYPECHECK" | ...
  message_template: string,        // "Type mismatch: expected {}, found {}"
  help_text: string,               // Detailed explanation
  examples: vector[string],        // Common cases
  documentation_url: string,       // Link to docs
}

// Error registry mapping
fn get_error_code(kind: string) -> option[ErrorCode] {
  // Lookup error definition
}
```

---

## Implementation in Cerberus

### Module Structure

```
stdlib/error_handler/
├── mod.cer                    # Module entrypoint
├── types.cer                  # Error type definitions
├── source_location.cer        # Location tracking
├── formatter.cer              # Message formatting
├── registry.cer               # Error code definitions
├── lexer_errors.cer           # Lexer error constructors
├── parser_errors.cer          # Parser error constructors
├── typecheck_errors.cer       # Type checker error constructors
├── codegen_errors.cer         # Codegen error constructors
├── runtime_errors.cer         # Runtime error constructors
└── utils.cer                  # Helper functions
```

### Core Types Module (`types.cer`)

```cerberus
// Base error type
type Error = {
  kind: string,                    // Category of error
  message: string,                 // Primary message
  code: string,                    // Error code (E0001, E0002, ...)
  location: SourceLocation,        // Where error occurred
  context: string,                 // What was being processed
  severity: string,                // "ERROR" | "WARNING" | "NOTE"
  suggestions: vector[string],     // How to fix
  related_locations: vector[SourceLocation],  // Related errors
}

// Specific error subtypes
type TypeError = {
  base: Error,
  expected: string,
  actual: string,
}

type ResolutionError = {
  base: Error,
  symbol: string,
  available: vector[string],       // Similar names for suggestions
}

type SyntaxError = {
  base: Error,
  expected: string,
  found: string,
}
```

### Source Location Module (`source_location.cer`)

```cerberus
// Track source positions
type SourceLocation = {
  filename: string,
  line: i64,
  column: i64,
  offset: i64,
  length: i64,
}

fn location_new(filename: string, line: i64, column: i64) -> SourceLocation {
  return {
    filename: filename,
    line: line,
    column: column,
    offset: 0,  // Calculate if needed
    length: 1,
  };
}

fn location_to_string(loc: SourceLocation) -> string {
  // Format: "main.cer:5:12"
  let line_str = int_to_string(loc.line);
  let col_str = int_to_string(loc.column);
  return loc.filename + ":" + line_str + ":" + col_str;
}

fn location_with_context(
  loc: SourceLocation,
  lines: vector[string]
) -> string {
  // Show code snippet with error indicator
  // 
  // 5 | let x = "hello" + 42;
  //     |         ^^^^^^^^^^ string
  //     |                    ^^ i64
  //   
  // Suggestion: ...
}
```

### Formatter Module (`formatter.cer`)

```cerberus
// Professional error formatting
fn format_error(error: Error) -> string {
  let header: string = "Error [" + error.kind + "]: " + error.message;
  let location: string = "  Location: " + location_to_string(error.location);
  let context: string = "  Context: " + error.context;
  
  let suggestions_text: string = "";
  if vector_len(error.suggestions) > 0 {
    suggestions_text = "\n  Suggestions:\n";
    let i: i64 = 0;
    while i < vector_len(error.suggestions) {
      let suggestion: string = vector_get(error.suggestions, i);
      suggestions_text = suggestions_text + "    - " + suggestion + "\n";
      i = i + 1;
    }
  }
  
  return header + "\n" + location + "\n" + context + suggestions_text;
}

fn format_with_snippet(error: Error, source_lines: vector[string]) -> string {
  // Includes code context
  let formatted: string = format_error(error);
  let line_idx: i64 = error.location.line - 1;
  
  if line_idx >= 0 && line_idx < vector_len(source_lines) {
    let code_line: string = vector_get(source_lines, line_idx);
    let line_num: string = int_to_string(error.location.line);
    
    formatted = formatted + "\n\n  " + line_num + " | " + code_line;
    
    // Add error indicator
    let indicator: string = "      | ";
    let col: i64 = 0;
    while col < error.location.column - 1 {
      indicator = indicator + " ";
      col = col + 1;
    }
    indicator = indicator + "^";
    col = col + 1;
    while col < error.location.column - 1 + error.location.length {
      indicator = indicator + "^";
      col = col + 1;
    }
    
    formatted = formatted + "\n" + indicator;
  }
  
  return formatted;
}

fn format_multiple(errors: vector[Error]) -> string {
  // Combine multiple errors with deduplication
  let result: string = "";
  let i: i64 = 0;
  
  while i < vector_len(errors) {
    let error: Error = vector_get(errors, i);
    if i > 0 {
      result = result + "\n\n";
    }
    result = result + format_error(error);
    i = i + 1;
  }
  
  return result;
}
```

### Error Constructors

#### Lexer Errors (`lexer_errors.cer`)

```cerberus
fn err_unclosed_string(loc: SourceLocation) -> Error {
  return {
    kind: "UNCLOSED_STRING",
    message: "Unclosed string literal",
    code: "E1001",
    location: loc,
    context: "While tokenizing",
    severity: "ERROR",
    suggestions: vector["Add closing quote \" to complete the string"],
    related_locations: vector[],
  };
}

fn err_invalid_escape(loc: SourceLocation, escape: string) -> Error {
  let message: string = "Invalid escape sequence: \\" + escape;
  return {
    kind: "INVALID_ESCAPE",
    message: message,
    code: "E1002",
    location: loc,
    context: "While tokenizing string",
    severity: "ERROR",
    suggestions: vector[
      "Use valid escapes: \\n (newline), \\t (tab), \\\\ (backslash)",
      "For literal backslash, use \\\\"
    ],
    related_locations: vector[],
  };
}

fn err_unknown_char(loc: SourceLocation, char: string) -> Error {
  return {
    kind: "UNKNOWN_CHARACTER",
    message: "Unknown character: " + char,
    code: "E1003",
    location: loc,
    context: "While tokenizing",
    severity: "ERROR",
    suggestions: vector["Check for typos or unsupported characters"],
    related_locations: vector[],
  };
}
```

#### Type Checker Errors (`typecheck_errors.cer`)

```cerberus
fn err_type_mismatch(
  loc: SourceLocation,
  expected: string,
  actual: string,
  context: string
) -> Error {
  return {
    kind: "TYPE_MISMATCH",
    message: "Type mismatch: expected " + expected + ", found " + actual,
    code: "E3001",
    location: loc,
    context: context,
    severity: "ERROR",
    suggestions: vector[
      "Check variable assignment",
      "Use explicit type conversion if needed",
      "Verify function parameter types"
    ],
    related_locations: vector[],
  };
}

fn err_unknown_symbol(
  loc: SourceLocation,
  symbol: string,
  available: vector[string]
) -> Error {
  let suggestions: vector[string] = vector[
    "Check spelling of '" + symbol + "'"
  ];
  
  // Add similar name suggestions if available
  if vector_len(available) > 0 {
    vector_push(suggestions, "Did you mean one of: " + vector_get(available, 0));
  }
  
  return {
    kind: "UNKNOWN_SYMBOL",
    message: "Symbol not found: " + symbol,
    code: "E3002",
    location: loc,
    context: "While resolving symbol",
    severity: "ERROR",
    suggestions: suggestions,
    related_locations: vector[],
  };
}

fn err_wrong_arg_count(
  loc: SourceLocation,
  func: string,
  expected: i64,
  found: i64
) -> Error {
  let expected_str: string = int_to_string(expected);
  let found_str: string = int_to_string(found);
  let message: string = func + "() expected " + expected_str + 
                        " arguments, found " + found_str;
  
  return {
    kind: "WRONG_ARG_COUNT",
    message: message,
    code: "E3003",
    location: loc,
    context: "While type-checking function call",
    severity: "ERROR",
    suggestions: vector[
      "Check function definition for parameter count",
      "Verify you're calling the correct function"
    ],
    related_locations: vector[],
  };
}

fn err_result_not_handled(
  loc: SourceLocation,
  result_type: string
) -> Error {
  return {
    kind: "RESULT_NOT_HANDLED",
    message: "Result type must be explicitly handled",
    code: "E3004",
    location: loc,
    context: "While type-checking assignment",
    severity: "ERROR",
    suggestions: vector[
      "Use match statement to handle Ok and Err cases",
      "Or use res_unwrap_or() with a default value"
    ],
    related_locations: vector[],
  };
}

fn err_option_not_handled(
  loc: SourceLocation,
  option_type: string
) -> Error {
  return {
    kind: "OPTION_NOT_HANDLED",
    message: "Option type must be explicitly handled",
    code: "E3005",
    location: loc,
    context: "While type-checking assignment",
    severity: "ERROR",
    suggestions: vector[
      "Use match statement to handle Some and None cases",
      "Or use opt_unwrap_or() with a default value"
    ],
    related_locations: vector[],
  };
}
```

#### Parser Errors (`parser_errors.cer`)

```cerberus
fn err_expected_token(
  loc: SourceLocation,
  expected: string,
  found: string
) -> Error {
  let message: string = "Expected " + expected + ", found " + found;
  return {
    kind: "EXPECTED_TOKEN",
    message: message,
    code: "E2001",
    location: loc,
    context: "While parsing",
    severity: "ERROR",
    suggestions: vector["Check syntax near this location"],
    related_locations: vector[],
  };
}

fn err_unexpected_eof(context: string) -> Error {
  return {
    kind: "UNEXPECTED_EOF",
    message: "Unexpected end of file",
    code: "E2002",
    location: {
      filename: "unknown",
      line: 0,
      column: 0,
      offset: 0,
      length: 1,
    },
    context: context,
    severity: "ERROR",
    suggestions: vector["Check that all blocks are properly closed with }"],
    related_locations: vector[],
  };
}

fn err_invalid_syntax(loc: SourceLocation, detail: string) -> Error {
  return {
    kind: "INVALID_SYNTAX",
    message: "Invalid syntax: " + detail,
    code: "E2003",
    location: loc,
    context: "While parsing",
    severity: "ERROR",
    suggestions: vector["Review Cerberus syntax documentation"],
    related_locations: vector[],
  };
}
```

---

## Usage Examples

### In Lexer

```cerberus
// stdlib/lexer/lexer_core.cer
fn tokenize_string(input: string, start_pos: i64) -> result[Token, Error] {
  let pos: i64 = start_pos + 1;
  let content: string = "";
  
  while pos < string_len(input) {
    let char: string = string_substr(input, pos, 1);
    
    if char == "\"" {
      // String closed successfully
      return Ok({
        type: "STRING",
        value: content,
        position: start_pos,
      });
    } else if char == "\n" {
      // Unclosed string
      let loc: SourceLocation = location_new("input.cer", 0, pos);
      return Err(err_unclosed_string(loc));
    } else if char == "\\" {
      // Handle escape sequence
      let next_char: string = string_substr(input, pos + 1, 1);
      match next_char {
        "n" => content = content + "\n",
        "t" => content = content + "\t",
        "\\" => content = content + "\\",
        "\"" => content = content + "\"",
        _ => {
          let loc: SourceLocation = location_new("input.cer", 0, pos);
          return Err(err_invalid_escape(loc, next_char));
        }
      }
      pos = pos + 2;
    } else {
      content = content + char;
      pos = pos + 1;
    }
  }
  
  // Reached EOF without closing quote
  let loc: SourceLocation = location_new("input.cer", 0, pos);
  return Err(err_unclosed_string(loc));
}
```

### In Type Checker

```cerberus
// stdlib/typecheck/typecheck_core.cer
fn check_binary_op(
  op: string,
  left_type: string,
  right_type: string,
  loc: SourceLocation
) -> result[string, Error] {
  
  // Check if operation is valid for types
  if op == "+" {
    if left_type == "string" && right_type == "string" {
      return Ok("string");
    } else if left_type == "i64" && right_type == "i64" {
      return Ok("i64");
    } else {
      let context: string = "Binary operation: " + left_type + " + " + right_type;
      let err: Error = err_type_mismatch(loc, "matching types", 
                                          left_type + " and " + right_type,
                                          context);
      return Err(err);
    }
  }
  
  return Err(err_type_mismatch(loc, "valid combination", 
                               left_type + " " + op + " " + right_type,
                               ""));
}

fn resolve_symbol(name: string, scope: SymbolTable, loc: SourceLocation) 
  -> result[SymbolInfo, Error] {
  
  match scope.lookup(name) {
    Some(info) => return Ok(info),
    None => {
      // Find similar names for suggestion
      let available: vector[string] = scope.get_similar(name);
      let err: Error = err_unknown_symbol(loc, name, available);
      return Err(err);
    }
  }
}
```

### Error Reporting to User

```cerberus
// In main error handler
fn report_errors(errors: vector[Error], source_file: string) -> unit {
  if vector_len(errors) == 0 {
    return;
  }
  
  let source_lines: vector[string] = read_file_lines(source_file);
  let formatted: string = format_multiple(errors);
  
  // Print with source context
  let i: i64 = 0;
  while i < vector_len(errors) {
    let error: Error = vector_get(errors, i);
    let output: string = format_with_snippet(error, source_lines);
    println(output);
    i = i + 1;
  }
  
  // Print summary
  let count: i64 = vector_len(errors);
  let count_str: string = int_to_string(count);
  println("\nFound " + count_str + " error(s)");
}
```

---

## Error Recovery Strategies

### Lexer Error Recovery

When the lexer encounters an error, it should:
1. Report the error with context
2. Attempt to recover by skipping to next token boundary
3. Continue tokenizing to find more errors

```cerberus
fn tokenize_with_recovery(input: string) -> (vector[Token], vector[Error]) {
  let tokens: vector[Token] = vector_new();
  let errors: vector[Error] = vector_new();
  let pos: i64 = 0;
  
  while pos < string_len(input) {
    match tokenize_one(input, pos) {
      Ok(token) => {
        vector_push(tokens, token);
        pos = token.end_pos;
      },
      Err(error) => {
        vector_push(errors, error);
        // Skip to next whitespace and try again
        pos = skip_to_next_boundary(input, pos);
      }
    }
  }
  
  return (tokens, errors);
}
```

### Parser Error Recovery

Parser should attempt to continue parsing after syntax errors:

```cerberus
fn parse_function_call_with_recovery(
  tokens: vector[Token],
  start_idx: i64
) -> (option[CallExpr], vector[Error]) {
  
  let errors: vector[Error] = vector_new();
  
  // Try to parse arguments, even if some are malformed
  let args: vector[Expr] = vector_new();
  let idx: i64 = start_idx + 1;  // Skip '('
  
  while idx < vector_len(tokens) {
    let token: Token = vector_get(tokens, idx);
    
    if token.type == ")" {
      break;
    }
    
    match parse_expr(tokens, idx) {
      Ok(expr) => {
        vector_push(args, expr);
        idx = expr.end_idx;
      },
      Err(error) => {
        vector_push(errors, error);
        // Skip to comma or closing paren
        while idx < vector_len(tokens) {
          let t: Token = vector_get(tokens, idx);
          if t.type == "," || t.type == ")" {
            break;
          }
          idx = idx + 1;
        }
      }
    }
    
    if idx < vector_len(tokens) {
      let t: Token = vector_get(tokens, idx);
      if t.type == "," {
        idx = idx + 1;
      }
    }
  }
  
  return (Some(call_expr), errors);
}
```

---

## Testing Error Messages

### Test Structure

```cerberus
// stdlib/error_handler/tests.cer
fn test_unclosed_string_error() {
  let input: string = "let x = \"hello;";
  let result: (vector[Token], vector[Error]) = tokenize_with_recovery(input);
  let errors = result.1;
  
  // Assert error occurred
  if vector_len(errors) != 1 {
    println("FAIL: Expected 1 error, got " + int_to_string(vector_len(errors)));
    return;
  }
  
  let error: Error = vector_get(errors, 0);
  
  // Check error properties
  if error.kind != "UNCLOSED_STRING" {
    println("FAIL: Expected UNCLOSED_STRING, got " + error.kind);
    return;
  }
  
  if error.location.line != 1 {
    println("FAIL: Expected line 1, got " + int_to_string(error.location.line));
    return;
  }
  
  println("PASS: Unclosed string error detected correctly");
}

fn test_type_mismatch_error() {
  let source: string = "let x: i64 = \"hello\";";
  let result = compile_and_check(source);
  let errors = result.1;
  
  if vector_len(errors) != 1 {
    println("FAIL: Expected 1 error");
    return;
  }
  
  let error: Error = vector_get(errors, 0);
  if error.kind != "TYPE_MISMATCH" {
    println("FAIL: Expected TYPE_MISMATCH");
    return;
  }
  
  // Check suggestions are present
  if vector_len(error.suggestions) == 0 {
    println("FAIL: Expected suggestions");
    return;
  }
  
  println("PASS: Type mismatch error with suggestions");
}

fn test_error_formatting() {
  let error: Error = err_type_mismatch(
    {
      filename: "test.cer",
      line: 5,
      column: 10,
      offset: 0,
      length: 5,
    },
    "i64",
    "string",
    "Assignment"
  );
  
  let formatted: string = format_error(error);
  
  // Check format includes key parts
  if string_contains(formatted, "TYPE_MISMATCH") == false {
    println("FAIL: Formatted message missing error type");
    return;
  }
  
  if string_contains(formatted, "test.cer") == false {
    println("FAIL: Formatted message missing filename");
    return;
  }
  
  if string_contains(formatted, "5:10") == false {
    println("FAIL: Formatted message missing location");
    return;
  }
  
  println("PASS: Error formatting correct");
}

fn run_all_tests() {
  test_unclosed_string_error();
  test_type_mismatch_error();
  test_error_formatting();
  // ... more tests
}
```

### Running Error Tests

```bash
# Compile and run error tests
cerberus-compiler compile stdlib/error_handler/tests.cer -o tests.crt
cerberus-compiler run tests.crt

# Output:
# PASS: Unclosed string error detected correctly
# PASS: Type mismatch error with suggestions
# PASS: Error formatting correct
```

---

## Integration Points

### With Compiler Pipeline

```
Lexer
  ↓ (produces Tokens or Errors)
Parser
  ↓ (produces AST or Errors)
Type Checker
  ↓ (produces Typed AST or Errors)
Codegen
  ↓ (produces Bytecode or Errors)
  
Error Handler catches and formats at each stage
```

### With CLI

```rust
// In src/main.rs
match compile_file(input_path) {
    Ok(artifact) => {
        println!("Compilation successful");
        write_file(output_path, artifact);
    },
    Err(errors) => {
        // errors is vector[Error] from Cerberus stdlib
        for error in errors {
            println!("{}", format_error_for_display(&error));
        }
        std::process::exit(1);
    }
}
```

---

## Error Code Registry

### Lexer Errors (E1xxx)

| Code | Error | Severity |
|------|-------|----------|
| E1001 | UNCLOSED_STRING | ERROR |
| E1002 | INVALID_ESCAPE | ERROR |
| E1003 | UNKNOWN_CHARACTER | ERROR |
| E1004 | NUMERIC_OVERFLOW | ERROR |

### Parser Errors (E2xxx)

| Code | Error | Severity |
|------|-------|----------|
| E2001 | EXPECTED_TOKEN | ERROR |
| E2002 | UNEXPECTED_EOF | ERROR |
| E2003 | INVALID_SYNTAX | ERROR |
| E2004 | DUPLICATE_DEFINITION | ERROR |

### Type Checker Errors (E3xxx)

| Code | Error | Severity |
|------|-------|----------|
| E3001 | TYPE_MISMATCH | ERROR |
| E3002 | UNKNOWN_SYMBOL | ERROR |
| E3003 | WRONG_ARG_COUNT | ERROR |
| E3004 | RESULT_NOT_HANDLED | ERROR |
| E3005 | OPTION_NOT_HANDLED | ERROR |

### Codegen Errors (E4xxx)

| Code | Error | Severity |
|------|-------|----------|
| E4001 | STACK_OVERFLOW | ERROR |
| E4002 | INSTRUCTION_ENCODING | ERROR |
| E4003 | SYMBOL_RESOLUTION | ERROR |

### Runtime Errors (E5xxx)

| Code | Error | Severity |
|------|-------|----------|
| E5001 | STACK_OVERFLOW_RUNTIME | ERROR |
| E5002 | DIV_BY_ZERO | ERROR |
| E5003 | OUT_OF_BOUNDS | ERROR |
| E5004 | EXECUTION_LIMIT | ERROR |

---

## Future Enhancements

### Planned Features

1. **Warning System**: Non-fatal issues (unused variables, type shadows)
   ```cerberus
   type WarningCode = { /* ... */ }
   fn warn_unused_variable(name: string, loc: SourceLocation) -> Error { }
   ```

2. **Hint System**: Informational messages
   ```cerberus
   fn hint_type_inference_available(loc: SourceLocation) -> Error { }
   ```

3. **Localization**: Multi-language error messages
   ```cerberus
   fn set_error_language(lang: string) { }
   ```

4. **Error Statistics**: Track error patterns
   ```cerberus
   type ErrorStats = { total: i64, by_kind: map[string, i64] }
   fn collect_stats(errors: vector[Error]) -> ErrorStats { }
   ```

---

**Document Status:** Design specification for stdlib implementation  
**Next Steps:** Implement modules in order (types → source_location → formatter → specific error types)
