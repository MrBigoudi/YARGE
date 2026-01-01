use cfg_aliases::cfg_aliases;

fn main() {
    // The script doesn't depend on our code
    println!("cargo:rerun-if-changed=build.rs");

    // Setup cfg aliases
    cfg_aliases! {
        // Platforms
        linux_platform: { target_os = "linux" },
        x11_platform: { all(feature = "x11", linux_platform) },
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

    // TODO: merge all config files into a giant config file

    // TODO: compile shaders
}
