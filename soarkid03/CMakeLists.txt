cmake_minimum_required(VERSION 3.10)

project(soarkid03)

add_executable(soarkid03 main.cpp)

target_link_libraries(soarkid03 ws2_32 userenv ntdll ../../target/i686-pc-windows-msvc/debug/soarkid03)
