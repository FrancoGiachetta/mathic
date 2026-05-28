cmake_minimum_required(VERSION 3.20)
project(Mathic C CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fexceptions")

# ── MLIR / LLVM ──────────────────────────────────────────────────────────────

find_package(MLIR REQUIRED CONFIG)

message(STATUS "Using MLIR ${MLIR_VERSION} from ${MLIR_DIR}")

list(APPEND CMAKE_MODULE_PATH "${MLIR_CMAKE_DIR}")
list(APPEND CMAKE_MODULE_PATH "${LLVM_CMAKE_DIR}")
list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/cmake")

set(LLVM_ENABLE_EH ON)
set(LLVM_ENABLE_RTTI ON) 
set(LLVM_REQUIRES_EH ON) 

include(TableGen)
include(AddLLVM)
include(AddMLIR)
include(HandleLLVMOptions)

# ── SymEngine ────────────────────────────────────────────────────────────────

find_package(SymEngine CONFIG QUIET)
if(NOT SymEngine_FOUND)
  find_package(SymEngine REQUIRED)
endif()

message(STATUS "Using SymEngine from ${SYMENGINE_INCLUDE_DIRS}")

# ── Include paths ─────────────────────────────────────────────────────────────

include_directories(${LLVM_INCLUDE_DIRS})
include_directories(${MLIR_INCLUDE_DIRS})
include_directories(${CMAKE_CURRENT_SOURCE_DIR}/include)
include_directories(${CMAKE_CURRENT_BINARY_DIR}/include)

# ── Set Libraries Directory ───────────────────────────────────────────────────

set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY "${PROJECT_BINARY_DIR}/lib")

set(CMAKE_LIBRARY_OUTPUT_DIRECTORY "${PROJECT_BINARY_DIR}/lib")

message(STATUS "Using ARCHIVE_DIR ${CMAKE_ARCHIVE_OUTPUT_DIRECTORY}")
message(STATUS "Using LIBRARY_DIR ${CMAKE_LIBRARY_OUTPUT_DIRECTORY}")

# ── Subdirectories ────────────────────────────────────────────────────────────

add_subdirectory(tools)
add_subdirectory(include)
add_subdirectory(lib)

# ── Create symlink immediately during configuration ───────────────────────────

execute_process(
    COMMAND ${CMAKE_COMMAND} -E create_symlink
    ${CMAKE_BINARY_DIR}/compile_commands.json
    ${CMAKE_SOURCE_DIR}/compile_commands.json
)

# Testing

if(BUILD_TESTING)
    enable_testing()
    add_subdirectory(tests)
endif()
