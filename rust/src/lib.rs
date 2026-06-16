use godot::prelude::*;
mod player;
mod wfc_node_3d;
struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
