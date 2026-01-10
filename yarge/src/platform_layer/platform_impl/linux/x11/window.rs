use std::collections::HashMap;

#[cfg(opengl_renderer)]
use crate::rendering_layer::rendering_impl::types::ImageFormat;

#[allow(unused)]
use crate::{
    config::Config,
    error::ErrorType,
    keyboard::KeyboardKey,
    log, log_debug, log_error, log_info,
    platform_layer::{DisplayMode, Event, Window, window::WindowCommonProperties},
};

use xcb::{Xid, x};

/// Handled atoms
pub struct LinuxX11Atoms {
    /// Tells the window manager what special messages you can understand
    /// Mainly use to announce support for window close
    pub protocols: x::Atom,
    /// Handles window close requests
    pub delete_window: x::Atom,
    /// Window states (fullscreen, maximized, minimized)
    pub state: x::Atom,
    /// Window maximized vertically
    pub state_maximized_vert: x::Atom,
    /// Window maximized horizontally
    pub state_maximized_horz: x::Atom,
    /// Window minimized
    pub state_hidden: x::Atom,
}

/// Properties of the X11 screen the window is attached to
pub struct LinuxX11ScreenProperties {
    /// The total screen width in pixels
    pub width: u16,
    /// The total screen height in pixels
    pub height: u16,
}

/// Properties to handle OpenGL with Linux X11
#[cfg(opengl_renderer)]
pub struct LinuxX11OpenglWindow {
    /// The xlib display
    pub display: *mut x11::xlib::Display,

    /// All the framebuffer configs
    pub all_framebuffer_configs: *mut x11::glx::GLXFBConfig,

    /// The selected framebuffer config
    pub framebuffer_config: *mut x11::glx::__GLXFBConfigRec,

    /// The OpenGL context
    pub context: *mut x11::glx::__GLXcontextRec,

    /// The Visual ID
    pub visual_id: std::os::raw::c_int,

    /// The colormap
    pub colormap: x::Colormap,

    /// The drawable window
    pub drawable: x11::glx::GLXDrawable,

    /// The GLX window
    pub window: x11::glx::GLXWindow,
}

#[cfg(opengl_renderer)]
impl LinuxX11OpenglWindow {
    /// Gets the glx render type bit from the framebuffer image format
    fn get_glx_render_type_bit(framebuffer_format: &ImageFormat) -> std::os::raw::c_int {
        match framebuffer_format {
            ImageFormat::R8G8B8A8_SFLOAT
            | ImageFormat::R8G8B8A8_UNORM
            | ImageFormat::R16G16B16A16_SFLOAT
            | ImageFormat::R16G16B16A16_UNORM
            | ImageFormat::R32G32B32A32_SFLOAT
            | ImageFormat::R32G32B32A32_UNORM => x11::glx::GLX_RGBA_BIT,
            _ => x11::glx::GLX_COLOR_INDEX_BIT,
        }
    }

    /// Gets the glx render type from the framebuffer image format
    fn get_glx_render_type(framebuffer_format: &ImageFormat) -> std::os::raw::c_int {
        match framebuffer_format {
            ImageFormat::R8G8B8A8_SFLOAT
            | ImageFormat::R8G8B8A8_UNORM
            | ImageFormat::R16G16B16A16_SFLOAT
            | ImageFormat::R16G16B16A16_UNORM
            | ImageFormat::R32G32B32A32_SFLOAT
            | ImageFormat::R32G32B32A32_UNORM => x11::glx::GLX_RGBA_TYPE,
            _ => x11::glx::GLX_COLOR_INDEX_TYPE,
        }
    }

    /// Gets the channel sizes from an image format
    fn get_channel_size(format: &ImageFormat) -> std::os::raw::c_int {
        format.get_channel_size() as std::os::raw::c_int
    }

    /// Gets the alphaa channel sizes from the framebuffer image format
    fn get_alpha_channel_size(framebuffer_format: &ImageFormat) -> Option<std::os::raw::c_int> {
        match framebuffer_format {
            ImageFormat::R8G8B8A8_SFLOAT
            | ImageFormat::R8G8B8A8_UNORM
            | ImageFormat::R16G16B16A16_SFLOAT
            | ImageFormat::R16G16B16A16_UNORM
            | ImageFormat::R32G32B32A32_SFLOAT
            | ImageFormat::R32G32B32A32_UNORM => {
                Some(framebuffer_format.get_channel_size() as std::os::raw::c_int)
            }
            _ => None,
        }
    }

    /// Creates the framebuffer configurations
    fn init_framebuffer_configurations(
        config: &Config,
        display: *mut x11::xlib::Display,
        screen_number: i32,
    ) -> Result<(*mut x11::glx::GLXFBConfig, std::os::raw::c_int), ErrorType> {
        let color_channel_size =
            Self::get_channel_size(&config.renderer_config.opengl_parameters.framebuffer_format);
        
        #[rustfmt::skip]
        // Use a fixed-size array on the stack
        const MAX_NB_ATTRIBUTES: usize = 32;
        let mut visual_attributes = [0i32; MAX_NB_ATTRIBUTES]; // Large enough for all possible attributes
        let mut idx = 0;
        
        // Helper to add attribute pairs
        let mut add_attrib = |attr: i32, value: i32| {
            if idx >= MAX_NB_ATTRIBUTES {
                log_error!("Trying to add too many attributes in the visual attributes array when initializing the X11 OpenGL window");
                return Err(ErrorType::InvalidIndex);
            }
            visual_attributes[idx] = attr;
            visual_attributes[idx + 1] = value;
            idx += 2;
            Ok(())
        };
        
        add_attrib(x11::glx::GLX_X_RENDERABLE, 1)?;
        add_attrib(x11::glx::GLX_DRAWABLE_TYPE, x11::glx::GLX_WINDOW_BIT)?;
        add_attrib(x11::glx::GLX_X_VISUAL_TYPE, x11::glx::GLX_TRUE_COLOR)?;
        add_attrib(x11::glx::GLX_DOUBLEBUFFER, 1)?;
        add_attrib(
            x11::glx::GLX_RENDER_TYPE,
            Self::get_glx_render_type_bit(
                &config.renderer_config.opengl_parameters.framebuffer_format,
            ),
        )?;
        add_attrib(x11::glx::GLX_RED_SIZE, color_channel_size)?;
        add_attrib(x11::glx::GLX_GREEN_SIZE, color_channel_size)?;
        add_attrib(x11::glx::GLX_BLUE_SIZE, color_channel_size)?;
        
        if let Some(alpha_size) = Self::get_alpha_channel_size(
            &config.renderer_config.opengl_parameters.framebuffer_format,
        ) {
            add_attrib(x11::glx::GLX_ALPHA_SIZE, alpha_size)?;
        }
        
        if let Some(format) = &config.renderer_config.opengl_parameters.depthbuffer_format {
            add_attrib(x11::glx::GLX_DEPTH_SIZE, Self::get_channel_size(format))?;
        }
        
        if let Some(format) = &config
            .renderer_config
            .opengl_parameters
            .stencilbuffer_format
        {
            add_attrib(x11::glx::GLX_STENCIL_SIZE, Self::get_channel_size(format))?;
        }
        
        // Null terminate the list
        if idx >= MAX_NB_ATTRIBUTES {
            log_error!("Invalid index for the end of the visual attributes array when initializing the X11 OpenGL window");
            return Err(ErrorType::InvalidIndex);
        }
        visual_attributes[idx] = x11::glx::GLX_NONE as i32;



        let mut nb_framebuffer_configs: std::os::raw::c_int = 0;
        let framebuffer_configs = unsafe {
            x11::glx::glXChooseFBConfig(
                display,
                screen_number,
                visual_attributes.as_ptr(),
                &mut nb_framebuffer_configs,
            )
        };

        if framebuffer_configs.is_null() {
            log_error!("Failed to get framebuffer configurations");
            return Err(ErrorType::Unknown);
        }
        if nb_framebuffer_configs == 0 {
            log_error!("No compatible framebuffer configurations found");
            return Err(ErrorType::DoesNotExist);
        }
        Ok((framebuffer_configs, nb_framebuffer_configs))
    }

    /// Creates the framebuffer configurations
    fn init_framebuffer_configuration(
        config: &Config,
        display: *mut x11::xlib::Display,
        screen_number: i32,
    ) -> Result<(*mut x11::glx::GLXFBConfig, *mut x11::glx::__GLXFBConfigRec), ErrorType> {
        let (framebuffer_configs, nb_framebuffer_configs) =
            match Self::init_framebuffer_configurations(config, display, screen_number) {
                Ok(configs) => configs,
                Err(err) => {
                    log_error!("Failed to init the framebuffer configurations: {:?}", err);
                    return Err(ErrorType::Unknown);
                }
            };

        let framebuffer_configs_slice = unsafe {
            std::slice::from_raw_parts(framebuffer_configs, nb_framebuffer_configs as usize)
        };

        // TODO: select best config instead of first
        log_info!(
            "{:?} framebuffer configurations found",
            nb_framebuffer_configs
        );
        let selected_config = framebuffer_configs_slice[0];

        Ok((framebuffer_configs, selected_config))
    }

    /// Creates the opengl context
    fn init_context(
        config: &Config,
        display: *mut x11::xlib::Display,
        framebuffer_config: *mut x11::glx::__GLXFBConfigRec,
    ) -> Result<(*mut x11::glx::__GLXcontextRec, std::os::raw::c_int), ErrorType> {
        // Get visual id
        let mut visual_id: std::os::raw::c_int = 0;
        let status = unsafe {
            x11::glx::glXGetFBConfigAttrib(
                display,
                framebuffer_config,
                x11::glx::GLX_VISUAL_ID,
                &mut visual_id,
            )
        };
        if status != 0 {
            log_error!("Failed to get the visual id from the framebuffer config");
            return Err(ErrorType::Unknown);
        }

        // Create context
        let share_list = std::ptr::null_mut();
        let direct_connection_to_gpu = true as std::os::raw::c_int;
        let context = unsafe {
            x11::glx::glXCreateNewContext(
                display,
                framebuffer_config,
                Self::get_glx_render_type(
                    &config.renderer_config.opengl_parameters.framebuffer_format,
                ),
                share_list,
                direct_connection_to_gpu,
            )
        };

        Ok((context, visual_id))
    }

    /// Inits the colormap
    fn init_colormap(
        connection: &xcb::Connection,
        screen: &xcb::x::Screen,
        visual_id: std::os::raw::c_int,
    ) -> Result<xcb::x::Colormap, ErrorType> {
        let colormap = connection.generate_id();
        let cookie = connection.send_request_checked(&x::CreateColormap {
            alloc: x::ColormapAlloc::None,
            mid: colormap,
            window: screen.root(),
            visual: visual_id as u32,
        });
        if let Err(err) = connection.check_request(cookie) {
            log_error!("Failed to create a colormap: {:?}", err);
            return Err(ErrorType::Unknown);
        };

        Ok(colormap)
    }

    pub fn init(
        config: &Config,
        display: *mut x11::xlib::Display,
        connection: &xcb::Connection,
        screen: &xcb::x::Screen,
        screen_number: i32,
    ) -> Result<Self, ErrorType> {
        let (all_framebuffer_configs, framebuffer_config) =
            match LinuxX11OpenglWindow::init_framebuffer_configuration(
                config,
                display,
                screen_number,
            ) {
                Ok(config) => config,
                Err(err) => {
                    log_error!(
                        "Failed to init the OpenGL framebuffer config when initializing the X11 linux window: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            };
        let (context, visual_id) = match LinuxX11OpenglWindow::init_context(
            config,
            display,
            framebuffer_config,
        ) {
            Ok(context) => context,
            Err(err) => {
                log_error!(
                    "Failed to init the OpenGL context when initializing the X11 linux window: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };
        let colormap = match LinuxX11OpenglWindow::init_colormap(connection, screen, visual_id) {
            Ok(config) => config,
            Err(err) => {
                log_error!(
                    "Failed to init the OpenGL colormap when initializing the X11 linux window: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };

        Ok(Self {
            display,
            all_framebuffer_configs,
            framebuffer_config,
            context,
            visual_id,
            colormap,
            // The drawable and the window must be created after the xcb window
            drawable: x11::glx::GLXDrawable::default(),
            window: x11::glx::GLXWindow::default(),
        })
    }

    /// Inits the drawable windows
    pub fn init_drawable(&mut self, window: x::Window) -> Result<(), ErrorType> {
        self.window = unsafe {
            x11::glx::glXCreateWindow(
                self.display,
                self.framebuffer_config,
                window.resource_id() as std::os::raw::c_ulong,
                std::ptr::null(),
            )
        };
        if self.window == x11::glx::GLXWindow::default() {
            log_error!("Failed to init the OpenGL GLX window");
            return Err(ErrorType::Unknown);
        }

        self.drawable = self.window;

        Ok(())
    }

    /// Shutds down the opengl window
    pub fn shutdown(&mut self) -> Result<(), ErrorType> {
        // Free the configs
        unsafe {
            x11::xlib::XFree(self.all_framebuffer_configs as *mut _);
        }
        // Shutting down the opengl window
        unsafe { x11::glx::glXDestroyWindow(self.display, self.window) };
        // Shutting down the opengl context
        unsafe { x11::glx::glXDestroyContext(self.display, self.context) };
        Ok(())
    }

    pub fn get_depth(&self) -> Result<u8, ErrorType> {
        let visual_info =
            unsafe { x11::glx::glXGetVisualFromFBConfig(self.display, self.framebuffer_config) };
        if visual_info.is_null() {
            log_error!(
                "Failed to get the visual info when initializing the linux X11 plateform with OpenGL"
            );
            return Err(ErrorType::Unknown);
        }
        let depth = (unsafe { *visual_info }).depth as u8;
        unsafe {
            x11::xlib::XFree(visual_info as *mut _);
        }

        Ok(depth)
    }
}

/// The required elements to manage a window in Linux X11
pub struct LinuxX11Window {
    /// Common window properties
    pub properties: WindowCommonProperties,
    /// The keycode to keysym map
    keymap: std::collections::HashMap<x::Keycode, x::Keysym>,
    /// The atoms needed
    atoms: LinuxX11Atoms,
    /// The xcb connection
    connection: xcb::Connection,
    /// The xcb window
    #[allow(unused)]
    window: x::Window,
    /// The xcb screen properties
    screen: LinuxX11ScreenProperties,

    #[cfg(opengl_renderer)]
    /// OpenGL specific window information
    opengl_window: LinuxX11OpenglWindow,
}

impl Window for LinuxX11Window {
    type WindowType = LinuxX11Window;

    fn init(config: &Config) -> Result<Self::WindowType, ErrorType> {
        // Open Xlib display
        let display = unsafe { x11::xlib::XOpenDisplay(std::ptr::null()) };
        if display.is_null() {
            log_error!("Failed to open an xlib display when initializing the X11 linux window");
            return Err(ErrorType::Unknown);
        }

        // Connect to the X server
        let connection = unsafe { xcb::Connection::from_xlib_display(display) };
        if let Err(err) = connection.has_error() {
            log_error!(
                "Failed to create an xcb connection when initializing the X11 linux window: {:?}",
                err
            );
            unsafe { x11::xlib::XCloseDisplay(display) };
            return Err(ErrorType::Unknown);
        }

        let default_screen_number = match xcb::Connection::connect(None) {
            Ok((_, default_screen_number)) => default_screen_number,
            Err(err) => {
                log_error!(
                    "Failed to create the default screen connection when initializing the X11 linux window: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };

        // Fetch the x::Setup and get the main x::Screen object
        let setup = connection.get_setup();
        let screen = match setup.roots().nth(default_screen_number as usize) {
            Some(screen) => screen,
            None => {
                log_error!("Failed to fetch the screen when initializing the X11 linux window");
                return Err(ErrorType::DoesNotExist);
            }
        };

        // Create OpenGL requirements
        #[cfg(opengl_renderer)]
        let mut opengl_window = match LinuxX11OpenglWindow::init(
            config,
            display,
            &connection,
            screen,
            default_screen_number,
        ) {
            Ok(opengl_window) => opengl_window,
            Err(err) => {
                log_error!(
                    "Failed to initialize the OpenGL window when initializing the X11 linux window: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };

        // Generate an Xid for the client window
        let window: x::Window = connection.generate_id();

        // Create the window
        let x = (config.window_config.position.x * (screen.width_in_pixels() as f32)) as i16;
        let y = (config.window_config.position.y * (screen.height_in_pixels() as f32)) as i16;
        let width = (config.window_config.width * (screen.width_in_pixels() as f32)) as u16;
        let height = (config.window_config.height * (screen.height_in_pixels() as f32)) as u16;

        #[allow(unused)]
        let depth = x::COPY_FROM_PARENT as u8;
        #[cfg(opengl_renderer)]
        let depth = match opengl_window.get_depth() {
            Ok(depth) => depth,
            Err(err) => {
                log_error!(
                    "Failed to get the depth when initializing the X11 linux plateform with OpenGL: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };

        #[allow(unused)]
        let visual = screen.root_visual();
        #[cfg(opengl_renderer)]
        let visual = opengl_window.visual_id as u32;

        let event_mask = x::EventMask::EXPOSURE
            | x::EventMask::KEY_PRESS
            | x::EventMask::KEY_RELEASE
            | x::EventMask::BUTTON_PRESS
            | x::EventMask::BUTTON_RELEASE
            | x::EventMask::POINTER_MOTION
            | x::EventMask::BUTTON_MOTION
            | x::EventMask::ENTER_WINDOW
            | x::EventMask::LEAVE_WINDOW
            | x::EventMask::STRUCTURE_NOTIFY
            | x::EventMask::FOCUS_CHANGE;

        // Warning, the list must be sorted in the same order as in
        // https://docs.rs/xcb/1.5.0/xcb/x/enum.Cw.html
        #[allow(unused_mut)]
        let mut value_list = vec![
            x::Cw::BackPixel(screen.black_pixel()),
            x::Cw::EventMask(event_mask),
        ];

        #[cfg(opengl_renderer)]
        value_list.push(x::Cw::Colormap(opengl_window.colormap));

        let cookie = connection.send_request_checked(&x::CreateWindow {
            depth,
            wid: window,
            parent: screen.root(),
            x,
            y,
            width,
            height,
            border_width: config.window_config.border_width,
            class: x::WindowClass::InputOutput,
            visual,
            value_list: &value_list,
        });

        // Check if the window creation worked
        if let Err(err) = connection.check_request(cookie) {
            log_error!(
                "Failed to create a window when initializing the X11 linux window: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }

        // Update window's title
        let cookie = connection.send_request_checked(&x::ChangeProperty {
            mode: x::PropMode::Replace,
            window,
            property: x::ATOM_WM_NAME,
            r#type: x::ATOM_STRING,
            data: config.window_config.title.as_bytes(),
        });
        if let Err(err) = connection.check_request(cookie) {
            log_error!(
                "Failed to update the window title when initializing the X11 linux window: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        };

        // Map the window
        connection.send_request(&x::MapWindow { window });

        // Get necessary atoms
        // An atom is an id replacement for a string
        let (wm_protocols, wm_del_window, wm_state, wm_max_vert, wm_max_horz, wm_hidden) = {
            let cookies = (
                connection.send_request(&x::InternAtom {
                    only_if_exists: true,
                    name: b"WM_PROTOCOLS",
                }),
                connection.send_request(&x::InternAtom {
                    only_if_exists: true,
                    name: b"WM_DELETE_WINDOW",
                }),
                connection.send_request(&x::InternAtom {
                    only_if_exists: true,
                    name: b"_NET_WM_STATE",
                }),
                connection.send_request(&x::InternAtom {
                    only_if_exists: true,
                    name: b"_NET_WM_STATE_MAXIMIZED_VERT",
                }),
                connection.send_request(&x::InternAtom {
                    only_if_exists: true,
                    name: b"_NET_WM_STATE_MINIMIZED_VERT",
                }),
                connection.send_request(&x::InternAtom {
                    only_if_exists: true,
                    name: b"_NET_WM_STATE_HIDDEN",
                }),
            );
            (
                match connection.wait_for_reply(cookies.0) {
                    Ok(reply) => reply.atom(),
                    Err(err) => {
                        log_error!(
                            "Failed to fetch back an atom when initializing the X11 linux window: {:?}",
                            err
                        );
                        return Err(ErrorType::Unknown);
                    }
                },
                match connection.wait_for_reply(cookies.1) {
                    Ok(reply) => reply.atom(),
                    Err(err) => {
                        log_error!(
                            "Failed to fetch back an atom when initializing the X11 linux window: {:?}",
                            err
                        );
                        return Err(ErrorType::Unknown);
                    }
                },
                match connection.wait_for_reply(cookies.2) {
                    Ok(reply) => reply.atom(),
                    Err(err) => {
                        log_error!(
                            "Failed to fetch back an atom when initializing the X11 linux window: {:?}",
                            err
                        );
                        return Err(ErrorType::Unknown);
                    }
                },
                match connection.wait_for_reply(cookies.3) {
                    Ok(reply) => reply.atom(),
                    Err(err) => {
                        log_error!(
                            "Failed to fetch back an atom when initializing the X11 linux window: {:?}",
                            err
                        );
                        return Err(ErrorType::Unknown);
                    }
                },
                match connection.wait_for_reply(cookies.4) {
                    Ok(reply) => reply.atom(),
                    Err(err) => {
                        log_error!(
                            "Failed to fetch back an atom when initializing the X11 linux window: {:?}",
                            err
                        );
                        return Err(ErrorType::Unknown);
                    }
                },
                match connection.wait_for_reply(cookies.5) {
                    Ok(reply) => reply.atom(),
                    Err(err) => {
                        log_error!(
                            "Failed to fetch back an atom when initializing the X11 linux window: {:?}",
                            err
                        );
                        return Err(ErrorType::Unknown);
                    }
                },
            )
        };

        // Activate the window close event
        if let Err(err) =
            connection.check_request(connection.send_request_checked(&x::ChangeProperty {
                mode: x::PropMode::Replace,
                window,
                property: wm_protocols,
                r#type: x::ATOM_ATOM,
                data: &[wm_del_window],
            }))
        {
            log_error!(
                "Failed to activate the window close event when initializing the X11 linux window: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        };

        // Cache the keymap
        let min_keycode = setup.min_keycode();
        let max_keycode = setup.max_keycode();
        let cookie = connection.send_request(&x::GetKeyboardMapping {
            first_keycode: min_keycode,
            count: (max_keycode - min_keycode + 1u8),
        });
        let keymap_reply = match connection.wait_for_reply(cookie) {
            Ok(reply) => reply,
            Err(err) => {
                log_error!(
                    "Failed to fetch the keymap when initializing the X11 linux window: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };
        let mut keymap = HashMap::new();
        let keysyms = keymap_reply.keysyms();
        let keysyms_per_keycode = keymap_reply.keysyms_per_keycode() as usize;
        for (i, chunk) in keysyms.chunks(keysyms_per_keycode).enumerate() {
            let keycode = min_keycode + (i as x::Keycode);
            if let Some(&keysym) = chunk.first() {
                keymap.insert(keycode, keysym);
            }
        }

        let properties = WindowCommonProperties {
            position: config.window_config.position,
            width: config.window_config.width,
            height: config.window_config.height,
            display_mode: DisplayMode::Floating(None),
        };

        let atoms = LinuxX11Atoms {
            protocols: wm_protocols,
            delete_window: wm_del_window,
            state: wm_state,
            state_maximized_vert: wm_max_vert,
            state_maximized_horz: wm_max_horz,
            state_hidden: wm_hidden,
        };

        let screen = LinuxX11ScreenProperties {
            width: screen.width_in_pixels(),
            height: screen.height_in_pixels(),
        };

        // Inits the opengl drawable windows
        #[cfg(opengl_renderer)]
        if let Err(err) = opengl_window.init_drawable(window) {
            log_error!(
                "Failed to initialize the OpenGL window when initializing the X11 linux window: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        };

        Ok(LinuxX11Window {
            properties,
            keymap,
            atoms,
            connection,
            window,
            screen,
            #[cfg(opengl_renderer)]
            opengl_window,
        })
    }

    fn shutdown(&mut self) -> Result<(), ErrorType> {
        #[cfg(opengl_renderer)]
        if let Err(err) = self.opengl_window.shutdown() {
            log_error!(
                "Failed to shut down the opengl window when shutting down the X11 linux window: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }
        Ok(())
    }

    fn get_properties(&self) -> WindowCommonProperties {
        self.properties
    }

    fn poll_event(&mut self) -> Result<Event, ErrorType> {
        match self.connection.wait_for_event() {
            Err(err) => {
                log_error!("Failed to wait for an event on the X11 linux: {:?}", err);
                Err(ErrorType::Unknown)
            }
            Ok(event) => match event {
                // Keyboard events
                xcb::Event::X(x::Event::KeyPress(event)) => {
                    match self.get_key_from_keysym(event.detail()) {
                        Some(key) => Ok(Event::KeyboardKeyPressed(key)),
                        None => Ok(Event::KeyboardKeyPressed(KeyboardKey::Unrecognized)),
                    }
                }
                xcb::Event::X(x::Event::KeyRelease(event)) => {
                    match self.get_key_from_keysym(event.detail()) {
                        Some(key) => Ok(Event::KeyboardKeyReleased(key)),
                        None => Ok(Event::KeyboardKeyReleased(KeyboardKey::Unrecognized)),
                    }
                }
                // Client message events
                xcb::Event::X(x::Event::ClientMessage(event)) => {
                    if event.r#type() == self.atoms.state {
                        if let x::ClientMessageData::Data32(
                            [_, first_property, second_property, ..],
                        ) = event.data()
                        {
                            // Window maximized
                            if first_property == self.atoms.state_maximized_horz.resource_id()
                                || first_property == self.atoms.state_maximized_vert.resource_id()
                                || second_property == self.atoms.state_maximized_horz.resource_id()
                                || second_property == self.atoms.state_maximized_vert.resource_id()
                            {
                                return Ok(Event::WindowRestored);
                            }
                            // Window minimized
                            else if first_property == self.atoms.state_hidden.resource_id() {
                                return Ok(Event::WindowMinimized);
                            }
                        }
                    } else if event.r#type() == self.atoms.protocols
                        && let x::ClientMessageData::Data32([atom, ..]) = event.data()
                    {
                        // Window closed
                        if atom == self.atoms.delete_window.resource_id() {
                            return Ok(Event::WindowClosed);
                        }
                    }
                    log_debug!("Unknown X11 linux client message event");
                    Ok(Event::Unrecognized)
                }
                // Configure notify events
                xcb::Event::X(x::Event::ConfigureNotify(event)) => {
                    let width = (event.width() as f32) / (self.screen.width as f32);
                    let height = (event.height() as f32) / (self.screen.height as f32);

                    // Detect if the size has changed and trigger the corresponding event
                    if width != self.properties.width || height != self.properties.height {
                        // Update the window's properties
                        self.properties.width = width;
                        self.properties.height = height;

                        return Ok(Event::WindowResized(width, height));
                    }
                    Ok(Event::Unrecognized)
                }
                xcb::Event::X(x::Event::Expose(_)) => Ok(Event::Expose),

                // TODO: other events
                _ => {
                    // log_debug!("Unknown X11 linux window event");
                    Ok(Event::Unrecognized)
                }
            },
        }
    }

    #[cfg(opengl_renderer)]
    fn opengl_swap_buffers(&mut self) -> Result<(), ErrorType> {
        unsafe {
            x11::glx::glXSwapBuffers(self.opengl_window.display, self.opengl_window.drawable)
        };
        Ok(())
    }

    #[cfg(opengl_renderer)]
    fn opengl_make_context_current(&mut self) -> Result<(), ErrorType> {
        if unsafe {
            x11::glx::glXMakeContextCurrent(
                self.opengl_window.display,
                self.opengl_window.drawable,
                self.opengl_window.drawable,
                self.opengl_window.context,
            )
        } == 0
        {
            log_error!("Failed to make the OpenGL context current on the Linux X11 plateform");
            return Err(ErrorType::Unknown);
        };

        Ok(())
    }

    #[cfg(opengl_renderer)]
    fn opengl_load_functions(&mut self) -> Result<(), ErrorType> {
        gl::load_with(|name| {
            let cname = std::ffi::CString::new(name).unwrap(); // should never fail for valid GL names
            unsafe {
                x11::glx::glXGetProcAddress(cname.as_ptr() as *const std::os::raw::c_uchar).unwrap()
                    as *const std::os::raw::c_void
            }
        });

        // Check if OpenGL functions are loaded
        if !gl::Clear::is_loaded() {
            log_error!("Failed to load the OpenGL `Clear' function");
            return Err(ErrorType::Unknown);
        }
        if !gl::ClearColor::is_loaded() {
            log_error!("Failed to load the OpenGL `Clear' function");
            return Err(ErrorType::Unknown);
        }
        // TODO: add other used functions

        log_info!("All OpenGL functions loaded");

        Ok(())
    }
}

impl LinuxX11Window {
    fn get_key_from_keysym(&self, keycode: x::Keycode) -> Option<KeyboardKey> {
        let keysym = self.keymap.get(&keycode)?;

        match keysym {
            // Alphabet keys
            0x0061 => Some(KeyboardKey::AlphaNumeric(crate::keyboard::AlphaNumeric::A)),
            0x0062 => Some(KeyboardKey::AlphaNumeric(crate::keyboard::AlphaNumeric::B)),
            0x0063 => Some(KeyboardKey::AlphaNumeric(crate::keyboard::AlphaNumeric::C)),
            0x0064 => Some(KeyboardKey::AlphaNumeric(crate::keyboard::AlphaNumeric::D)),
            0x0065 => Some(KeyboardKey::AlphaNumeric(crate::keyboard::AlphaNumeric::E)),
            0x0066 => Some(KeyboardKey::AlphaNumeric(crate::keyboard::AlphaNumeric::F)),
            0x0067 => Some(KeyboardKey::AlphaNumeric(crate::keyboard::AlphaNumeric::G)),
            0x0068 => Some(KeyboardKey::AlphaNumeric(crate::keyboard::AlphaNumeric::H)),
            0x0069 => Some(KeyboardKey::AlphaNumeric(crate::keyboard::AlphaNumeric::I)),
            0x006A => Some(KeyboardKey::AlphaNumeric(crate::keyboard::AlphaNumeric::J)),
            0x006B => Some(KeyboardKey::AlphaNumeric(crate::keyboard::AlphaNumeric::K)),
            0x006C => Some(KeyboardKey::AlphaNumeric(crate::keyboard::AlphaNumeric::L)),
            0x006D => Some(KeyboardKey::AlphaNumeric(crate::keyboard::AlphaNumeric::M)),
            0x006E => Some(KeyboardKey::AlphaNumeric(crate::keyboard::AlphaNumeric::N)),
            0x006F => Some(KeyboardKey::AlphaNumeric(crate::keyboard::AlphaNumeric::O)),
            0x0070 => Some(KeyboardKey::AlphaNumeric(crate::keyboard::AlphaNumeric::P)),
            0x0071 => Some(KeyboardKey::AlphaNumeric(crate::keyboard::AlphaNumeric::Q)),
            0x0072 => Some(KeyboardKey::AlphaNumeric(crate::keyboard::AlphaNumeric::R)),
            0x0073 => Some(KeyboardKey::AlphaNumeric(crate::keyboard::AlphaNumeric::S)),
            0x0074 => Some(KeyboardKey::AlphaNumeric(crate::keyboard::AlphaNumeric::T)),
            0x0075 => Some(KeyboardKey::AlphaNumeric(crate::keyboard::AlphaNumeric::U)),
            0x0076 => Some(KeyboardKey::AlphaNumeric(crate::keyboard::AlphaNumeric::V)),
            0x0077 => Some(KeyboardKey::AlphaNumeric(crate::keyboard::AlphaNumeric::W)),
            0x0078 => Some(KeyboardKey::AlphaNumeric(crate::keyboard::AlphaNumeric::X)),
            0x0079 => Some(KeyboardKey::AlphaNumeric(crate::keyboard::AlphaNumeric::Y)),
            0x007A => Some(KeyboardKey::AlphaNumeric(crate::keyboard::AlphaNumeric::Z)),

            // Digit keys (0-9)
            0x0030 => Some(KeyboardKey::AlphaNumeric(
                crate::keyboard::AlphaNumeric::Zero,
            )),
            0x0031 => Some(KeyboardKey::AlphaNumeric(
                crate::keyboard::AlphaNumeric::One,
            )),
            0x0032 => Some(KeyboardKey::AlphaNumeric(
                crate::keyboard::AlphaNumeric::Two,
            )),
            0x0033 => Some(KeyboardKey::AlphaNumeric(
                crate::keyboard::AlphaNumeric::Three,
            )),
            0x0034 => Some(KeyboardKey::AlphaNumeric(
                crate::keyboard::AlphaNumeric::Four,
            )),
            0x0035 => Some(KeyboardKey::AlphaNumeric(
                crate::keyboard::AlphaNumeric::Five,
            )),
            0x0036 => Some(KeyboardKey::AlphaNumeric(
                crate::keyboard::AlphaNumeric::Six,
            )),
            0x0037 => Some(KeyboardKey::AlphaNumeric(
                crate::keyboard::AlphaNumeric::Seven,
            )),
            0x0038 => Some(KeyboardKey::AlphaNumeric(
                crate::keyboard::AlphaNumeric::Eight,
            )),
            0x0039 => Some(KeyboardKey::AlphaNumeric(
                crate::keyboard::AlphaNumeric::Nine,
            )),

            // Arrows
            0xFF51 => Some(KeyboardKey::Arrow(crate::keyboard::Arrow::Left)), // Left arrow key
            0xFF53 => Some(KeyboardKey::Arrow(crate::keyboard::Arrow::Right)), // Right arrow key
            0xFF52 => Some(KeyboardKey::Arrow(crate::keyboard::Arrow::Up)),   // Up arrow key
            0xFF54 => Some(KeyboardKey::Arrow(crate::keyboard::Arrow::Down)), // Down arrow key

            // Modifiers
            0xFFE1 => Some(KeyboardKey::Modifier(crate::keyboard::Modifier::ShiftLeft)), // Shift key
            0xFFE9 => Some(KeyboardKey::Modifier(crate::keyboard::Modifier::AltLeft)), // Left Alt key
            0xFFE3 => Some(KeyboardKey::Modifier(
                crate::keyboard::Modifier::ControlLeft,
            )), // Left Control key

            // Special keys
            0xFF0D => Some(KeyboardKey::Special(crate::keyboard::Special::Enter)), // Enter key
            0xFF08 => Some(KeyboardKey::Special(crate::keyboard::Special::Backspace)), // Backspace key
            0xFFFF => Some(KeyboardKey::Special(crate::keyboard::Special::Delete)),    // Delete key
            0x0020 => Some(KeyboardKey::Special(crate::keyboard::Special::Spacebar)),  // Space key
            0xFF09 => Some(KeyboardKey::Special(crate::keyboard::Special::Tab)),       // Tab key
            0xFF1B => Some(KeyboardKey::Special(crate::keyboard::Special::Escape)),    // Escape key

            // If no match, return Unrecognized
            _ => Some(KeyboardKey::Unrecognized),
        }
    }
}
