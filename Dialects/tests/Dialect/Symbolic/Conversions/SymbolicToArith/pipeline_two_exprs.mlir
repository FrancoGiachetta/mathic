// RUN: dialect-driver --symbolic-extract-eval --symbolic-to-arith %s | FileCheck %s

// CHECK: func.func private @__eval_op_
// CHECK-SAME: (%arg0: i32) -> i32 {
// CHECK-NEXT:   %0 = arith.muli %arg0, %arg0 : i32
// CHECK-NEXT:   %1 = arith.muli %0, %arg0 : i32
// CHECK-NEXT:   return %1 : i32
// CHECK-NEXT: }

// CHECK: func.func private @__eval_op_
// CHECK-SAME: (%arg0: i32) -> i32 {
// CHECK-NEXT:   %0 = arith.addi %arg0, %arg0 : i32
// CHECK-NEXT:   %1 = arith.muli %0, %arg0 : i32
// CHECK-NEXT:   return %1 : i32
// CHECK-NEXT: }

// CHECK-LABEL: func.func @test_two_exprs
// CHECK:         call @__eval_op_
// CHECK:         call @__eval_op_
// CHECK:         %2 = arith.addi
// CHECK:         return %2 : i32
// CHECK-NOT:     symbolic.
// CHECK-NOT:     unrealized_conversion_cast

func.func @test_two_exprs(%x_val: i32) -> i32 {
  %x = symbolic.sym "x" : !symbolic.expr<i32, isSigned = true>
  %r1 = symbolic.mul %x, %x : (!symbolic.expr<i32, isSigned = true>, !symbolic.expr<i32, isSigned = true>) -> !symbolic.expr<i32, isSigned = true>
  %r2 = symbolic.mul %r1, %x : (!symbolic.expr<i32, isSigned = true>, !symbolic.expr<i32, isSigned = true>) -> !symbolic.expr<i32, isSigned = true>
  %res1 = symbolic.eval %r2, "x", %x_val : (!symbolic.expr<i32, isSigned = true>, i32) -> i32
  %x2 = symbolic.add %x, %x : (!symbolic.expr<i32, isSigned = true>, !symbolic.expr<i32, isSigned = true>) -> !symbolic.expr<i32, isSigned = true>
  %r3 = symbolic.mul %x2, %x : (!symbolic.expr<i32, isSigned = true>, !symbolic.expr<i32, isSigned = true>) -> !symbolic.expr<i32, isSigned = true>
  %res2 = symbolic.eval %r3, "x", %x_val : (!symbolic.expr<i32, isSigned = true>, i32) -> i32
  %res3 = arith.addi %res1, %res2 : i32
  return %res3 : i32
}
