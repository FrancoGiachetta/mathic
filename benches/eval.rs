use criterion::{Criterion, black_box, criterion_group, criterion_main};
use mathic::compiler::{MathicCompiler, OptLvl};
use mathic::executor::MathicExecutor;
use pyo3::types::{PyAnyMethods, PyDict, PyDictMethods};
use std::ffi::CString;

const EXPRS: &[(&str, &str, i64)] = &[(
    "poly10",
    "df main() i32 {
        sym x:expr<i32>;
        let expr: expr<i32> = x;
        for i in 0..100 {
            expr = expr + x;
        }
        return eval(expr,x,10);
    }",
    1000,
)];

fn bench_mathic(c: &mut Criterion) {
    let compiler = MathicCompiler::new().unwrap();
    let opts = [OptLvl::None, OptLvl::O1, OptLvl::O2, OptLvl::O3];

    for opt in opts {
        let mut group = c.benchmark_group(format!("mathic_{opt:?}"));

        for &(name, src, expected) in EXPRS {
            let module = compiler.compile_source(src, opt, None).unwrap();
            let executor = MathicExecutor::new(&module, opt).unwrap();

            group.bench_function(name, |b| {
                b.iter(|| {
                    let res = executor.call_function("main").unwrap();
                    black_box(res);
                    assert_eq!(res, expected);
                });
            });
        }

        group.finish();
    }
}

fn bench_sympy(c: &mut Criterion) {
    pyo3::prepare_freethreaded_python();

    let exprs: &[(&str, i64)] = &[("sum([x for i in range(0,100)])", 1000)];

    pyo3::Python::with_gil(|py| {
        for (py_expr, expected) in exprs {
            let py_code = CString::new(format!(
                "
from sympy import lambdify, Symbol
x = Symbol('x')
lamb = lambdify(x, {py_expr}, modules='numpy')
res = lamb(10)
"
            ))
            .unwrap();
            let locals = PyDict::new(py);

            py.run(py_code.as_c_str(), None, Some(&locals)).unwrap();

            let res: i64 = locals.get_item("res").unwrap().unwrap().extract().unwrap();
            assert_eq!(res, *expected);

            let mut group = c.benchmark_group("sympy");
            group.bench_function(*py_expr, |b| {
                b.iter(|| {
                    py.run(py_code.as_c_str(), None, Some(&locals)).unwrap();

                    let res: i64 = locals.get_item("res").unwrap().unwrap().extract().unwrap();
                    black_box(res);
                });
            });
            group.finish();
        }
    });
}

criterion_group!(benches, bench_mathic, bench_sympy);
criterion_main!(benches);
