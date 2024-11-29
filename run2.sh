# Build the Rust plugin
cd rust/plugin
cargo build

# Change back to root directory and open Godot project
cd ../..

# Replace moddable-pong-2
rm -rf moddable-pong-2
cp -r moddable-pong moddable-pong-2

/Applications/Godot.app/Contents/MacOS/Godot moddable-pong-2/project.godot
