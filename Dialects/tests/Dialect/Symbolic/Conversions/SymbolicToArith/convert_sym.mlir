// RUN: dialect-driver --symbolic-to-arith %s | FileCheck %s

// CHECK-LABEL: func.func private @test_sym
// CHECK-SAME: (%arg0: f64) -> f64 {
// CHECK-NEXT:   return %arg0 : f64

func.func private @test_sym(%arg0: f64) -> !symbolic.expr {
  %0 = symbolic.sym "x" : !symbolic.expr
  return %0 : !symbolic.expr
}
