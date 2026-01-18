#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::{config::Config, maths::Vector2};

use super::event::Event;

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Tells how the window should be displayed
pub enum DisplayMode {
    /// Fullscreen mode
    Fullscreen,

    /// Minimized mode
    Minimized,

    /// Floating mode
    /// Can optionally give the window's x and y positions
    /// as well as the window's width and height
    /// # Examples
    /// ```
    /// use yarge::DisplayMode;
    ///
    /// // To center the window
    /// let width=0.5;  // width in [0.,1.]
    /// let height=0.5; // height in [0.,1.]
    /// let x=0.5-(width/2.); // x in [0.,1.]
    /// let y=0.5-(height/2.); // y in [0.,1.]
    /// let mode = DisplayMode::Floating(Some((x, y, width, height)));
    /// ```
    Floating(Option<(f32, f32, f32, f32)>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Common properties for any platform specific window implementations
pub(crate) struct WindowCommonProperties {
    /// The window's position
    /// The position is such that
    /// `x` is the left of the window
    /// `y` is the top of the window
    /// The values are given between 0 and 1
    pub(crate) position: Vector2,

    /// The window's width
    /// The width must be between 0. and 1.
    /// 1 meaning the monitor's width
    pub(crate) width: f32,

    /// The window's height
    /// The height must be between 0. and 1.
    /// 1 meaning the monitor's height
    pub(crate) height: f32,

    /// The current display mode
    pub(crate) display_mode: DisplayMode,
}

/// Abstract trait for a window
/// The window's position is such that:
/// [0.,0.] is the top left corner of the monitor
/// [1.,0.] is the top right corner of the monitor
/// [0.,1.] is the bottom left corner of the monitor
/// [1.,1.] is the bottom right corner of the monitor
pub(crate) trait Window {
    /// The type of the struct implementing the trait
    /// This would often be `Self`
    type WindowType;

    /// Initializes the window
    fn init(config: &Config) -> Result<Self::WindowType, ErrorType>;

    /// Shuts down the window
    fn shutdown(&mut self) -> Result<(), ErrorType>;

    /// Gets the window's properties
    fn get_properties(&self) -> WindowCommonProperties;

    /// Poll the next event
    fn poll_event(&mut self) -> Result<Event, ErrorType>;

    /// Swaps the color buffer and show it as output to the screen
    #[cfg(opengl_renderer)]
    fn opengl_swap_buffers(&mut self) -> Result<(), ErrorType>;

    /// Makes the context of the window current on the calling thread
    #[cfg(opengl_renderer)]
    fn opengl_make_context_current(&mut self) -> Result<(), ErrorType>;

    /// Loads all the OpenGL functions
    #[cfg(opengl_renderer)]
    fn opengl_load_functions(&mut self) -> Result<(), ErrorType>;

    /// Get the required Vulkan extensions to interface with the window system
    #[cfg(vulkan_renderer)]
    fn vulkan_get_required_instance_extensions(
        &self,
    ) -> Result<
        Vec<crate::rendering_layer::rendering_impl::types::extensions::VkInstanceExtensions>,
        ErrorType,
    >;

    /// Get the Vulkan surface to interface with the window system
    #[cfg(vulkan_renderer)]
    fn vulkan_get_surface(
        &self,
        vk_entry: &ash::Entry,
        vk_instance: &ash::Instance,
        allocator: Option<&ash::vk::AllocationCallbacks<'_>>,
    ) -> Result<ash::vk::SurfaceKHR, ErrorType>;
}
