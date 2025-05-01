use std::collections::HashMap;

use crate::{
    config::Config, error::ErrorType, keyboard::KeyboardKey, log_debug, log_error, platform_layer::{window::WindowCommonProperties, DisplayMode, Event, Window}
};

use xcb::{x, Xid};

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
    window: x::Window,
    /// The xcb screen properties
    screen: LinuxX11ScreenProperties,
}

impl Window for LinuxX11Window {
    type WindowType = LinuxX11Window;

    fn init(config: &Config) -> Result<Self::WindowType, ErrorType> {
        // Conenct to the X server
        let (connection, screen_number) = match xcb::Connection::connect(None){
            Ok((connection, screen_number)) => (connection, screen_number),
            Err(err) => {
                log_error!("Failed to create an xcb connection when initializing the X11 linux window: {:?}", err);
                return Err(ErrorType::Unknown);
            },
        };

        // Fetch the x::Setup and get the main x::Screen object
        let setup = connection.get_setup();
        let screen = match setup.roots().nth(screen_number as usize){
            Some(screen) => screen,
            None => {
                log_error!("Failed to fetch the screen when initializing the X11 linux window");
                return Err(ErrorType::DoesNotExist);
            },
        };

        // Generate an Xid for the client window
        let window: x::Window = connection.generate_id();

        // Create the window
        let x = (config.window_config.position.x * (screen.width_in_pixels() as f32)) as i16;
        let y = (config.window_config.position.y * (screen.height_in_pixels() as f32)) as i16;
        let width = (config.window_config.width * (screen.width_in_pixels() as f32)) as u16;
        let height = (config.window_config.height * (screen.height_in_pixels() as f32)) as u16;
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
            | x::EventMask::FOCUS_CHANGE
        ;
        let cookie = connection.send_request_checked(&x::CreateWindow {
            depth: x::COPY_FROM_PARENT as u8,
            wid: window,
            parent: screen.root(),
            x,
            y,
            width,
            height,
            border_width: 0,
            class: x::WindowClass::InputOutput,
            visual: screen.root_visual(),
            value_list: &[
                x::Cw::BackPixel(screen.black_pixel()),
                x::Cw::EventMask(event_mask)
            ]
        });

        // Check if the window creation worked
        if let Err(err) = connection.check_request(cookie){
            log_error!("Failed to create a window when initializing the X11 linux window: {:?}", err);
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
        if let Err(err) = connection.check_request(cookie){
            log_error!("Failed to update the window title when initializing the X11 linux window: {:?}", err);
            return Err(ErrorType::Unknown);
        };

        // Map the window
        connection.send_request(&x::MapWindow{
            window,
        });

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
                        log_error!("Failed to fetch back an atom when initializing the X11 linux window: {:?}", err);
                        return Err(ErrorType::Unknown);
                    },
                },
                match connection.wait_for_reply(cookies.1) {
                    Ok(reply) => reply.atom(),
                    Err(err) => {
                        log_error!("Failed to fetch back an atom when initializing the X11 linux window: {:?}", err);
                        return Err(ErrorType::Unknown);
                    },
                },
                match connection.wait_for_reply(cookies.2) {
                    Ok(reply) => reply.atom(),
                    Err(err) => {
                        log_error!("Failed to fetch back an atom when initializing the X11 linux window: {:?}", err);
                        return Err(ErrorType::Unknown);
                    },
                },
                match connection.wait_for_reply(cookies.3) {
                    Ok(reply) => reply.atom(),
                    Err(err) => {
                        log_error!("Failed to fetch back an atom when initializing the X11 linux window: {:?}", err);
                        return Err(ErrorType::Unknown);
                    },
                },
                match connection.wait_for_reply(cookies.4) {
                    Ok(reply) => reply.atom(),
                    Err(err) => {
                        log_error!("Failed to fetch back an atom when initializing the X11 linux window: {:?}", err);
                        return Err(ErrorType::Unknown);
                    },
                },
                match connection.wait_for_reply(cookies.5) {
                    Ok(reply) => reply.atom(),
                    Err(err) => {
                        log_error!("Failed to fetch back an atom when initializing the X11 linux window: {:?}", err);
                        return Err(ErrorType::Unknown);
                    },
                },
            )
        };

        // Activate the window close event
        if let Err(err) = connection.check_request(
            connection.send_request_checked(&x::ChangeProperty {
                mode: x::PropMode::Replace,
                window,
                property: wm_protocols,
                r#type: x::ATOM_ATOM,
                data: &[wm_del_window],
            })){
                log_error!("Failed to activate the window close event when initializing the X11 linux window: {:?}", err);
                return Err(ErrorType::Unknown);
            }
        ;

        // Cache the keymap
        let min_keycode = setup.min_keycode();
        let max_keycode = setup.max_keycode();
        let cookie = connection.send_request(&x::GetKeyboardMapping {
            first_keycode: min_keycode,
            count: (max_keycode - min_keycode +1).into(),
        });
        let keymap_reply = match connection.wait_for_reply(cookie){
            Ok(reply) => reply,
            Err(err) => {
                log_error!("Failed to fetch the keymap when initializing the X11 linux window: {:?}", err);
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

        Ok(LinuxX11Window {
            properties,
            keymap,
            atoms,
            connection,
            window,
            screen,
        })
    }

    fn shutdown(&mut self) -> Result<(), ErrorType> {
        // TODO: Implement Linux X11 specific code
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
            },
            Ok(event) => match event {
                // Keyboard events
                xcb::Event::X(x::Event::KeyPress(event)) => {
                    match self.get_key_from_keysym(event.detail()){
                        Some(key) => Ok(Event::KeyboardKeyPressed(key)),
                        None => {
                            Ok(Event::KeyboardKeyPressed(KeyboardKey::Unrecognized))
                        },
                    }
                },
                xcb::Event::X(x::Event::KeyRelease(event)) => {
                    match self.get_key_from_keysym(event.detail()){
                        Some(key) => Ok(Event::KeyboardKeyReleased(key)),
                        None => {
                            Ok(Event::KeyboardKeyReleased(KeyboardKey::Unrecognized))
                        },
                    }
                },
                // Client message events
                xcb::Event::X(x::Event::ClientMessage(event)) => {
                    if event.r#type() == self.atoms.state {
                        if let x::ClientMessageData::Data32([_, first_property, second_property, ..]) = event.data() {
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
                    } else if event.r#type() == self.atoms.protocols {
                        if let x::ClientMessageData::Data32([atom, ..]) = event.data() {
                            // Window closed
                            if atom == self.atoms.delete_window.resource_id() {
                                return Ok(Event::WindowClosed);
                            }
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
                },
                // TODO: other events
                _ => {
                    log_debug!("Unknown X11 linux window event");
                    Ok(Event::Unrecognized)
                }
            }
        }
    }
}


impl LinuxX11Window {
    fn get_key_from_keysym(&self, keycode: x::Keycode) -> Option<KeyboardKey> {
        let keysym = match self.keymap.get(&keycode) {
            Some(keysym) => keysym,
            None => return None,
        };
        match keysym {
            // Alphabet keys
            0x0061 => Some(KeyboardKey::A),
            0x0062 => Some(KeyboardKey::B),
            0x0063 => Some(KeyboardKey::C),
            0x0064 => Some(KeyboardKey::D),
            0x0065 => Some(KeyboardKey::E),
            0x0066 => Some(KeyboardKey::F),
            0x0067 => Some(KeyboardKey::G),
            0x0068 => Some(KeyboardKey::H),
            0x0069 => Some(KeyboardKey::I),
            0x006a => Some(KeyboardKey::J),
            0x006b => Some(KeyboardKey::K),
            0x006c => Some(KeyboardKey::L),
            0x006d => Some(KeyboardKey::M),
            0x006e => Some(KeyboardKey::N),
            0x006f => Some(KeyboardKey::O),
            0x0070 => Some(KeyboardKey::P),
            0x0071 => Some(KeyboardKey::Q),
            0x0072 => Some(KeyboardKey::R),
            0x0073 => Some(KeyboardKey::S),
            0x0074 => Some(KeyboardKey::T),
            0x0075 => Some(KeyboardKey::U),
            0x0076 => Some(KeyboardKey::V),
            0x0077 => Some(KeyboardKey::W),
            0x0078 => Some(KeyboardKey::X),
            0x0079 => Some(KeyboardKey::Y),
            0x007a => Some(KeyboardKey::Z),

            // Digit keys (0-9)
            0x0030 => Some(KeyboardKey::Zero),
            0x0031 => Some(KeyboardKey::One),
            0x0032 => Some(KeyboardKey::Two),
            0x0033 => Some(KeyboardKey::Three),
            0x0034 => Some(KeyboardKey::Four),
            0x0035 => Some(KeyboardKey::Five),
            0x0036 => Some(KeyboardKey::Six),
            0x0037 => Some(KeyboardKey::Seven),
            0x0038 => Some(KeyboardKey::Eight),
            0x0039 => Some(KeyboardKey::Nine),

            // Special keys
            0xFFE1 => Some(KeyboardKey::Shift),   // Shift key
            0xFF0D => Some(KeyboardKey::Enter),   // Enter key
            0xFF08 => Some(KeyboardKey::BackSpace), // Backspace key
            0xFFFF => Some(KeyboardKey::Del),     // Delete key
            0x0020 => Some(KeyboardKey::Space),   // Space key
            0xFF09 => Some(KeyboardKey::Tab),     // Tab key
            0xFF51 => Some(KeyboardKey::Left),    // Left arrow key
            0xFF53 => Some(KeyboardKey::Right),   // Right arrow key
            0xFF52 => Some(KeyboardKey::Up),      // Up arrow key
            0xFF54 => Some(KeyboardKey::Down),    // Down arrow key
            0xFFE9 => Some(KeyboardKey::Alt),     // Left Alt key
            0xFFE3 => Some(KeyboardKey::LeftCtrl), // Left Control key
            0xFF1B => Some(KeyboardKey::Escape),  // Escape key

            // If no match, return Unrecognized
            _ => Some(KeyboardKey::Unrecognized),
        }
    }
}