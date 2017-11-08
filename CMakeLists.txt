cmake_minimum_required(VERSION 3.8)
project(rbsegs)

set(CMAKE_CXX_STANDARD 11)

set(SOURCE_FILES
        src/lib.rs
        target/debug/deps/redblue_segs-f63c87c88b495860
        target/debug/redblue_segs-f63c87c88b495860
        target/debug/redblue_segs-f63c87c88b495860.d
        Cargo.lock
        Cargo.toml
        rbsegs.iml
        README.md)

add_executable(rbsegs ${SOURCE_FILES})