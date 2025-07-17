# LibSignal Dependency Analysis Report

This report analyzes function calls from `rust/` code to `deps/` code in the LibSignal project.

## 📊 Overall Statistics

- **Total rust functions calling deps:** 75
- **Total deps functions called:** 136
- **Total rust files using deps:** 33
- **Total dependency calls:** 232

## 🔥 Top 20 Most Called Deps Functions

| Rank | Calls | Function | Crate | Path |
|------|-------|----------|-------|------|
| 1 | 13 | `ct_eq` | subtle | `deps/subtle/src/lib.rs` |
| 2 | 6 | `from` | subtle | `deps/subtle/src/lib.rs` |
| 3 | 6 | `from_curve_name` | boring-signal | `deps/boring-signal/src/ec.rs` |
| 4 | 5 | `sha256` | boring-signal | `deps/boring-signal/src/hash.rs` |
| 5 | 5 | `serial_number` | boring-signal | `deps/boring-signal/src/x509/mod.rs` |
| 6 | 4 | `days_from_now` | boring-signal | `deps/boring-signal/src/asn1.rs` |
| 7 | 4 | `from_unix` | boring-signal | `deps/boring-signal/src/asn1.rs` |
| 8 | 3 | `build` | boring-signal | `deps/boring-signal/src/ssl/connector.rs` |
| 9 | 3 | `configure` | boring-signal | `deps/boring-signal/src/ssl/connector.rs` |
| 10 | 3 | `builder` | boring-signal | `deps/boring-signal/src/ssl/connector.rs` |
| 11 | 3 | `tls_client` | boring-signal | `deps/boring-signal/src/ssl/mod.rs` |
| 12 | 3 | `public_key` | boring-signal | `deps/boring-signal/src/x509/mod.rs` |
| 13 | 3 | `to_owned` | boring-signal | `deps/boring-signal/src/x509/crl.rs` |
| 14 | 3 | `subject_name` | boring-signal | `deps/boring-signal/src/x509/mod.rs` |
| 15 | 3 | `compare` | boring-signal | `deps/boring-signal/src/asn1.rs` |
| 16 | 3 | `private_key_from_der` | boring-signal | `deps/boring-signal/src/pkey.rs` |
| 17 | 3 | `not_after` | boring-signal | `deps/boring-signal/src/x509/mod.rs` |
| 18 | 3 | `not_before` | boring-signal | `deps/boring-signal/src/x509/mod.rs` |
| 19 | 3 | `new` | boring-signal | `deps/boring-signal/src/x509/store.rs` |
| 20 | 3 | `add_cert` | boring-signal | `deps/boring-signal/src/x509/store.rs` |

## 📦 Deps Crate Usage

| Crate | Calls | Path Pattern |
|-------|-------|--------------|
| `boring-signal` | 205 | `deps/boring-signal/` |
| `subtle` | 26 | `deps/subtle/` |
| `ctr` | 1 | `deps/ctr/` |

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
| 7 | `rust/net/infra/src/tcp_ssl.rs` |
| 7 | `rust/net/infra/src/tcp_ssl/proxy.rs` |
| 6 | `rust/attest/src/dcap/revocation_list.rs` |
| 3 | `rust/zkgroup/src/crypto/profile_key_encryption.rs` |
| 3 | `rust/net/infra/src/errors.rs` |
| 3 | `rust/attest/src/dcap/ecdsa.rs` |
| 2 | `rust/core/src/curve.rs` |
| 2 | `rust/message-backup/src/frame.rs` |

## 🔍 Detailed Breakdown by Deps Crate

### BORING-SIGNAL

**Total calls:** 205

| Function | Calls | Path |
|----------|-------|------|
| `from_curve_name` | 6 | `deps/boring-signal/src/ec.rs` |
| `sha256` | 5 | `deps/boring-signal/src/hash.rs` |
| `serial_number` | 5 | `deps/boring-signal/src/x509/mod.rs` |
| `days_from_now` | 4 | `deps/boring-signal/src/asn1.rs` |
| `from_unix` | 4 | `deps/boring-signal/src/asn1.rs` |
| `build` | 3 | `deps/boring-signal/src/ssl/connector.rs` |
| `configure` | 3 | `deps/boring-signal/src/ssl/connector.rs` |
| `builder` | 3 | `deps/boring-signal/src/ssl/connector.rs` |
| `tls_client` | 3 | `deps/boring-signal/src/ssl/mod.rs` |
| `public_key` | 3 | `deps/boring-signal/src/x509/mod.rs` |
| ... | ... | ... |
| *116 more functions* | | |

### CTR

**Total calls:** 1

| Function | Calls | Path |
|----------|-------|------|
| `inner_iv_init` | 1 | `deps/ctr/src/ctr_core.rs` |

### SUBTLE

**Total calls:** 26

| Function | Calls | Path |
|----------|-------|------|
| `ct_eq` | 13 | `deps/subtle/src/lib.rs` |
| `from` | 6 | `deps/subtle/src/lib.rs` |
| `conditional_assign` | 1 | `deps/subtle/src/lib.rs` |
| `from` | 1 | `deps/subtle/src/lib.rs` |
| `unwrap_u8` | 1 | `deps/subtle/src/lib.rs` |
| `ct_ne` | 1 | `deps/subtle/src/lib.rs` |
| `into_option` | 1 | `deps/subtle/src/lib.rs` |
| `from` | 1 | `deps/subtle/src/lib.rs` |
| `conditional_select` | 1 | `deps/subtle/src/lib.rs` |
