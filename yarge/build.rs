use cfg_aliases::cfg_aliases;

fn main() {
    // The script doesn't depend on our code
    println!("cargo:rerun-if-changed=build.rs");

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

    // TODO: compile shaders
}
