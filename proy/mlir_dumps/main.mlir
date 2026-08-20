module attributes {llvm.data_layout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128", llvm.target_triple = "x86_64-unknown-linux-gnu"} {
  func.func private @"mathic__main::main"() -> i32 {
    %0 = symbolic.sym "x" : !symbolic.expr<i32, isSigned = true>
    %1 = symbolic.mul %0, %0 : (!symbolic.expr<i32, isSigned = true>, !symbolic.expr<i32, isSigned = true>) -> !symbolic.expr<i32, isSigned = true>
    %2 = symbolic.div %0, %0 : (!symbolic.expr<i32, isSigned = true>, !symbolic.expr<i32, isSigned = true>) -> !symbolic.expr<i32, isSigned = true>
    %3 = symbolic.add %1, %2 : (!symbolic.expr<i32, isSigned = true>, !symbolic.expr<i32, isSigned = true>) -> !symbolic.expr<i32, isSigned = true>
    %c10_i32 = arith.constant 10 : i32
    %4 = symbolic.eval %3, ["x"], %c10_i32 : (!symbolic.expr<i32, isSigned = true>, i32) -> i32
    %c1_i64 = arith.constant 1 : i64
    %5 = llvm.alloca %c1_i64 x i32 {alignment = 32 : i64} : (i64) -> !llvm.ptr
    llvm.store %4, %5 : i32, !llvm.ptr
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %6 = llvm.load %5 : !llvm.ptr -> i32
    %c1_i64_0 = arith.constant 1 : i64
    %7 = llvm.alloca %c1_i64_0 x i32 {alignment = 32 : i64} : (i64) -> !llvm.ptr
    llvm.store %6, %7 : i32, !llvm.ptr
    %8 = llvm.load %7 : !llvm.ptr -> i32
    return %8 : i32
  }
}
