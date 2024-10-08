mod consts;

pub mod opcode;
pub mod payloads;
pub mod permissions;
pub use consts::*;
pub mod color;
pub mod intents;
pub mod reaction;

#[allow(non_snake_case)]
mod AttachmentFlags {
    /// This attachment has been edited using the remix feature on mobile
    pub const IS_REMIX: u32 = 1 << 2;
}
