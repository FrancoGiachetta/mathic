// RUN: test-runner %s > %t
// RUN: FileCheck %s < %t

module {
  // CHECK-LABEL: func @test_var
  func.func @test_var() -> !symbolic.expr {
    // CHECK: symbolic.sym "x" : !symbolic.expr
    %x = symbolic.sym "x" : !symbolic.expr
    return %x : !symbolic.expr
  }

  // CHECK-LABEL: func @test_add
  func.func @test_add() -> !symbolic.expr {
    %x = symbolic.sym "x" : !symbolic.expr
    %y = symbolic.sym "y" : !symbolic.expr
    // CHECK: symbolic.add
    %res = symbolic.add %x, %y : !symbolic.expr
    return %res : !symbolic.expr
  }

  // CHECK-LABEL: func @test_diff
  func.func @test_diff() -> !symbolic.expr {
    %x = symbolic.sym "x" : !symbolic.expr
    %x2 = symbolic.mul %x, %x : !symbolic.expr
    // CHECK: symbolic.diff
    %res = symbolic.diff %x2, "x" : !symbolic.expr
    return %res : !symbolic.expr
  }
}
