// RUN: dialect-driver --symbolic-extract-eval %s | FileCheck %s

module {
  // CHECK:      func.func private @__eval_op_
  // CHECK-SAME: (%arg0: i32, %arg1: i32) -> !symbolic.expr<i32, isSigned = true> {
  // CHECK-NEXT:   %0 = symbolic.add %arg0, %arg1 : (i32, i32) -> !symbolic.expr<i32, isSigned = true>
  // CHECK-NEXT:   return %0 : !symbolic.expr<i32, isSigned = true>
  // CHECK-NEXT: }
  // CHECK-NEXT: func.func private @__eval_op_
  // CHECK-SAME: (%arg0: i32, %arg1: i32) -> !symbolic.expr<i32, isSigned = true> {
  // CHECK-NEXT:   %0 = symbolic.mul %arg1, %arg0 : (i32, i32) -> !symbolic.expr<i32, isSigned = true>
  // CHECK-NEXT:   return %0 : !symbolic.expr<i32, isSigned = true>
  // CHECK-NEXT: }

  // CHECK-LABEL: func @test_multi_sym
  func.func @test_multi_sym(%x_val: i32, %y_val: i32) -> i32 {
    %x = symbolic.sym "x" : !symbolic.expr<i32, isSigned = true>
    %y = symbolic.sym "y" : !symbolic.expr<i32, isSigned = true>
    %e = symbolic.add %x, %y : (!symbolic.expr<i32, isSigned = true>, !symbolic.expr<i32, isSigned = true>) -> !symbolic.expr<i32, isSigned = true>
    // CHECK: call @__eval_op_
    %res = symbolic.eval %e, ["x", "y"], %x_val, %y_val : (!symbolic.expr<i32, isSigned = true>, i32, i32) -> i32
    return %res : i32
  }

  // CHECK-LABEL: func @test_multi_reordered
  func.func @test_multi_reordered(%x_val: i32, %y_val: i32) -> i32 {
    %x = symbolic.sym "x" : !symbolic.expr<i32, isSigned = true>
    %y = symbolic.sym "y" : !symbolic.expr<i32, isSigned = true>
    %e = symbolic.mul %x, %y : (!symbolic.expr<i32, isSigned = true>, !symbolic.expr<i32, isSigned = true>) -> !symbolic.expr<i32, isSigned = true>
    // CHECK: call @__eval_op_
    %res = symbolic.eval %e, ["y", "x"], %y_val, %x_val : (!symbolic.expr<i32, isSigned = true>, i32, i32) -> i32
    return %res : i32
  }
}