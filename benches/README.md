# Benchmarks

Compares symbolic expression evaluation time between:

- **Mathic** — JIT-compiles `eval(expr, sym, value)` to native code via MLIR/LLVM. Benchmarked at four optimization levels: `None`, `O1`, `O2`, `O3`.
- **SymPy** — `lambdify(expr, 'numpy')` evaluated with the same input.

## Expression

| Expression | `x` | Result |
|------------|-----|--------|
| `x * (x + x + x + x)` | `10` | `400` |

## How to run

```bash
pip install sympy
cargo bench
```

## Output

Criterion reports per call for each combination of framework and optimization level.
