cmake_minimum_required(VERSION 3.9 FATAL_ERROR)

project(workflow_inliner)

file(GLOB SOURCES
	*.cpp
	*.c
	*.h)

add_library(workflow_inliner SHARED ${SOURCES})

target_link_libraries(workflow_inliner binaryninjaapi)

set_target_properties(workflow_inliner PROPERTIES
	CXX_STANDARD 17
	CXX_VISIBILITY_PRESET hidden
	CXX_STANDARD_REQUIRED ON
	C_STANDARD 99
	C_STANDARD_REQUIRED ON
	C_VISIBILITY_PRESET hidden
	VISIBILITY_INLINES_HIDDEN ON
	POSITION_INDEPENDENT_CODE ON)

if(BN_INTERNAL_BUILD)
	plugin_rpath(workflow_inliner)
	set_target_properties(workflow_inliner PROPERTIES
		LIBRARY_OUTPUT_DIRECTORY ${BN_CORE_PLUGIN_DIR}
		RUNTIME_OUTPUT_DIRECTORY ${BN_CORE_PLUGIN_DIR})
endif()
