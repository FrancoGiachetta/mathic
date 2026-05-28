message(STATUS "Using lit: ${LIT_TOOL}")
message(STATUS "Using FileCheck: ${FILECHECK_TOOL}")

configure_lit_site_cfg(
  ${CMAKE_CURRENT_SOURCE_DIR}/lit.site.cfg.py.in
  ${CMAKE_CURRENT_BINARY_DIR}/lit.cfg.py
  MAIN_CONFIG
)

set(LLVM_EXTERNAL_LIT /home/linuxbrew/.linuxbrew/opt/lit/bin/lit CACHE STRING "Description" FORCE)

add_lit_testsuite(check-dialects "Running Symbolic Dialect Tests..."
  ${CMAKE_CURRENT_BINARY_DIR}

  DEPENDS
    FileCheck count not
    mlir-opt
    mlir-runner
    test-runner
)

