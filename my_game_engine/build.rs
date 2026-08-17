use std::path::Path;

fn main() {
    // Candidates for finding opengl_wrapper_lib source and header files relative to crate root
    let c_candidates = [
        "../starter/opengl_wrapper_lib/opengl_wrapper_lib.c",
        "../opengl_wrapper_lib/opengl_wrapper_lib.c",
        "starter/opengl_wrapper_lib/opengl_wrapper_lib.c",
        "opengl_wrapper_lib/opengl_wrapper_lib.c",
    ];

    let h_candidates = [
        "../starter/opengl_wrapper_lib/opengl_wrapper_lib.h",
        "../opengl_wrapper_lib/opengl_wrapper_lib.h",
        "starter/opengl_wrapper_lib/opengl_wrapper_lib.h",
        "opengl_wrapper_lib/opengl_wrapper_lib.h",
    ];

    let c_src = c_candidates
        .iter()
        .find(|&&p| Path::new(p).exists())
        .copied()
        .unwrap_or_else(|| panic!("Could not find opengl_wrapper_lib.c in any known relative path"));

    let h_src = h_candidates
        .iter()
        .find(|&&p| Path::new(p).exists())
        .copied()
        .unwrap_or_else(|| panic!("Could not find opengl_wrapper_lib.h in any known relative path"));

    // Compile C wrapper source file into static library `libopenglwrapper.a`
    cc::Build::new().file(c_src).compile("openglwrapper");

    // Link dynamic system dependencies (GLFW and OpenGL)
    println!("cargo:rustc-link-lib=glfw");
    println!("cargo:rustc-link-lib=GL");

    // Re-run build script if C source or header files change
    println!("cargo:rerun-if-changed={}", c_src);
    println!("cargo:rerun-if-changed={}", h_src);
}
