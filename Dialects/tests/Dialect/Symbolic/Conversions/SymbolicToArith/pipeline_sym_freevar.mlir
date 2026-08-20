// RUN: dialect-driver --symbolic-extract-eval --symbolic-to-arith %s | FileCheck %s

module {
  // CHECK:      func.func private @__eval_op_
  // CHECK-SAME: (%arg0: i32, %arg1: i32) -> i32 {
  // CHECK-NEXT:   %0 = arith.addi %arg0, %arg0 : i32
  // CHECK-NEXT:   %1 = arith.addi %0, %arg1 : i32
  // CHECK-NEXT:   return %1 : i32
  // CHECK-NEXT: }

  // CHECK-LABEL: func @test_sym_freevar
  func.func @test_sym_freevar(%x_val: i32, %b: i32) -> i32 {
    %x = symbolic.sym "x" : !symbolic.expr<i32, isSigned = true>
    %e = symbolic.add %x, %x : (!symbolic.expr<i32, isSigned = true>, !symbolic.expr<i32, isSigned = true>) -> !symbolic.expr<i32, isSigned = true>
    %s = symbolic.add %e, %b : (!symbolic.expr<i32, isSigned = true>, i32) -> !symbolic.expr<i32, isSigned = true>
    // CHECK: call @__eval_op_
    %res = symbolic.eval %s, ["x"], %x_val : (!symbolic.expr<i32, isSigned = true>, i32) -> i32
    // CHECK: return
    return %res : i32
  }

  // CHECK-NOT: symbolic.
  // CHECK-NOT: unrealized_conversion_cast
}