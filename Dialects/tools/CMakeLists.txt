get_property(dialect_libs GLOBAL PROPERTY MLIR_DIALECT_LIBS)
get_property(conversion_libs GLOBAL PROPERTY MLIR_CONVERSION_LIBS)

set(LIBS
    ${dialect_libs}
    ${conversion_libs}
    MLIROptLib
    MLIRPass
    MLIRSymbolicDialect
    MLIRSymbolicTransforms
    MLIRSymbolicToArith
    MLIRRegisterAllDialects
    MLIRRegisterAllPasses
)

add_llvm_executable(dialect-driver dialect-driver.cpp)

llvm_update_compile_flags(dialect-driver)
target_link_libraries(dialect-driver PRIVATE ${LIBS})

mlir_check_all_link_libraries(dialect-driver)
