add_mlir_library(MLIRSymbolicTransforms
  SymbolicExtractEval.cpp

  DEPENDS
    MLIRSymbolicDialect
    MLIRSymbolicOpsIncGen
    MLIRSymbolicTypesIncGen
    MLIRSymbolicTransformsIncGen

  LINK_COMPONENTS
    Core

  LINK_LIBS PUBLIC
    MLIRTransforms
    MLIRPass
    MLIRFuncDialect
)
