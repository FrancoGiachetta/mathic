// RUN: dialect-driver --symbolic-to-arith %s | FileCheck %s

// CHECK-LABEL: func.func private @test_div_i32_operands
// CHECK-SAME: (%arg0: i32) -> i32 {
// CHECK-NEXT:   %0 = arith.divsi %arg0, %arg0 : i32
// CHECK-NEXT:   return %0 : i32

func.func private @test_div_i32_operands(%arg0: i32) -> !symbolic.expr<i32, isSigned = true> {
  %0 = symbolic.div %arg0, %arg0 : (i32, i32) -> !symbolic.expr<i32, isSigned = true>
  return %0 : !symbolic.expr<i32, isSigned = true>
}