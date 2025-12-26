# Missing f16 ↔ i128/u128 Compiler Builtins on macOS

## Summary

Direct conversions between `f16` and `i128`/`u128` fail to link on macOS (both
Intel and Apple Silicon) due to missing compiler builtins:

- `__floattihf` (i128 → f16)
- `__floatuntihf` (u128 → f16)

## Reproduction

```bash
cargo +nightly build
```

### Expected Behavior

The code should compile and link successfully, as the type system accepts these
conversions.

### Actual Behavior

```
Undefined symbols for architecture arm64:
  "___floattihf", referenced from: ...
  "___floatuntihf", referenced from: ...
ld: symbol(s) not found for architecture arm64
```

(Similar error on x86_64)

## Environment

- **Architecture**: Both x86_64 (Intel) and aarch64 (Apple Silicon)
- **OS**: macOS
- **Rust**: nightly (required for f16 support)
- **Target**: x86_64-apple-darwin, aarch64-apple-darwin

## Workaround

Convert via `f64` intermediate:

```rust
// Instead of: let f: f16 = value as f16;
let f: f16 = (value as f64) as f16;
```

## Notes

- These conversions work on platforms where the builtins are available
- The type system accepts the conversions (no compile errors)
- Only linking fails due to missing runtime support
- All other integer ↔ f16 conversions work (u8-u64, i8-i64)
