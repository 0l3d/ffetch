cmake_minimum_required(VERSION 3.15)

project(ffetch_build_wrapper NONE)

add_custom_target(build ALL
	COMMAND ${CMAKE_COMMAND} -E env CARGO_TARGET_DIR=${CMAKE_SOURCE_DIR}/target cargo build --locked
	WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
	USES_TERMINAL
)
