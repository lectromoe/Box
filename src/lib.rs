/// Player action structures
pub mod actions;

/// Player & editor cameras
pub mod camera;

/// Develper runtime console
pub mod console;

/// UI debugger
pub mod debugger;

pub mod prelude {
    pub use crate::actions::*;
    pub use crate::camera::*;
    pub use crate::console::*;
    pub use crate::debugger::*;
}
