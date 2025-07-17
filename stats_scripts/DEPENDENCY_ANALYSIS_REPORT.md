# LibSignal Dependency Analysis Report

This report analyzes function calls from `rust/` code to `deps/` code in the LibSignal project.

## 📊 Overall Statistics

- **Total rust functions calling deps:** 163
- **Total deps functions called:** 176
- **Total rust files using deps:** 59
- **Total dependency calls:** 373

## 🔥 Top 20 Most Called Deps Functions

| Rank | Calls | Function | Crate | Path |
|------|-------|----------|-------|------|
| 1 | 14 | `ct_eq` | subtle | `deps/subtle/src/lib.rs` |
| 2 | 13 | `compress` | curve25519-dalek | `deps/curve25519-dalek/src/ristretto.rs` |
| 3 | 9 | `identity` | curve25519-dalek | `deps/curve25519-dalek/src/ristretto.rs` |
| 4 | 9 | `from_bytes_mod_order_wide` | curve25519-dalek | `deps/curve25519-dalek/src/scalar.rs` |
| 5 | 8 | `as_bytes` | curve25519-dalek | `deps/curve25519-dalek/src/ristretto.rs` |
| 6 | 7 | `to_bytes` | curve25519-dalek | `deps/curve25519-dalek/src/ristretto.rs` |
| 7 | 7 | `invert` | curve25519-dalek | `deps/curve25519-dalek/src/scalar.rs` |
| 8 | 6 | `from` | subtle | `deps/subtle/src/lib.rs` |
| 9 | 6 | `from_curve_name` | boring-signal | `deps/boring-signal/src/ec.rs` |
| 10 | 5 | `serial_number` | boring-signal | `deps/boring-signal/src/x509/mod.rs` |
| 11 | 5 | `compress` | curve25519-dalek | `deps/curve25519-dalek/src/edwards.rs` |
| 12 | 5 | `from_bytes_mod_order` | curve25519-dalek | `deps/curve25519-dalek/src/scalar.rs` |
| 13 | 5 | `sha256` | boring-signal | `deps/boring-signal/src/hash.rs` |
| 14 | 4 | `decompress` | curve25519-dalek | `deps/curve25519-dalek/src/edwards.rs` |
| 15 | 4 | `from_canonical_bytes` | curve25519-dalek | `deps/curve25519-dalek/src/scalar.rs` |
| 16 | 4 | `build` | boring-signal | `deps/boring-signal/src/ssl/connector.rs` |
| 17 | 4 | `configure` | boring-signal | `deps/boring-signal/src/ssl/connector.rs` |
| 18 | 4 | `new` | aes-gcm-siv | `deps/aes-gcm-siv/src/lib.rs` |
| 19 | 4 | `tls_client` | boring-signal | `deps/boring-signal/src/ssl/mod.rs` |
| 20 | 4 | `builder` | boring-signal | `deps/boring-signal/src/ssl/connector.rs` |

## 📦 Deps Crate Usage

| Crate | Calls | Path Pattern |
|-------|-------|--------------|
| `boring-signal` | 209 | `deps/boring-signal/` |
| `curve25519-dalek` | 124 | `deps/curve25519-dalek/` |
| `subtle` | 25 | `deps/subtle/` |
| `ed25519-dalek` | 7 | `deps/ed25519-dalek/` |
| `aes-gcm-siv` | 5 | `deps/aes-gcm-siv/` |
| `chacha20poly1305` | 2 | `deps/chacha20poly1305/` |
| `ctr` | 1 | `deps/ctr/` |

## 📝 Crate References in Code Bodies

| Crate | References |
|-------|------------|
| `aes` | 236 |
| `cbc` | 118 |
| `ctr` | 65 |
| `curve25519_dalek` | 34 |
| `hmac` | 155 |
| `sha2` | 178 |
| `subtle` | 4 |

## 📁 Top 15 Rust Files by Deps Usage

| Deps Calls | File |
|------------|------|
| 67 | `rust/attest/src/cert_chain.rs` |
| 31 | `rust/attest/src/dcap.rs` |
| 23 | `rust/svrb/src/lib.rs` |
| 20 | `rust/zkcredential/src/endorsements.rs` |
| 18 | `rust/device-transfer/src/lib.rs` |
| 18 | `rust/attest/src/dcap/fakes.rs` |
| 15 | `rust/attest/src/dcap/sgx_quote.rs` |
| 15 | `rust/core/src/curve/curve25519.rs` |
| 15 | `rust/net/infra/src/certs.rs` |
| 13 | `rust/keytrans/src/vrf.rs` |
| 11 | `rust/device-transfer/tests/tests.rs` |
| 9 | `rust/poksho/src/statement.rs` |
| 9 | `rust/usernames/src/username.rs` |
| 7 | `rust/net/infra/src/tcp_ssl.rs` |
| 7 | `rust/net/infra/src/tcp_ssl/proxy.rs` |

## 🔍 Detailed Breakdown by Deps Crate

### AES-GCM-SIV

**Total calls:** 5

| Function | Calls | Path |
|----------|-------|------|
| `new` | 4 | `deps/aes-gcm-siv/src/lib.rs` |
| `encrypt_in_place_detached` | 1 | `deps/aes-gcm-siv/src/lib.rs` |

### BORING-SIGNAL

**Total calls:** 209

| Function | Calls | Path |
|----------|-------|------|
| `from_curve_name` | 6 | `deps/boring-signal/src/ec.rs` |
| `serial_number` | 5 | `deps/boring-signal/src/x509/mod.rs` |
| `sha256` | 5 | `deps/boring-signal/src/hash.rs` |
| `build` | 4 | `deps/boring-signal/src/ssl/connector.rs` |
| `configure` | 4 | `deps/boring-signal/src/ssl/connector.rs` |
| `tls_client` | 4 | `deps/boring-signal/src/ssl/mod.rs` |
| `builder` | 4 | `deps/boring-signal/src/ssl/connector.rs` |
| `days_from_now` | 4 | `deps/boring-signal/src/asn1.rs` |
| `from_unix` | 4 | `deps/boring-signal/src/asn1.rs` |
| `generate` | 3 | `deps/boring-signal/src/ec.rs` |
| ... | ... | ... |
| *116 more functions* | | |

### CHACHA20POLY1305

**Total calls:** 2

| Function | Calls | Path |
|----------|-------|------|
| `new` | 2 | `deps/chacha20poly1305/src/lib.rs` |

### CTR

**Total calls:** 1

| Function | Calls | Path |
|----------|-------|------|
| `inner_iv_init` | 1 | `deps/ctr/src/ctr_core.rs` |

### CURVE25519-DALEK

**Total calls:** 124

| Function | Calls | Path |
|----------|-------|------|
| `compress` | 13 | `deps/curve25519-dalek/src/ristretto.rs` |
| `identity` | 9 | `deps/curve25519-dalek/src/ristretto.rs` |
| `from_bytes_mod_order_wide` | 9 | `deps/curve25519-dalek/src/scalar.rs` |
| `as_bytes` | 8 | `deps/curve25519-dalek/src/ristretto.rs` |
| `to_bytes` | 7 | `deps/curve25519-dalek/src/ristretto.rs` |
| `invert` | 7 | `deps/curve25519-dalek/src/scalar.rs` |
| `compress` | 5 | `deps/curve25519-dalek/src/edwards.rs` |
| `from_bytes_mod_order` | 5 | `deps/curve25519-dalek/src/scalar.rs` |
| `decompress` | 4 | `deps/curve25519-dalek/src/edwards.rs` |
| `from_canonical_bytes` | 4 | `deps/curve25519-dalek/src/scalar.rs` |
| ... | ... | ... |
| *26 more functions* | | |

### ED25519-DALEK

**Total calls:** 7

| Function | Calls | Path |
|----------|-------|------|
| `from_bytes` | 3 | `deps/ed25519-dalek/src/verifying.rs` |
| `as_bytes` | 3 | `deps/ed25519-dalek/src/verifying.rs` |
| `verify` | 1 | `deps/ed25519-dalek/src/verifying.rs` |

### SUBTLE

**Total calls:** 25

| Function | Calls | Path |
|----------|-------|------|
| `ct_eq` | 14 | `deps/subtle/src/lib.rs` |
| `from` | 6 | `deps/subtle/src/lib.rs` |
| `conditional_select` | 1 | `deps/subtle/src/lib.rs` |
| `conditional_assign` | 1 | `deps/subtle/src/lib.rs` |
| `from` | 1 | `deps/subtle/src/lib.rs` |
| `unwrap_u8` | 1 | `deps/subtle/src/lib.rs` |
| `ct_ne` | 1 | `deps/subtle/src/lib.rs` |
