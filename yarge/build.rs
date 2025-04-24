use cfg_aliases::cfg_aliases;

fn main() {
    // The script doesn't depend on our code
    println!("cargo:rerun-if-changed=build.rs");

    // Setup cfg aliases
    cfg_aliases! {
        linux_platform: { target_os = "linux" },
        x11_platform: { all(feature = "x11", linux_platform) },
        wayland_platform: { all(feature = "wayland", linux_platform) },
        web_platform: { all(target_family = "wasm", target_os = "unknown") },
        windows_platform: { target_os = "windows" },
        macos_platform: { target_os = "macos" },
    }

    // TODO: merge all config files into a giant config file

    // TODO: compile shaders
}
