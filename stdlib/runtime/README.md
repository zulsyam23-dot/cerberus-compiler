# Runtime Module Layout

- `runtime.cer`: runtime entrypoint.
- `vm/runtime_vm.cer`: facade VM module.
- `vm/runtime_io.cer`: host I/O adapter.
- `vm/runtime_vm_text.cer`: text parsing helpers.
- `vm/runtime_vm_value.cer`: tagged value helpers.
- `vm/runtime_vm_validate.cer`: static validation for directives/opcodes/args/targets.
- `vm/runtime/runtime_vm_exec.cer`: interpreter loop and opcode execution.

## Current Capabilities

- Label-aware control flow: `label`, `jump`, `jump_if_false`.
- Function frames: `call`, `ret`, `ret_val`.
- Runtime limits: `limit_steps`, `limit_stack`, `limit_call`.
- Detailed runtime errors with instruction pointer context.
- Typed runtime value envelope: `v1|kind|len|payload` (reads legacy `i:`, `b:`, `s:` too).
- Preflight static validation before execution.

## Script Contract

- Recommended header:
  - `@cerberus_vm 1`
  - `@entry <label|ip>`
  - `@limit_steps <n>` (optional)
  - `@limit_stack <n>` (optional)
  - `@limit_call <n>` (optional)
- Runtime rejects unknown directives.
- Directives must appear before the first opcode.
- Legacy scripts without directives are still accepted.
- Instruction separator supports newline (recommended) and `;` (legacy).
- Inline `#` comments are supported.
- `;` inside string literals is preserved correctly.
