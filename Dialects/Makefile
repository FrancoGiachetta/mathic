BUILDS_ROOT=builds
BUILD_DIR=${BUILDS_ROOT}/build

.PHONY: build
build: clean
	cmake -S . -B ${BUILD_DIR} -DCMAKE_POLICY_VERSION_MINIMUM=3.10
	cmake --build ${BUILD_DIR}

.PHONY: test
test: clean
	cmake -S . -B ${BUILD_DIR} -DBUILD_TESTING=ON -DCMAKE_POLICY_VERSION_MINIMUM=3.10
	cmake --build ${BUILD_DIR} --target check-dialects

.PHONY: fmt
fmt:
	find . \
		\( -iname 'vcpkg' -o -iname 'builds' \) -prune \
		-o -type f \( -iname '*.h' -o -iname '*.cpp' -o -iname '*.hpp' \) -print \
		| clang-format --style=Microsoft -i --files=/dev/stdin

.PHONY: clean
clean:
	rm -rf ${BUILDS_ROOT} .cache
	rm -f compile_commands.json
