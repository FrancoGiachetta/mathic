add_mlir_library(MLIRSymbolicTransforms
  SymbolicCanonicalize.cpp

  DEPENDS
    MLIRSymbolicTransformsIncGen

  LINK_LIBS PUBLIC
    MLIRSymbolicDialect
    MLIRTransforms
    MLIRPass
)
