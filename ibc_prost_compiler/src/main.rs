use std::fs::remove_dir_all;
use std::fs::{copy, create_dir_all};
use std::path::{Path, PathBuf};
use tempdir::TempDir;
use walkdir::WalkDir;

fn main() {
    let in_dir = "ibc_proto/src/prost";
    let out_dir = TempDir::new("ibc_proto_out").unwrap();

    compile_protos(&out_dir);
    copy_generated_files(&in_dir, &out_dir);
}

fn compile_protos(out_dir: impl AsRef<Path>) {
    println!("[info ] Compiling .proto files to Rust...");

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
    let mut config = prost_build::Config::default();
    config.out_dir(out_dir.as_ref());
    config.compile_protos(&protos, &includes).unwrap();

    println!("[info ] => Done!");
}

fn copy_generated_files(ibc_proto_path: impl AsRef<Path>, out_dir: impl AsRef<Path>) {
    println!(
        "[info ] Copying generated files into '{}'...",
        ibc_proto_path.as_ref().display()
    );

    // Remove old compiled files
    remove_dir_all(&ibc_proto_path).unwrap_or_default();
    create_dir_all(&ibc_proto_path).unwrap();

    // Copy new compiled files (prost does not use folder structures)
    let err: Vec<std::io::Error> = WalkDir::new(out_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| {
            copy(
                e.path(),
                format!(
                    "{}/{}",
                    ibc_proto_path.as_ref().display(),
                    &e.file_name().to_os_string().to_str().unwrap()
                ),
            )
        })
        .filter_map(|e| e.err())
        .collect();

    if !err.is_empty() {
        for e in err {
            println!("[error] error while copying compiled file: {}", e);
        }

        panic!("error while copying compiled files")
    }

    println!("[info ] => Done!");
}
