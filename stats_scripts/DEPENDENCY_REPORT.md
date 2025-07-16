# LibSignal Dependency Analysis Report

This report analyzes how functions in the `rust/` directory call functions from the `deps/` directory.

## Executive Summary

- **75 Rust functions** make calls to dependency functions
- **136 unique dependency functions** are called
- **33 Rust files** use dependencies
- **232 total dependency calls** across the codebase

## Most Critical Dependencies

These are the dependency functions called most frequently:

| Rank | Function | Calls | Purpose |
|------|----------|-------|---------|
| 1 | `ct_eq` | 13 | Constant-time equality (security) |
| 2 | `from_curve_name` | 6 | Elliptic curve initialization |
| 3 | `from` | 6 | General utility |
| 4 | `sha256` | 5 | SHA-256 hashing |
| 5 | `serial_number` | 5 | X.509 certificate serial number |
| 6 | `from_unix` | 4 | Unix timestamp conversion |
| 7 | `days_from_now` | 4 | Date arithmetic |
| 8 | `subject_name` | 3 | X.509 certificate subject |
| 9 | `sign` | 3 | Digital signature creation |
| 10 | `to_owned` | 3 | General utility |

## Dependency Crate Usage

| Crate | Total Calls | Primary Use Case |
|-------|-------------|------------------|
| `boring-signal` | 205 | Cryptographic operations (BoringSSL wrapper) |
| `ctr` | 1 | Counter mode encryption |
| `subtle` | 26 | Constant-time operations for security |

## High-Impact Rust Files

Files that make the most dependency calls:

- `rust/core/src/curve.rs`
- `rust/protocol/src/crypto.rs`
- `rust/protocol/src/kem.rs`
- `rust/keytrans/src/vrf.rs`
- `rust/crypto/src/aes_gcm.rs`
- `rust/zkgroup/src/crypto/profile_key_struct.rs`
- `rust/poksho/src/proof.rs`
- `rust/core/examples/ed_to_xed.rs`
- `rust/zkgroup/src/crypto/profile_key_encryption.rs`
- `rust/zkcredential/src/endorsements.rs`

## Security Implications

### Constant-Time Operations
The heavy use of `subtle::ct_eq` (13 calls) indicates proper attention to timing attack resistance.

### Cryptographic Dependencies
- **boring-signal**: Used for production-grade cryptographic operations
- **subtle**: Ensures constant-time operations for security-critical code
- **curve25519-dalek**: Elliptic curve operations (referenced in code)

## Recommendations

1. **Monitor boring-signal updates**: With 205 calls, this is a critical dependency
2. **Audit constant-time usage**: Ensure all security-critical comparisons use `subtle`
3. **Review attestation module**: `rust/attest/` has the highest dependency usage
4. **Consider dependency consolidation**: Multiple crypto crates could potentially be unified

## Technical Details

### Analysis Method
This analysis was performed by parsing the `libsignal_with_deps.json` file, which contains:
- Function identifiers and their dependencies
- File paths for each function
- Function bodies for context analysis

### Limitations
- Only analyzes direct function calls tracked in the JSON
- Does not include macro expansions or generic instantiations
- Code body analysis is text-based (may include false positives)
