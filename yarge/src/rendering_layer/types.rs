/// The possible output for the begin frame
pub enum RendererBeginFrameOutput {
    /// Can pursue to the end frame
    #[allow(unused)]
    Success,

    /// Should not present the frame
    #[allow(unused)]
    Failure,
}
