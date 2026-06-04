// RUN: dialect-driver --symbolic-extract-eval --symbolic-to-arith %s | FileCheck %s

// CHECK: func.func private @__eval_op_
// CHECK-SAME: (%arg0: f64) -> f64 {
// CHECK-NEXT:   %0 = arith.mulf %arg0, %arg0 : f64
// CHECK-NEXT:   %1 = arith.mulf %0, %arg0 : f64
// CHECK-NEXT:   return %1 : f64
// CHECK-NEXT: }

// CHECK: func.func private @__eval_op_
// CHECK-SAME: (%arg0: f64) -> f64 {
// CHECK-NEXT:   %0 = arith.addf %arg0, %arg0 : f64
// CHECK-NEXT:   %1 = arith.mulf %0, %arg0 : f64
// CHECK-NEXT:   return %1 : f64
// CHECK-NEXT: }

// CHECK-LABEL: func.func @test_two_exprs
// CHECK:         call @__eval_op_
// CHECK:         call @__eval_op_
// CHECK:         %2 = arith.addf
// CHECK:         return %2 : f64
// CHECK-NOT:     symbolic.
// CHECK-NOT:     unrealized_conversion_cast

func.func @test_two_exprs(%x_val: f64) -> f64 {
  %x = symbolic.sym "x" : !symbolic.expr
  %r1 = symbolic.mul %x, %x : !symbolic.expr
  %r2 = symbolic.mul %r1, %x : !symbolic.expr
  %res1 = symbolic.eval %r2, "x", %x_val : f64 -> f64
  %x2 = symbolic.add %x, %x : !symbolic.expr
  %r3 = symbolic.mul %x2, %x : !symbolic.expr
  %res2 = symbolic.eval %r3, "x", %x_val : f64 -> f64
  %res3 = arith.addf %res1, %res2 : f64
  return %res3 : f64
}
