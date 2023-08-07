cmake_minimum_required(VERSION 3.16)

project(repro-rocksdb-project
	VERSION 0.1.0
)

include(FetchContent)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.4.2
)
FetchContent_MakeAvailable(Corrosion)

corrosion_import_crate(
    MANIFEST_PATH "repro-rocksdb-lib/Cargo.toml"
)

add_executable(repro repro.cpp)
target_link_libraries(repro repro-rocksdb-lib-static)
