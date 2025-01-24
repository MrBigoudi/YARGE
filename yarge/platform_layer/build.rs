use cfg_aliases::cfg_aliases;

fn main() {
    // The script doesn't depend on our code
    println!("cargo:rerun-if-changed=build.rs");

    // Setup cfg aliases
    cfg_aliases! {
        // Linux
        linux_platform: { target_os = "linux" },
        x11_platform: { all(feature = "x11", linux_platform) },
        wayland_platform: { all(feature = "wayland", linux_platform) },
        // Sony
        psvita_platform: {target_os = "vita" },
        sony_platform: { any(psvita_platform)},
        // Web
        web_platform: { all(target_family = "wasm", target_os = "unknown") },
        // Windows
        windows_platform: { target_os = "windows" },
        // Apple
        macos_platform: { target_os = "macos" },
    }
}
