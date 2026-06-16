// RUN: dialect-driver --symbolic-to-arith %s | FileCheck %s

// CHECK-LABEL: func.func private @test_sub
// CHECK-SAME: (%arg0: i32) -> i32 {
// CHECK-NEXT:   %0 = arith.subi %arg0, %arg0 : i32
// CHECK-NEXT:   return %0 : i32

func.func private @test_sub(%arg0: i32) -> !symbolic.expr<i32, isSigned = true> {
  %0 = symbolic.sym "x" : !symbolic.expr<i32, isSigned = true>
  %1 = symbolic.sub %0, %0 : (!symbolic.expr<i32, isSigned = true>, !symbolic.expr<i32, isSigned = true>) -> !symbolic.expr<i32, isSigned = true>
  return %1 : !symbolic.expr<i32, isSigned = true>
}
