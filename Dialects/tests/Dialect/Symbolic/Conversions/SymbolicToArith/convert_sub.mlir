// RUN: dialect-driver --symbolic-to-arith %s | FileCheck %s

// CHECK-LABEL: func.func private @test_sub
// CHECK-SAME: (%arg0: f64) -> f64 {
// CHECK-NEXT:   %0 = arith.subf %arg0, %arg0 : f64
// CHECK-NEXT:   return %0 : f64

func.func private @test_sub(%arg0: f64) -> !symbolic.expr {
  %0 = symbolic.sym "x" : !symbolic.expr
  %1 = symbolic.sub %0, %0 : !symbolic.expr
  return %1 : !symbolic.expr
}
