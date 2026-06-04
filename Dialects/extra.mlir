module {
func.func @test_basic_extract(%x_val: f64) -> f64 {
    %x = symbolic.sym "x" : !symbolic.expr

    %xx = symbolic.mul %x, %x : !symbolic.expr
    %r = symbolic.mul %xx, %x : !symbolic.expr
    %res = symbolic.eval %r, "x", %x_val : f64 -> f64
   
    %x2 = symbolic.add %x, %x : !symbolic.expr
    %r2 = symbolic.mul %x2, %x : !symbolic.expr
    %res2 = symbolic.eval %r2, "x", %x_val : f64 -> f64

    %res3 = arith.addf %res2, %res : f64
    return %res3 : f64
  }
}
