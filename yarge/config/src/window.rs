use maths::Vector2;

pub struct WindowConfig {
    pub title: String,
    pub position: Vector2,
    pub width: f32,
    pub height: f32,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self { 
            title: String::from("NewWindow"), 
            position: Vector2::ZEROS,
            width: 1., 
            height: 1., 
        }
    }
}