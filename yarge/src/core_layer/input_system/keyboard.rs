//! Contains the implementation for a Keyboard device
//!
use std::collections::HashMap;

/// The state of a key
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardKeyState {
    /// When a key is being pressed
    Pressed,
    /// When a key is being released
    #[default]
    Released,
}

/// The letter and digit keyboard keys
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlphaNumeric {
    /// The 'A' key
    A,
    /// The 'B' key
    B,
    /// The 'C' key
    C,
    /// The 'D' key
    D,
    /// The 'E' key
    E,
    /// The 'F' key
    F,
    /// The 'G' key
    G,
    /// The 'H' key
    H,
    /// The 'I' key
    I,
    /// The 'J' key
    J,
    /// The 'K' key
    K,
    /// The 'L' key
    L,
    /// The 'M' key
    M,
    /// The 'N' key
    N,
    /// The 'O' key
    O,
    /// The 'P' key
    P,
    /// The 'Q' key
    Q,
    /// The 'R' key
    R,
    /// The 'S' key
    S,
    /// The 'T' key
    T,
    /// The 'U' key
    U,
    /// The 'V' key
    V,
    /// The 'W' key
    W,
    /// The 'X' key
    X,
    /// The 'Y' key
    Y,
    /// The 'Z' key
    Z,

    /// The '0' key
    Zero,
    /// The '1' key
    One,
    /// The '2' key
    Two,
    /// The '3' key
    Three,
    /// The '4' key
    Four,
    /// The '5' key
    Five,
    /// The '6' key
    Six,
    /// The '7' key
    Seven,
    /// The '8' key
    Eight,
    /// The '9' key
    Nine,
}

/// The arrow keyboard keys
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arrow {
    /// The 'Up' key
    Up,
    /// The 'Down' key
    Down,
    /// The 'Left' key
    Left,
    /// The 'Right' key
    Right,
}

/// The modifier keyboard keys
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Modifier {
    /// The left 'Shift' key
    ShiftLeft,
    /// The right 'Shift' key
    ShiftRight,
    /// The left 'Ctrl' key
    ControlLeft,
    /// The right 'Ctrl' key
    ControlRight,
    /// The left 'Alt' key
    AltLeft,
    /// The right 'AltGr' key
    AltRight,
}

/// Other special keyboard keys
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Special {
    /// The 'Escape' key
    Escape,
    /// The 'Enter' key
    Enter,
    /// The 'Backspace' key
    Backspace,
    /// The 'Tab' key
    Tab,
    /// The 'Del' key
    Delete,
    /// The 'Spacebar' key
    Spacebar,
}

/// All available keys
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyboardKey {
    /// An unrecognized key
    Unrecognized,
    /// An alpha-numeric key
    /// See 'AlphaNumeric'
    AlphaNumeric(AlphaNumeric),
    /// An arrow key
    /// See 'Arrow'
    Arrow(Arrow),
    /// A modifier key
    /// See 'Modifier'
    Modifier(Modifier),
    /// A special key
    /// See 'Special'
    Special(Special),
}

/// A structure representing a keyboard
#[derive(Clone)]
pub struct Keyboard {
    /// The current key states
    pub current_key_states: HashMap<KeyboardKey, KeyboardKeyState>,
    /// The last key states
    /// The states are being swapped each frame
    pub last_key_states: HashMap<KeyboardKey, KeyboardKeyState>,
}

impl Default for Keyboard {
    fn default() -> Self {
        let mut keys = HashMap::new();

        // Insert all alphanumeric keys
        for key in [
            AlphaNumeric::A,
            AlphaNumeric::B,
            AlphaNumeric::C,
            AlphaNumeric::D,
            AlphaNumeric::E,
            AlphaNumeric::F,
            AlphaNumeric::G,
            AlphaNumeric::H,
            AlphaNumeric::I,
            AlphaNumeric::J,
            AlphaNumeric::K,
            AlphaNumeric::L,
            AlphaNumeric::M,
            AlphaNumeric::N,
            AlphaNumeric::O,
            AlphaNumeric::P,
            AlphaNumeric::Q,
            AlphaNumeric::R,
            AlphaNumeric::S,
            AlphaNumeric::T,
            AlphaNumeric::U,
            AlphaNumeric::V,
            AlphaNumeric::W,
            AlphaNumeric::X,
            AlphaNumeric::Y,
            AlphaNumeric::Z,
            AlphaNumeric::Zero,
            AlphaNumeric::One,
            AlphaNumeric::Two,
            AlphaNumeric::Three,
            AlphaNumeric::Four,
            AlphaNumeric::Five,
            AlphaNumeric::Six,
            AlphaNumeric::Seven,
            AlphaNumeric::Eight,
            AlphaNumeric::Nine,
        ] {
            keys.insert(KeyboardKey::AlphaNumeric(key), KeyboardKeyState::Released);
        }

        // Insert all arrow keys
        for key in [Arrow::Up, Arrow::Down, Arrow::Left, Arrow::Right] {
            keys.insert(KeyboardKey::Arrow(key), KeyboardKeyState::Released);
        }

        // Insert all modifier keys
        for key in [
            Modifier::ShiftLeft,
            Modifier::ShiftRight,
            Modifier::ControlLeft,
            Modifier::ControlRight,
            Modifier::AltLeft,
            Modifier::AltRight,
        ] {
            keys.insert(KeyboardKey::Modifier(key), KeyboardKeyState::Released);
        }

        // Insert all special keys
        for key in [
            Special::Escape,
            Special::Enter,
            Special::Backspace,
            Special::Tab,
            Special::Delete,
            Special::Spacebar,
        ] {
            keys.insert(KeyboardKey::Special(key), KeyboardKeyState::Released);
        }

        Self {
            current_key_states: keys.clone(),
            last_key_states: keys,
        }
    }
}
