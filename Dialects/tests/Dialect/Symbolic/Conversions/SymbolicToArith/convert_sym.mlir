// RUN: dialect-driver --symbolic-to-arith %s | FileCheck %s

// CHECK-LABEL: func.func private @test_sym
// CHECK-SAME: (%arg0: i32) -> i32 {
// CHECK-NEXT:   return %arg0 : i32

func.func private @test_sym(%arg0: i32) -> !symbolic.expr<i32, isSigned = true> {
  %0 = symbolic.sym "x" : !symbolic.expr<i32, isSigned = true>
  return %0 : !symbolic.expr<i32, isSigned = true>
}
