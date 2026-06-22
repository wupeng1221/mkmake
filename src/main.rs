use clap::Parser;
use std::fs;
use std::fs::File;
use std::io::Write;

const CPP_DRAFT: &str = r#"
#include <iostream>

int main() {
    std::cout << "Hello, World!" << std::endl;
}
"#;

const CMAKELISTS_DRAFT: &str = r#"
CMAKE_MINIMUM_REQUIRED(VERSION 3.16)

# ============================================================================
# project name
# ============================================================================
PROJECT(@app_name@)

# ============================================================================
# c++ standard
# ============================================================================
SET(CMAKE_CXX_STANDARD 17)
SET(CMAKE_CXX_STANDARD_REQUIRED ON)

# ============================================================================
# output directory
# ============================================================================
SET(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

# ============================================================================
# source files
# ============================================================================
FILE(GLOB SRC_FILES
    "${CMAKE_SOURCE_DIR}/src/*.cpp"
    "${CMAKE_SOURCE_DIR}/src/*.c"
)

FILE(GLOB INC_FILES
    "${CMAKE_SOURCE_DIR}/src/*.hpp"
    "${CMAKE_SOURCE_DIR}/src/*.h"
)

# ============================================================================
# target
# ============================================================================
ADD_EXECUTABLE(@app_name@ ${SRC_FILES} ${INC_FILES})

# ============================================================================
# target includes
# ============================================================================
TARGET_INCLUDE_DIRECTORIES(@app_name@ PRIVATE ${CMAKE_SOURCE_DIR}/include)
"#;

const XMAKE_LUA: &str = r#"
set_project("@app_name@")
set_languages("cxx17")

target("@app_name@")
    set_kind("binary")
    add_files("src/**.cpp", "src/**.c")
    add_includedirs("src")
    set_targetdir("bin")
"#;

#[derive(Parser)]
#[command(
    name = "mkmake",
    version,
    about = "Create C++ demo project. Must specify --cmake or --xmake."
)]
#[command(group(
    clap::ArgGroup::new("build_system")
        .required(true)
        .args(["cmake", "xmake"])
))]
struct Cli {
    #[arg(short, long)]
    cmake: bool,

    #[arg(short, long)]
    xmake: bool,

    #[arg(required = true)]
    name: String,
}

fn main() {
    let cli = Cli::parse();

    let name = cli.name;

    let proj = std::env::current_dir().unwrap().join(&name);
    fs::create_dir_all(proj.join("src")).unwrap();

    File::create(proj.join("src/main.cpp"))
        .unwrap()
        .write_all(CPP_DRAFT.as_bytes())
        .unwrap();

    if cli.xmake {
        let content = XMAKE_LUA.replace("@app_name@", &name);
        File::create(proj.join("xmake.lua"))
            .unwrap()
            .write_all(content.as_bytes())
            .unwrap();
        println!("✅ xmake project: {}", name);
    } else {
        let content = CMAKELISTS_DRAFT.replace("@app_name@", &name);
        File::create(proj.join("CMakeLists.txt"))
            .unwrap()
            .write_all(content.as_bytes())
            .unwrap();
        println!("✅ cmake project: {}", name);
    }
}
