fn main() {
    println!("cargo:rerun-if-changed=schema/shared/*");

    ::capnpc::CompilerCommand::new()
        .src_prefix("schema/shared")
        .file("schema/shared/store.capnp")
        .file("schema/shared/entry.capnp")
        .file("schema/shared/network.capnp")
        .run()
        .expect("Failed to compile Cap'n Proto schema");
}
