use crate::core_layer::{gamepad::GamepadButton, keyboard::KeyboardKey, mouse::MouseButton};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    /// Default unrecognized event
    Unrecognized,

    // Keyboard related events
    /// Event triggered on key press
    KeyboardKeyPressed(KeyboardKey),
    /// Event triggered on key release
    KeyboardKeyReleased(KeyboardKey),

    // Mouse related events
    /// Event triggered on button press
    MouseButtonPressed(MouseButton),
    /// Event triggered on button release
    MouseButtonReleased(MouseButton),
    /// Event triggered when the mouse wheel is moved
    /// MouseScroll(delta)
    /// The delta can be positive or negative
    /// The delta is the amount in pixels to scroll
    MouseScrolled(f32),
    /// Event triggered on mouse movement
    /// MouseMove(new_x, new_y)
    /// The position new_x and new_y are in pixel coordinates
    /// MouseMove(0,0) corresponds to the bottom left of the window
    /// MouseMove(width, height) corresponds to the top right of the window
    MouseMoved(u16, u16),
    /// Event triggered on mouse movement when a button is pressed
    MouseMovedAndButton(u16, u16, MouseButton),
    /// Event triggered when the mouse enters the window
    /// MouseEnteredWindow(x, y)
    /// The position x and y correspond to the position at which the mouse entered the window
    MouseEnteredWindow(u16, u16),
    /// Event triggered when the mouse leaves the window
    /// The position x and y correspond to the position at which the mouse left the window
    MouseLeftWindow(u16, u16),

    // Gamepad related events
    /// Event triggered on button press
    GamepadButtonPressed(GamepadButton),
    /// Event triggered on button release
    GamepadButtonReleased(GamepadButton),

    // Window related events
    /// Event triggered when the window is resized
    /// WindowResize(new_width, new_height)
    /// new_width and new_height are given as values between 0 and 1
    /// 1 being the display size
    WindowResized(f32, f32),
    /// Event triggered when the window is closed
    WindowClosed,
    /// Event triggered when the window is mimized
    WindowMinimized,    
    /// Event triggered when the window is restored (opposite of WindowMinimized)
    WindowRestored,
    /// Event triggered when the focus enters the window
    WindowFocused, 
    /// Event triggered when the focus leaves the window
    WindowUnfocused, 
}
