use cfg_aliases::cfg_aliases;

const INPUT_SHADERS_DIR: &str = "assets/shaders";
const OUTPUT_SHADERS_DIR: &str = "assets/shaders/compiled";

struct Shader<'a> {
    path: &'a str,
    entries: Vec<&'a str>,
}

fn compile_shader(shader: &Shader) {
    let input_path = std::path::PathBuf::from(INPUT_SHADERS_DIR).join(shader.path);

    let output_path = std::path::PathBuf::from(OUTPUT_SHADERS_DIR)
        .join(shader.path)
        .with_extension("spv");

    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }

    let mut cmd = std::process::Command::new("slangc");

    cmd.arg(&input_path)
        .arg("-target")
        .arg("spirv")
        .arg("-profile")
        .arg("spirv_1_4")
        .arg("-emit-spirv-directly")
        .arg("-fvk-use-entrypoint-name");

    for entry in &shader.entries {
        cmd.arg("-entry").arg(entry);
    }

    cmd.arg("-o").arg(&output_path);

    let status = cmd.status().unwrap();

    if !status.success() {
        panic!("Shader compilation failed for {:?}", shader.path);
    }
}

fn main() {
    // The script doesn't depend on our code
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=assets/shaders");

    // Sets up cfg aliases
    cfg_aliases! {
        // Platforms
        linux_platform: { target_os = "linux" },
        x11_platform: { all(feature = "x", linux_platform) },
        wayland_platform: { all(feature = "wayland", linux_platform) },
        web_platform: { all(target_family = "wasm", target_os = "unknown") },
        windows_platform: { target_os = "windows" },
        macos_platform: { target_os = "macos" },
        // Renderers
        vulkan_renderer: { all(feature = "vulkan") },
        opengl_renderer: { all(feature = "opengl") },
        wgpu_renderer: { all(feature = "wgpu") },
        directx_renderer: { all(feature = "directx") },
        metal_renderer: { all(feature = "metal") },
        // STD usage
        bare_metal: { all(feature = "no_std") },
    }

    // Find correct libraries
    if cfg!(all(feature = "opengl", target_os = "linux")) {
        pkg_config::Config::new()
            .probe("gl")
            .expect("Failed to find libGL in linux");
    }

    // TODO: merge all config files into a giant config file

    // TODO: compile engine shaders
    let shaders = vec![
        Shader {
            path: "vk_tuto.slang",
            entries: vec!["vert_main", "frag_main"],
        },
        // TODO: add other shaders or make a function to scrap the directory
        // The second option is more complexe as it needs to defer the entry points
    ];
    for shader in &shaders {
        compile_shader(shader);
    }
}
