// RUN: dialect-driver --symbolic-to-arith %s | FileCheck %s

// CHECK-LABEL: func.func private @test_div_i32_operands_unsigned
// CHECK-SAME: (%arg0: i32) -> i32 {
// CHECK-NEXT:   %0 = arith.divui %arg0, %arg0 : i32
// CHECK-NEXT:   return %0 : i32

func.func private @test_div_i32_operands_unsigned(%arg0: i32) -> !symbolic.expr<i32, isSigned = false> {
  %0 = symbolic.div %arg0, %arg0 : (i32, i32) -> !symbolic.expr<i32, isSigned = false>
  return %0 : !symbolic.expr<i32, isSigned = false>
}