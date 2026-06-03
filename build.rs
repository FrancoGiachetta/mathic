use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    // let dialects_root = root.join("Dialects");
    // let build_dir = dialects_root.join("builds/build");

    // std::fs::create_dir_all(&build_dir).unwrap();

    // let llvm_prefix = env::var("LLVM_SYS_211_PREFIX").unwrap();

    // // ── Build ─────────────────────────────────────────────────────────────

    // let status = Command::new("cmake")
    //     .arg(&dialects_root)
    //     .arg(format!("-DMLIR_DIR={}/lib/cmake/mlir", llvm_prefix))
    //     .arg(format!("-DLLVM_DIR={}/lib/cmake/llvm", llvm_prefix))
    //     .arg("-DBUILD_SHARED_LIBS=ON")
    //     .arg("-S")
    //     .arg(&dialects_root)
    //     .arg("-B")
    //     .arg(&build_dir)
    //     .arg("-DCMAKE_POLICY_VERSION_MINIMUM=3.10")
    //     .status()
    //     .expect("cmake configure failed");
    // assert!(status.success(), "cmake configure failed");

    // let status = Command::new("cmake")
    //     .arg("--build")
    //     .arg(&build_dir)
    //     .status()
    //     .expect("cmake build failed");
    // assert!(status.success(), "cmake build failed");

    // // ── Link ──────────────────────────────────────────────────────────────────

    // let lib_dir = build_dir.join("lib/");
    // println!("cargo:rustc-link-search=native={}", lib_dir.display());
    // println!("cargo:rustc-link-lib=dylib=MLIRSymbolicDialect");
    // println!("cargo:rustc-link-lib=dylib=MLIRSymbolicTransforms");
    // println!("cargo:rustc-link-lib=dylib=MLIRSymbolicToArith");

    // // ── Rerun triggers ────────────────────────────────────────────────────────

    // println!("cargo:rerun-if-changed=CMakeLists.txt");
    // println!("cargo:rerun-if-changed=include/Dialects");
    // println!("cargo:rerun-if-changed=lib/Dialects");
}
