// RUN: dialect-driver --symbolic-to-arith %s | FileCheck %s

// CHECK-LABEL: func.func private @test_all_ops
// CHECK-SAME: (%arg0: f64) -> f64 {
// CHECK-NEXT:   %0 = arith.addf %arg0, %arg0 : f64
// CHECK-NEXT:   %1 = arith.subf %arg0, %arg0 : f64
// CHECK-NEXT:   %2 = arith.mulf %0, %1 : f64
// CHECK-NEXT:   %3 = arith.divf %2, %arg0 : f64
// CHECK-NEXT:   return %3 : f64

func.func private @test_all_ops(%arg0: f64) -> !symbolic.expr {
  %0 = symbolic.sym "x" : !symbolic.expr
  %1 = symbolic.add %0, %0 : !symbolic.expr
  %2 = symbolic.sub %0, %0 : !symbolic.expr
  %3 = symbolic.mul %1, %2 : !symbolic.expr
  %4 = symbolic.div %3, %0 : !symbolic.expr
  return %4 : !symbolic.expr
}
