add_mlir_library(MLIRSymbolicTransforms
  SymbolicExtractEval.cpp

  DEPENDS
    MLIRSymbolicTransformsIncGen

  LINK_COMPONENTS
    Core

  LINK_LIBS PUBLIC
    MLIRSymbolicDialect
    MLIRTransforms
    MLIRPass
    MLIRFuncDialect
)
