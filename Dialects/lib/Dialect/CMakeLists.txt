add_subdirectory(Symbolic)

add_library(dialect_bindings capi.cpp)
target_include_directories(dialect_bindings
  PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/include/Dialect/capi.h
)
target_link_libraries(dialect_bindings
  PUBLIC
    MLIRIR
    MLIRPass
    MLIRSymbolicDialect
    MLIRSymbolicTransforms
  PRIVATE
    MLIRSupport
)
