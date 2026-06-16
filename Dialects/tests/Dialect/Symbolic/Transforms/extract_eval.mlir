// RUN: dialect-driver --symbolic-extract-eval %s | FileCheck %s

module {
  // CHECK:      func private @__eval_op_
  // CHECK-SAME: (%arg0: i32) -> !symbolic.expr<i32, isSigned = true> {
  // CHECK-NEXT:   %0 = symbolic.sym "x" : !symbolic.expr<i32, isSigned = true>
  // CHECK-NEXT:   %1 = symbolic.mul %0, %0 : (!symbolic.expr<i32, isSigned = true>, !symbolic.expr<i32, isSigned = true>) -> !symbolic.expr<i32, isSigned = true>
  // CHECK-NEXT:   %2 = symbolic.mul %1, %0 : (!symbolic.expr<i32, isSigned = true>, !symbolic.expr<i32, isSigned = true>) -> !symbolic.expr<i32, isSigned = true>
  // CHECK-NEXT:   return %2 : !symbolic.expr<i32, isSigned = true>
  // CHECK-NEXT: }

  // CHECK-LABEL: func @test_basic_extract
  func.func @test_basic_extract(%x_val: i32) -> i32 {
    %x = symbolic.sym "x" : !symbolic.expr<i32, isSigned = true>
    %xx = symbolic.mul %x, %x : (!symbolic.expr<i32, isSigned = true>, !symbolic.expr<i32, isSigned = true>) -> !symbolic.expr<i32, isSigned = true>
    %r = symbolic.mul %xx, %x : (!symbolic.expr<i32, isSigned = true>, !symbolic.expr<i32, isSigned = true>) -> !symbolic.expr<i32, isSigned = true>
    // CHECK: call @__eval_op_
    %res = symbolic.eval %r, "x", %x_val : (!symbolic.expr<i32, isSigned = true>, i32) -> i32
    return %res : i32
  }

  // CHECK-LABEL: func @test_dedup
  func.func @test_dedup(%a: i32, %b: i32, %c: i32) -> i32 {
    %x = symbolic.sym "x" : !symbolic.expr<i32, isSigned = true>
    %xx = symbolic.mul %x, %x : (!symbolic.expr<i32, isSigned = true>, !symbolic.expr<i32, isSigned = true>) -> !symbolic.expr<i32, isSigned = true>
    // CHECK: call @__eval_op_
    %r1 = symbolic.eval %xx, "x", %a : (!symbolic.expr<i32, isSigned = true>, i32) -> i32
    // CHECK: call @__eval_op_
    %r2 = symbolic.eval %xx, "x", %b : (!symbolic.expr<i32, isSigned = true>, i32) -> i32
    // CHECK: call @__eval_op_
    %r3 = symbolic.eval %xx, "x", %c : (!symbolic.expr<i32, isSigned = true>, i32) -> i32
    return %r3 : i32
  }
}
