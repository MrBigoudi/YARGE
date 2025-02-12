use x11rb::{connection::Connection, protocol::xproto::{ConnectionExt, Screen, WindowClass}, rust_connection::RustConnection, COPY_DEPTH_FROM_PARENT};

use crate::{
    config::{Config, WindowConfig}, error::ErrorType, log_debug, log_error, maths::{vec2, Vector2}, platform_layer::{DisplayMode, Window}
};

pub struct LinuxX11Window {
    pub connection: RustConnection,
    pub screen_number: usize,
    pub window_id: u32,

    /// The window's configuration
    pub config: WindowConfig,
}

impl LinuxX11Window {
    /// Creates a window using the configuration
    fn create_window(config: &Config, connection: &RustConnection, window_id: u32, screen: &Screen) -> Result<(), ErrorType> {
        let screen_size = vec2(screen.width_in_pixels as f32, screen.height_in_pixels as f32);
        let width = (config.window_config.width * screen_size.x) as u16;
        let height = (config.window_config.height * screen_size.y) as u16;
        let position = config.window_config.position * screen_size;
        let x = position.x as i16;
        let y = position.y as i16;
        let border_width = config.window_config.border_width;

        if let Err(err) = connection.create_window(
            COPY_DEPTH_FROM_PARENT,    // depth of the screen (same as root)
            window_id,                 // window Id
            screen.root,               // Id of an existing window that should be the parent of the new window
            x,                         // x position of the top-left corner of the window in pixels
            y,                         // y position of the top-left corner of the window in pixels
            width,                       // width of the window in pixels
            height,                       // height of the window in pixels
            border_width,                        // border width in pixels
            WindowClass::INPUT_OUTPUT, // class
            screen.root_visual,        // visual
            &Default::default(),       // auxiliary and optional information 
        ){
            log_error!("Failed to create a xcb window: {:?}", err);
            return Err(ErrorType::PlatformDependentFailure);
        }

        Ok(())
    }
}

impl Window for LinuxX11Window {
    type WindowType = LinuxX11Window;

    /// Initiate a Linux11Window using XCB protocol
    /// https://www.x.org/releases/X11R7.7/doc/libxcb/tutorial/index.html
    /// https://docs.rs/x11rb/latest/x11rb/index.html
    fn init(config: &Config) -> Result<Self::WindowType, ErrorType> {
        // TODO: Implement Linux X11 specific code

        // Open the connection to the X server
        // dpy_name is None to use the $DISPLAY environment variable
        let (connection, screen_number) = match x11rb::connect(None) {
            Err(err) => {
                log_error!("Failed to establish a xcb connection: {:?}", err);
                return Err(ErrorType::PlatformDependentFailure);
            },
            Ok(connection) => connection,
        };

        // Get the screen
        let screen = &connection.setup().roots[screen_number];
        #[cfg(debug_assertions)]
        log_debug!("Information about new screen: {}", super::helper::xcb_screen_to_string(screen));

        // Ask for a new window id
        let window_id = match connection.generate_id(){
            Err(err) => {
                log_error!("Failed to generate a xcb window id: {:?}", err);
                return Err(ErrorType::PlatformDependentFailure);
            },
            Ok(id) => id
        };

        // Create the window
        // The window is masked when first created
        LinuxX11Window::create_window(config, &connection, window_id, screen)?;

        // Map the window on the screen
        if let Err(err) = connection.map_window(window_id) {
            log_error!("Failed to map the xcb window to the screen: {:?}", err);
            return Err(ErrorType::PlatformDependentFailure);
        }

        // TODO: remove these tests
        connection.flush().unwrap();
        std::thread::sleep(std::time::Duration::from_secs(5));

        Ok(LinuxX11Window {
            connection,
            screen_number,
            window_id,
            config: config.window_config.clone(),
        })
    }

    fn shutdown(&mut self) -> Result<(), ErrorType> {
        // TODO: Implement Linux X11 specific code
        Ok(())
    }

    fn get_width(&self) -> f32 {
        // TODO: Implement Linux X11 specific code
        todo!()
    }

    fn get_height(&self) -> f32 {
        // TODO: Implement Linux X11 specific code
        todo!()
    }

    fn get_position(&self) -> Vector2 {
        // TODO: Implement Linux X11 specific code
        todo!()
    }

    fn get_dpi_factor(&self) -> f32 {
        // TODO: Implement Linux X11 specific code
        todo!()
    }

    fn get_id(&self) -> u8 {
        // TODO: Implement Linux X11 specific code
        todo!()
    }

    fn set_display_mode(&mut self, _mode: DisplayMode) -> Result<(), ErrorType> {
        // TODO: Implement Linux X11 specific code
        todo!()
    }

    fn set_position(&mut self, _x: f32, _y: f32) -> Result<(), ErrorType> {
        // TODO: Implement Linux X11 specific code
        todo!()
    }
}
