use melior::{Context, ir::Module};

use crate::{
    MathicResult, codegen::error::CodegenError, error::MathicError, parser::grammar::Program,
};

pub mod error;
pub mod expression;
pub mod statement;

pub struct MathicCodeGen<'this, 'ctx>
where
    'this: 'ctx,
{
    module: &'this Module<'ctx>,
}

impl<'this, 'ctx> MathicCodeGen<'this, 'ctx>
where
    'this: 'ctx,
{
    pub fn new(module: &'this Module<'ctx>) -> Self {
        Self { module }
    }

    pub fn generate_module(&mut self, ctx: &'ctx Context, program: Program) -> MathicResult<()> {
        // Check if main function is present
        if !program.funcs.iter().any(|f| f.name == "main") {
            return Err(MathicError::Codegen(CodegenError::MissingMainFunction));
        }

        // TODO: Compile structs in the future

        for func in program.funcs {
            self.compile_function(ctx, func)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        compiler::{MathicCompiler, OptLvl},
        executor::MathicExecutor,
    };

    #[test]
    fn test_numeric_literal_compilation() {
        let compiler = MathicCompiler::new().expect("Failed to create compiler");

        // Test various numeric literals
        let test_cases = vec![
            ("df main() { return 0; }", 0),
            ("df main() { return 1; }", 1),
            ("df main() { return 123; }", 123),
            ("df main() { return 42; }", 42),
            (
                "df main() { return 18446744073709551615; }",
                u64::MAX as i64,
            ), // u64::MAX
            ("df main() { return 9223372036854775807; }", i64::MAX), // i64::MAX
        ];

        for (source, expected) in test_cases {
            let module = compiler
                .compile_source(source, OptLvl::default())
                .expect(&format!("Failed to compile: {}", source));

            // Execute and verify result using executor
            let executor =
                MathicExecutor::new(&module, OptLvl::default()).expect("Failed to create executor");
            let result = executor
                .call_function("main")
                .expect("Failed to execute main");
            assert_eq!(result, expected, "Unexpected result for: {}", source);
        }
    }

    #[test]
    fn test_boolean_literal_compilation() {
        let compiler = MathicCompiler::new().expect("Failed to create compiler");

        // Test boolean literals (true = 1, false = 0)
        let test_cases = vec![
            ("df main() { return true; }", 1),
            ("df main() { return false; }", 0),
        ];

        for (source, expected) in test_cases {
            let module = compiler
                .compile_source(source, OptLvl::default())
                .expect(&format!("Failed to compile: {}", source));

            let executor =
                MathicExecutor::new(&module, OptLvl::default()).expect("Failed to create executor");
            let result = executor
                .call_function("main")
                .expect("Failed to execute main");
            assert_eq!(result, expected, "Unexpected result for: {}", source);
        }
    }
    #[test]
    fn test_logical_expressions_compilation() {
        let compiler = MathicCompiler::new().expect("Failed to create compiler");

        // Test boolean literals (true = 1, false = 0)
        let test_cases = vec![
            ("df main() { return true and true; }", 1),
            ("df main() { return false and false; }", 0),
            ("df main() { return false and true; }", 0),
            ("df main() { return true or true; }", 1),
            ("df main() { return false or false; }", 0),
            ("df main() { return false or true; }", 1),
        ];

        for (source, expected) in test_cases {
            let module = compiler
                .compile_source(source, OptLvl::default())
                .expect(&format!("Failed to compile: {}", source));

            let executor =
                MathicExecutor::new(&module, OptLvl::default()).expect("Failed to create executor");
            let result = executor
                .call_function("main")
                .expect("Failed to execute main");
            assert_eq!(result, expected, "Unexpected result for: {}", source);
        }
    }
}
