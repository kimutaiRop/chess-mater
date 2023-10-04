mod interface;
mod actions;

use godot::prelude::*;

struct ChessMater;

#[gdextension]
unsafe impl ExtensionLibrary for ChessMater {}
