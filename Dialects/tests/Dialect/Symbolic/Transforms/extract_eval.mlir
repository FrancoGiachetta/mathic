// RUN: dialect-driver --symbolic-extract-eval %s | FileCheck %s

module {
  // CHECK: func private @__eval_op_
  // CHECK-SAME: (%arg0: f64) -> !symbolic.expr {
  // CHECK-NEXT:   %0 = symbolic.sym "x" : !symbolic.expr
  // CHECK-NEXT:   %1 = symbolic.mul %0, %0 : !symbolic.expr
  // CHECK-NEXT:   %2 = symbolic.mul %1, %0 : !symbolic.expr
  // CHECK-NEXT:   return %2 : !symbolic.expr
  // CHECK-NEXT: }

  // CHECK-LABEL: func @test_basic_extract
  func.func @test_basic_extract(%x_val: f64) -> f64 {
    %x = symbolic.sym "x" : !symbolic.expr
    %xx = symbolic.mul %x, %x : !symbolic.expr
    %r = symbolic.mul %xx, %x : !symbolic.expr
    // CHECK: call @__eval_op_
    %res = symbolic.eval %r, "x", %x_val : f64 -> f64
    return %res : f64
  }

  // CHECK-LABEL: func @test_dedup
  func.func @test_dedup(%a: f64, %b: f64, %c: f64) -> f64 {
    %x = symbolic.sym "x" : !symbolic.expr
    %xx = symbolic.mul %x, %x : !symbolic.expr
    // CHECK: call @__eval_op_
    %r1 = symbolic.eval %xx, "x", %a : f64 -> f64
    // CHECK: call @__eval_op_
    %r2 = symbolic.eval %xx, "x", %b : f64 -> f64
    // CHECK: call @__eval_op_
    %r3 = symbolic.eval %xx, "x", %c : f64 -> f64
    return %r3 : f64
  }
}
