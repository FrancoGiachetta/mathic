# ── GiNaC ─────────────────────────────────────────────────────────────

pkg_check_modules(GINAC REQUIRED IMPORTED_TARGET ginac)

# ── Create dialect library ────────────────────────────────────────────

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
)
