add_subdirectory(Symbolic)

add_library(dialect_bindings capi.cpp)
target_include_directories(dialect_bindings
  PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/include/Dialect/capi.h
)
target_link_libraries(dialect_bindings
  PUBLIC
    MLIRSymbolicDialect
    MLIRSymbolicTransforms
    MLIRSymbolicToArith
  PRIVATE
    MLIRIR
    MLIRPass
    MLIRSupport
)
