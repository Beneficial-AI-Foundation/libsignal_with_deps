//
// Copyright 2023 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

fn main() {
    let protos = [
        "src/proto/backup_metadata.proto",
        "src/proto/chat_websocket.proto",
        "src/proto/cds2.proto",
        "src/proto/chat_noise.proto",
    ];
    prost_build::Config::new()
        .bytes([".signal.proto.chat_noise", ".signal.proto.chat_websocket"])
        .compile_protos(&protos, &["src"])
        .expect("Protobufs in src are valid");
    for proto in &protos {
        println!("cargo:rerun-if-changed={proto}");
    }
}
