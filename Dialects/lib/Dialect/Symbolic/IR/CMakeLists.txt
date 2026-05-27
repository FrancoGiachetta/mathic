add_mlir_library(MLIRSymbolicDialect
  SymbolicDialect.cpp
  SymbolicOps.cpp
  SymbolicTypes.cpp

  DEPENDS
    MLIRSymbolicDialectIncGen
    MLIRSymbolicOpsIncGen
    MLIRSymbolicTypesIncGen

  LINK_LIBS PUBLIC
    MLIRIR
    MLIRSupport
    MLIRParser
    symengine
)
