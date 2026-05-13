add_mlir_library(MLIRSymbolicTransforms
  SymbolicPasses.cpp

  DEPENDS
    MLIRSymbolicOpsIncGen

  LINK_LIBS PUBLIC
    MLIRSymbolicDialect
    MLIRTransforms
    MLIRPass
)
