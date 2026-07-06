# Benchmarks

## Qué mide

Compara el tiempo de evaluación de expresiones simbólicas entre:

- **Mathic** — JIT-compila `eval(expr, sym, value)` a código nativo via MLIR. Se mide con OptLvl: None, O1, O2, O3.
- **SymPy** — `lambdify(expr, 'numpy')` evaluado con el mismo input.

Seis expresiones, todas con `x = 10`:

| Expresión | Resultado |
|---|---|
| `x + x` | 20 |
| `x * x` | 100 |
| `x * x + x` | 110 |
| `(x + x) * x` | 200 |
| `(x * x) / x` | 10 |
| `x * x + x / x` | 101 |

## Cómo correr

```bash
pip install sympy numpy
cargo bench
```

El output de Criterion reporta ns/call para cada combinación.
