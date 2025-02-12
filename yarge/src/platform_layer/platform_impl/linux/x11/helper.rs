#[cfg(debug_assertions)]
use x11rb::reexports::x11rb_protocol;

#[cfg(debug_assertions)]
pub fn xcb_screen_to_string(screen: &x11rb_protocol::protocol::xproto::Screen) -> String {
    use std::fmt::Write;

    
    let mut output = String::new();
    writeln!(&mut output, "xcb screen:").unwrap();
    writeln!(&mut output, "\troot: {}", screen.root).unwrap();
    writeln!(&mut output, "\tdefault_colormap: {}", screen.default_colormap).unwrap();
    writeln!(&mut output, "\twhite_pixel: {}", screen.white_pixel).unwrap();
    writeln!(&mut output, "\tblack_pixel: {}", screen.black_pixel).unwrap();
    writeln!(&mut output, "\tcurrent_input_masks: {:?}", screen.current_input_masks).unwrap();
    writeln!(&mut output, "\twidth_in_pixels: {}", screen.width_in_pixels).unwrap();
    writeln!(&mut output, "\theight_in_pixels: {}", screen.height_in_pixels).unwrap();
    writeln!(&mut output, "\twidth_in_millimeters: {}", screen.width_in_millimeters).unwrap();
    writeln!(&mut output, "\theight_in_millimeters: {}", screen.height_in_millimeters).unwrap();
    writeln!(&mut output, "\tmin_installed_maps: {}", screen.min_installed_maps).unwrap();
    writeln!(&mut output, "\tmax_installed_maps: {}", screen.max_installed_maps).unwrap();
    writeln!(&mut output, "\troot_visual: {}", screen.root_visual).unwrap();
    writeln!(&mut output, "\tbacking_stores: {:?}", screen.backing_stores).unwrap();
    writeln!(&mut output, "\tsave_unders: {}", screen.save_unders).unwrap();
    writeln!(&mut output, "\troot_depth: {}", screen.root_depth).unwrap();
    
    output
}