add_mlir_library(MLIRSymbolicToArith
  SymbolicToArith.cpp

  DEPENDS
    MLIRSymbolicToArithPassIncGen

  LINK_COMPONENTS
    Core

  LINK_LIBS PUBLIC
    MLIRSymbolicDialect
    MLIRArithDialect
    MLIRFuncDialect
    MLIRFuncTransforms
    MLIRIR
    MLIRPass
    MLIRTransforms
)
