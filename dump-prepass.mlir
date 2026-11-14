module attributes {llvm.data_layout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-n32:64-S128-Fn32", llvm.target_triple = "arm64-apple-darwin25.4.0"} {
  func.func private @"mathic__program::main"() -> i32 {
    %0 = llvm.mlir.undef : !llvm.struct<(i32, i32)>
    %c5_i32 = arith.constant 5 : i32
    %c5_i32_0 = arith.constant 5 : i32
    %1 = llvm.insertvalue %c5_i32, %0[0] : !llvm.struct<(i32, i32)> 
    %2 = llvm.insertvalue %c5_i32_0, %1[1] : !llvm.struct<(i32, i32)> 
    %c1_i64 = arith.constant 1 : i64
    %3 = llvm.alloca %c1_i64 x !llvm.struct<(i32, i32)> {alignment = 32 : i64} : (i64) -> !llvm.ptr
    llvm.store %2, %3 : !llvm.struct<(i32, i32)>, !llvm.ptr
    %4 = llvm.mlir.undef : !llvm.struct<(i32, i32)>
    %c10_i32 = arith.constant 10 : i32
    %c10_i32_1 = arith.constant 10 : i32
    %5 = llvm.insertvalue %c10_i32, %4[0] : !llvm.struct<(i32, i32)> 
    %6 = llvm.insertvalue %c10_i32_1, %5[1] : !llvm.struct<(i32, i32)> 
    %c1_i64_2 = arith.constant 1 : i64
    %7 = llvm.alloca %c1_i64_2 x !llvm.struct<(i32, i32)> {alignment = 32 : i64} : (i64) -> !llvm.ptr
    llvm.store %6, %7 : !llvm.struct<(i32, i32)>, !llvm.ptr
    %8 = llvm.load %3 : !llvm.ptr -> !llvm.struct<(i32, i32)>
    %9 = llvm.load %7 : !llvm.ptr -> !llvm.struct<(i32, i32)>
    %c1_i64_3 = arith.constant 1 : i64
    %10 = llvm.alloca %c1_i64_3 x !llvm.struct<(i32, i32)> {alignment = 32 : i64} : (i64) -> !llvm.ptr
    %11 = call @"mathic__program::add"(%8, %9) : (!llvm.struct<(i32, i32)>, !llvm.struct<(i32, i32)>) -> !llvm.struct<(i32, i32)>
    llvm.store %11, %10 : !llvm.struct<(i32, i32)>, !llvm.ptr
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %12 = llvm.load %10 : !llvm.ptr -> !llvm.struct<(i32, i32)>
    %c1_i64_4 = arith.constant 1 : i64
    %13 = llvm.alloca %c1_i64_4 x !llvm.struct<(i32, i32)> {alignment = 32 : i64} : (i64) -> !llvm.ptr
    llvm.store %12, %13 : !llvm.struct<(i32, i32)>, !llvm.ptr
    %14 = llvm.load %13 : !llvm.ptr -> !llvm.struct<(i32, i32)>
    %15 = llvm.extractvalue %14[0] : !llvm.struct<(i32, i32)> 
    return %15 : i32
  }
  func.func private @"mathic__program::add"(%arg0: !llvm.struct<(i32, i32)>, %arg1: !llvm.struct<(i32, i32)>) -> !llvm.struct<(i32, i32)> {
    %c1_i64 = arith.constant 1 : i64
    %0 = llvm.alloca %c1_i64 x !llvm.struct<(i32, i32)> {alignment = 8 : i64} : (i64) -> !llvm.ptr
    llvm.store %arg0, %0 : !llvm.struct<(i32, i32)>, !llvm.ptr
    %c1_i64_0 = arith.constant 1 : i64
    %1 = llvm.alloca %c1_i64_0 x !llvm.struct<(i32, i32)> {alignment = 8 : i64} : (i64) -> !llvm.ptr
    llvm.store %arg1, %1 : !llvm.struct<(i32, i32)>, !llvm.ptr
    %2 = llvm.mlir.undef : !llvm.struct<(i32, i32)>
    %3 = llvm.load %0 : !llvm.ptr -> !llvm.struct<(i32, i32)>
    %4 = llvm.extractvalue %3[0] : !llvm.struct<(i32, i32)> 
    %5 = llvm.load %1 : !llvm.ptr -> !llvm.struct<(i32, i32)>
    %6 = llvm.extractvalue %5[0] : !llvm.struct<(i32, i32)> 
    %7 = arith.addi %4, %6 : i32
    %8 = llvm.load %0 : !llvm.ptr -> !llvm.struct<(i32, i32)>
    %9 = llvm.extractvalue %8[1] : !llvm.struct<(i32, i32)> 
    %10 = llvm.load %1 : !llvm.ptr -> !llvm.struct<(i32, i32)>
    %11 = llvm.extractvalue %10[1] : !llvm.struct<(i32, i32)> 
    %12 = arith.addi %9, %11 : i32
    %13 = llvm.insertvalue %7, %2[0] : !llvm.struct<(i32, i32)> 
    %14 = llvm.insertvalue %12, %13[1] : !llvm.struct<(i32, i32)> 
    return %14 : !llvm.struct<(i32, i32)>
  }
  func.func private @"mathic__program::new"(%arg0: i32, %arg1: i32) -> !llvm.struct<(i32, i32)> {
    %c1_i64 = arith.constant 1 : i64
    %0 = llvm.alloca %c1_i64 x i32 {alignment = 8 : i64} : (i64) -> !llvm.ptr
    llvm.store %arg0, %0 : i32, !llvm.ptr
    %c1_i64_0 = arith.constant 1 : i64
    %1 = llvm.alloca %c1_i64_0 x i32 {alignment = 8 : i64} : (i64) -> !llvm.ptr
    llvm.store %arg1, %1 : i32, !llvm.ptr
    %2 = llvm.mlir.undef : !llvm.struct<(i32, i32)>
    %3 = llvm.load %0 : !llvm.ptr -> i32
    %4 = llvm.load %1 : !llvm.ptr -> i32
    %5 = llvm.insertvalue %3, %2[0] : !llvm.struct<(i32, i32)> 
    %6 = llvm.insertvalue %4, %5[1] : !llvm.struct<(i32, i32)> 
    return %6 : !llvm.struct<(i32, i32)>
  }
}
