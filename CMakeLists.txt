cmake_minimum_required (VERSION 3.8)

project ("Soft Render")

add_executable(SoftRender
	"Soft Render.cpp"
	"Soft Render.h"
	"src/Buffer.hpp"
	"src/Engine.hpp")
target_include_directories(SoftRender PUBLIC
	"${CMAKE_SOURCE_DIR}/external/SDL2/include")
target_link_libraries(SoftRender PUBLIC
	"${CMAKE_SOURCE_DIR}/external/SDL2/lib/x64/SDL2.lib"
	"${CMAKE_SOURCE_DIR}/external/SDL2/lib/x64/SDL2main.lib")
