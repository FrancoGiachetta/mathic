// RUN: dialect-driver --symbolic-to-arith %s | FileCheck %s

// CHECK-LABEL: func.func private @test_all_ops
// CHECK-SAME: (%arg0: i32) -> i32 {
// CHECK-NEXT:   %0 = arith.addi %arg0, %arg0 : i32
// CHECK-NEXT:   %1 = arith.subi %arg0, %arg0 : i32
// CHECK-NEXT:   %2 = arith.muli %0, %1 : i32
// CHECK-NEXT:   %3 = arith.divsi %2, %arg0 : i32
// CHECK-NEXT:   return %3 : i32

func.func private @test_all_ops(%arg0: i32) -> !symbolic.expr<i32, isSigned = true> {
  %0 = symbolic.sym "x" : !symbolic.expr<i32, isSigned = true>
  %1 = symbolic.add %0, %0 : (!symbolic.expr<i32, isSigned = true>, !symbolic.expr<i32, isSigned = true>) -> !symbolic.expr<i32, isSigned = true>
  %2 = symbolic.sub %0, %0 : (!symbolic.expr<i32, isSigned = true>, !symbolic.expr<i32, isSigned = true>) -> !symbolic.expr<i32, isSigned = true>
  %3 = symbolic.mul %1, %2 : (!symbolic.expr<i32, isSigned = true>, !symbolic.expr<i32, isSigned = true>) -> !symbolic.expr<i32, isSigned = true>
  %4 = symbolic.div %3, %0 : (!symbolic.expr<i32, isSigned = true>, !symbolic.expr<i32, isSigned = true>) -> !symbolic.expr<i32, isSigned = true>
  return %4 : !symbolic.expr<i32, isSigned = true>
}
