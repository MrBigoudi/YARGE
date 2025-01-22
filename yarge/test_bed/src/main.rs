use platform_layer::{PlatformLayer, PlatformLayerImpl, Window};

fn main() {
    let mut platform = PlatformLayerImpl::init().unwrap();
    let window = platform.get_window(0);
    window.get_id();
    platform.shutdown().unwrap()
}
