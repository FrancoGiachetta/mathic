module attributes {llvm.data_layout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-n32:64-S128-Fn32", llvm.target_triple = "arm64-apple-darwin25.4.0"} {
  llvm.func @"mathic__program::main"() -> i32 attributes {sym_visibility = "private"} {
    %0 = llvm.mlir.constant(10 : i32) : i32
    %1 = llvm.mlir.constant(1 : i64) : i64
    %2 = llvm.mlir.undef : !llvm.struct<(i32, i32)>
    %3 = llvm.mlir.constant(5 : i32) : i32
    %4 = llvm.insertvalue %3, %2[0] : !llvm.struct<(i32, i32)> 
    %5 = llvm.insertvalue %3, %4[1] : !llvm.struct<(i32, i32)> 
    %6 = llvm.alloca %1 x !llvm.struct<(i32, i32)> {alignment = 32 : i64} : (i64) -> !llvm.ptr
    llvm.store %5, %6 : !llvm.struct<(i32, i32)>, !llvm.ptr
    %7 = llvm.insertvalue %0, %2[0] : !llvm.struct<(i32, i32)> 
    %8 = llvm.insertvalue %0, %7[1] : !llvm.struct<(i32, i32)> 
    %9 = llvm.alloca %1 x !llvm.struct<(i32, i32)> {alignment = 32 : i64} : (i64) -> !llvm.ptr
    llvm.store %8, %9 : !llvm.struct<(i32, i32)>, !llvm.ptr
    %10 = llvm.load %6 : !llvm.ptr -> !llvm.struct<(i32, i32)>
    %11 = llvm.load %9 : !llvm.ptr -> !llvm.struct<(i32, i32)>
    %12 = llvm.alloca %1 x !llvm.struct<(i32, i32)> {alignment = 32 : i64} : (i64) -> !llvm.ptr
    %13 = llvm.call @"mathic__program::add"(%10, %11) : (!llvm.struct<(i32, i32)>, !llvm.struct<(i32, i32)>) -> !llvm.struct<(i32, i32)>
    llvm.store %13, %12 : !llvm.struct<(i32, i32)>, !llvm.ptr
    %14 = llvm.load %12 : !llvm.ptr -> !llvm.struct<(i32, i32)>
    %15 = llvm.alloca %1 x !llvm.struct<(i32, i32)> {alignment = 32 : i64} : (i64) -> !llvm.ptr
    llvm.store %14, %15 : !llvm.struct<(i32, i32)>, !llvm.ptr
    %16 = llvm.load %15 : !llvm.ptr -> !llvm.struct<(i32, i32)>
    %17 = llvm.extractvalue %16[0] : !llvm.struct<(i32, i32)> 
    llvm.return %17 : i32
  }
  llvm.func @"mathic__program::add"(%arg0: !llvm.struct<(i32, i32)>, %arg1: !llvm.struct<(i32, i32)>) -> !llvm.struct<(i32, i32)> attributes {sym_visibility = "private"} {
    %0 = llvm.mlir.undef : !llvm.struct<(i32, i32)>
    %1 = llvm.mlir.constant(1 : i64) : i64
    %2 = llvm.alloca %1 x !llvm.struct<(i32, i32)> {alignment = 8 : i64} : (i64) -> !llvm.ptr
    llvm.store %arg0, %2 : !llvm.struct<(i32, i32)>, !llvm.ptr
    %3 = llvm.alloca %1 x !llvm.struct<(i32, i32)> {alignment = 8 : i64} : (i64) -> !llvm.ptr
    llvm.store %arg1, %3 : !llvm.struct<(i32, i32)>, !llvm.ptr
    %4 = llvm.load %2 : !llvm.ptr -> !llvm.struct<(i32, i32)>
    %5 = llvm.extractvalue %4[0] : !llvm.struct<(i32, i32)> 
    %6 = llvm.load %3 : !llvm.ptr -> !llvm.struct<(i32, i32)>
    %7 = llvm.extractvalue %6[0] : !llvm.struct<(i32, i32)> 
    %8 = llvm.add %5, %7 : i32
    %9 = llvm.load %2 : !llvm.ptr -> !llvm.struct<(i32, i32)>
    %10 = llvm.extractvalue %9[1] : !llvm.struct<(i32, i32)> 
    %11 = llvm.load %3 : !llvm.ptr -> !llvm.struct<(i32, i32)>
    %12 = llvm.extractvalue %11[1] : !llvm.struct<(i32, i32)> 
    %13 = llvm.add %10, %12 : i32
    %14 = llvm.insertvalue %8, %0[0] : !llvm.struct<(i32, i32)> 
    %15 = llvm.insertvalue %13, %14[1] : !llvm.struct<(i32, i32)> 
    llvm.return %15 : !llvm.struct<(i32, i32)>
  }
  llvm.func @"mathic__program::new"(%arg0: i32, %arg1: i32) -> !llvm.struct<(i32, i32)> attributes {sym_visibility = "private"} {
    %0 = llvm.mlir.undef : !llvm.struct<(i32, i32)>
    %1 = llvm.mlir.constant(1 : i64) : i64
    %2 = llvm.alloca %1 x i32 {alignment = 8 : i64} : (i64) -> !llvm.ptr
    llvm.store %arg0, %2 : i32, !llvm.ptr
    %3 = llvm.alloca %1 x i32 {alignment = 8 : i64} : (i64) -> !llvm.ptr
    llvm.store %arg1, %3 : i32, !llvm.ptr
    %4 = llvm.load %2 : !llvm.ptr -> i32
    %5 = llvm.load %3 : !llvm.ptr -> i32
    %6 = llvm.insertvalue %4, %0[0] : !llvm.struct<(i32, i32)> 
    %7 = llvm.insertvalue %5, %6[1] : !llvm.struct<(i32, i32)> 
    llvm.return %7 : !llvm.struct<(i32, i32)>
  }
}
