/// The state of a keyboard key
pub enum KeyboardKeyState {
    /// State when the key is currently being pressed
    Pressed,
    /// State when the key is not currently being pressed
    Released
}

/// A structure representing a keyboard
pub struct Keyboard {
    /// The current key states
    pub current_key_states: std::collections::HashMap<KeyboardKey, KeyboardKeyState>,
    /// The last key states
    /// The states are being swapped each frame
    pub last_key_states: std::collections::HashMap<KeyboardKey, KeyboardKeyState>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
/// The different keyboard keys
pub enum KeyboardKey {
    // Default key mapping to non recognized keys
    Unrecognized,

    A,B,C,D,E,
    F,G,H,I,J,
    K,L,M,N,O,
    P,Q,R,S,T,
    U,V,W,X,Y,
    Z,

    Shift, Enter, Del,
    Space, Tab, LeftCtrl,
    Left, Right, Up, Down,
    Alt, BackSpace,

    Zero, One, Two, Three,
    Four, Five, Six, Seven, 
    Eight, Nine, Escape,
}
