cmake_minimum_required(VERSION 3.13)
project(actix_web)

set(CMAKE_CXX_STANDARD 14)

add_executable(actix_web
        app/src/main.rs
        app/Cargo.toml
        db/src/lib.rs
        db/Cargo.toml
        src/main.rs
        view/src/lib.rs
        view/Cargo.toml
        Cargo.toml)
