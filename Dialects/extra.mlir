module {
func.func @test_basic_extract(%x_val: f64) -> f64 {
    %x = symbolic.sym "x" : !symbolic.expr
    %xx = symbolic.mul %x, %x : !symbolic.expr
    %r = symbolic.mul %xx, %x : !symbolic.expr
    %res = symbolic.eval %r, "x", %x_val : f64 -> f64

    return %res : f64
  }
}
