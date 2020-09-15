extern crate prost_build;

use std::path::PathBuf;
use walkdir::WalkDir;

fn main() {
    // Paths
    let proto_paths = [
        "../proto/mock",
        "../cosmos-sdk/proto/ibc",
        "../cosmos-sdk/proto/cosmos/tx",
        "../cosmos-sdk/proto/cosmos/base",
    ];
    let proto_includes_paths = [
        "../proto",
        "../cosmos-sdk/proto",
        "../cosmos-sdk/third_party/proto",
    ];

    // List available proto files
    let mut protos: Vec<PathBuf> = vec![];
    for proto_path in &proto_paths {
        protos.append(
            &mut WalkDir::new(proto_path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.file_type().is_file()
                        && e.path().extension().is_some()
                        && e.path().extension().unwrap() == "proto"
                })
                .map(|e| e.into_path())
                .collect(),
        );
    }

    // List available paths for dependencies
    let includes: Vec<PathBuf> = proto_includes_paths.iter().map(PathBuf::from).collect();

    // Compile all proto files
    prost_build::compile_protos(&protos, &includes).unwrap();
}
