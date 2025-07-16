# LibSignal Dependency Analysis Report

This report analyzes function calls from `rust/` code to `deps/` code in the LibSignal project.

## 📊 Overall Statistics

- **Total rust functions calling deps:** 75
- **Total deps functions called:** 136
- **Total rust files using deps:** 33
- **Total dependency calls:** 232

## 🔥 Top 20 Most Called Deps Functions

| Rank | Calls | Function | Crate |
|------|-------|----------|-------|
| 1 | 13 | `ct_eq` | subtle |
| 2 | 6 | `from_curve_name` | boring-signal |
| 3 | 6 | `from` | subtle |
| 4 | 5 | `sha256` | boring-signal |
| 5 | 5 | `serial_number` | boring-signal |
| 6 | 4 | `from_unix` | boring-signal |
| 7 | 4 | `days_from_now` | boring-signal |
| 8 | 3 | `subject_name` | boring-signal |
| 9 | 3 | `sign` | boring-signal |
| 10 | 3 | `to_owned` | boring-signal |
| 11 | 3 | `add_cert` | boring-signal |
| 12 | 3 | `build` | boring-signal |
| 13 | 3 | `new` | boring-signal |
| 14 | 3 | `compare` | boring-signal |
| 15 | 3 | `generate` | boring-signal |
| 16 | 3 | `builder` | boring-signal |
| 17 | 3 | `build` | boring-signal |
| 18 | 3 | `tls_client` | boring-signal |
| 19 | 3 | `configure` | boring-signal |
| 20 | 3 | `not_after` | boring-signal |

## 📦 Deps Crate Usage

| Crate | Calls | Description |
|-------|-------|-------------|
| `boring-signal` | 205 | External dependency |
| `subtle` | 26 | External dependency |
| `ctr` | 1 | External dependency |

## 📝 Crate References in Code Bodies

| Crate | References |
|-------|------------|
| `aes` | 217 |
| `cbc` | 104 |
| `ctr` | 64 |
| `curve25519_dalek` | 30 |
| `hmac` | 139 |
| `sha2` | 159 |
| `subtle` | 4 |

## 📁 Top 15 Rust Files by Deps Usage

| Deps Calls | File |
|------------|------|
| 67 | `rust/attest/src/cert_chain.rs` |
| 31 | `rust/attest/src/dcap.rs` |
| 18 | `rust/attest/src/dcap/fakes.rs` |
| 18 | `rust/device-transfer/src/lib.rs` |
| 15 | `rust/attest/src/dcap/sgx_quote.rs` |
| 15 | `rust/net/infra/src/certs.rs` |
| 11 | `rust/device-transfer/tests/tests.rs` |
| 7 | `rust/net/infra/src/tcp_ssl/proxy.rs` |
| 7 | `rust/net/infra/src/tcp_ssl.rs` |
| 6 | `rust/attest/src/dcap/revocation_list.rs` |
| 3 | `rust/zkgroup/src/crypto/profile_key_encryption.rs` |
| 3 | `rust/net/infra/src/errors.rs` |
| 3 | `rust/attest/src/dcap/ecdsa.rs` |
| 2 | `rust/crypto/src/aes_gcm.rs` |
| 2 | `rust/core/src/curve/curve25519.rs` |

## 🔍 Detailed Breakdown by Deps Crate

### BORING-SIGNAL

**Total calls:** 205

| Function | Calls |
|----------|-------|
| `from_curve_name` | 6 |
| `sha256` | 5 |
| `serial_number` | 5 |
| `from_unix` | 4 |
| `days_from_now` | 4 |
| `subject_name` | 3 |
| `sign` | 3 |
| `to_owned` | 3 |
| `add_cert` | 3 |
| `build` | 3 |
| ... | ... |
| *116 more functions* | |

### CTR

**Total calls:** 1

| Function | Calls |
|----------|-------|
| `inner_iv_init` | 1 |

### SUBTLE

**Total calls:** 26

| Function | Calls |
|----------|-------|
| `ct_eq` | 13 |
| `from` | 6 |
| `into_option` | 1 |
| `unwrap_u8` | 1 |
| `from` | 1 |
| `conditional_assign` | 1 |
| `ct_ne` | 1 |
| `from` | 1 |
| `conditional_select` | 1 |
