module {
  func.func private @__eval_op_15174355473605916139(%arg0: f64) -> !symbolic.expr {
    %0 = symbolic.sym "x" : !symbolic.expr
    %1 = symbolic.mul %0, %0 : !symbolic.expr
    %2 = symbolic.mul %1, %0 : !symbolic.expr
    return %2 : !symbolic.expr
  }
  func.func private @__eval_op_13371367243461428439(%arg0: f64) -> !symbolic.expr {
    %0 = symbolic.sym "x" : !symbolic.expr
    %1 = symbolic.add %0, %0 : !symbolic.expr
    %2 = symbolic.mul %1, %0 : !symbolic.expr
    return %2 : !symbolic.expr
  }
  func.func @test_basic_extract(%arg0: f64) -> f64 {
    %0 = call @__eval_op_15174355473605916139(%arg0) : (f64) -> !symbolic.expr
    %1 = builtin.unrealized_conversion_cast %0 : !symbolic.expr to f64
    %2 = call @__eval_op_13371367243461428439(%arg0) : (f64) -> !symbolic.expr
    %3 = builtin.unrealized_conversion_cast %2 : !symbolic.expr to f64
    %4 = arith.addf %3, %1 : f64
    return %4 : f64
  }
}

