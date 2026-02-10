module attributes {llvm.data_layout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128", llvm.target_triple = "x86_64-unknown-linux-gnu"} {
  llvm.func @mathic_main() -> i64 attributes {llvm.emit_c_interface} {
    %0 = llvm.mlir.constant(42 : i64) : i64
    llvm.return %0 : i64
  }
  llvm.func @_mlir_ciface_mathic_main() -> i64 attributes {llvm.emit_c_interface} {
    %0 = llvm.call @mathic_main() : () -> i64
    llvm.return %0 : i64
  }
}
