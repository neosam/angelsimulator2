//! All system which set up states are located here.
pub mod gameover_startup_system;
pub mod heaven_startup_system;
pub mod ingame_startup_system;

pub use gameover_startup_system::gameover_startup_system;
pub use heaven_startup_system::heaven_startup_system;
pub use ingame_startup_system::ingame_startup_system;
