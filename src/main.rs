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

fn main() {
    let proj_name = std::env::args().nth(1).expect("no project name given");

    let curr_dir = std::env::current_dir().expect("failed to get current directory");
    let proj_path = curr_dir.join(&proj_name);

    if !proj_path.exists() {
        fs::create_dir_all(&proj_path).expect("failed to create project directory");
        println!("project directory created: {:?}", proj_path.display());
    } else {
        println!(
            "project directory already exists: {:?}",
            proj_path.display()
        );
    }

    let src_path = proj_path.join("src");
    fs::create_dir_all(&src_path).expect("failed to create src directory");
    println!("src directory created: {:?}", src_path.display());

    let file_path = src_path.join("main.cpp");
    let mut cpp_file = File::create(&file_path).expect("failed to create file");
    writeln!(cpp_file, "{}", CPP_DRAFT).expect("failed to init cpp demo file");

    let cmakelists_path = proj_path.join("CMakeLists.txt");
    let mut cmakelists_file =
        File::create(&cmakelists_path).expect("failed to create CMakeLists.txt");
    writeln!(
        cmakelists_file,
        "{}",
        CMAKELISTS_DRAFT.replace("@app_name@", &proj_name)
    )
    .expect("failed to init CMakeLists.txt");
}
