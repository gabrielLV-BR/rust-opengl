# Rust OpenGL 🦀

Hello! This is a simple OpenGL renderer I'm creating in order to learn the API (as well as Rust itself)
This is probably my tenth attempt at making something like this, although, this time, I've gotten further than before

# What can it do?

Not much, for now. I plan on continuously updating it and making it more powerful and versatile. I don't expect it to
become something that people will use, but maybe they'll be able to take a look at the code to figure something out? I
don't know, I'm doing it because it's fun!

# Features

* Support for OBJ files
* Supports for simple and textured materials

That's basically it.

# Building

Thanks to Rust and it's amazing package manager (❤️ Cargo), it should be pretty straightforward to download the source code and run.
Just check that you have every `glfw` dependency installed and `cargo run`.

# What am I using?

* [Rust](https://www.rust-lang.org) - Language
* OpenGL ([gl](https://docs.rs/gl/latest/gl/) and [glfw](https://docs.rs/glfw/latest/glfw/) crate) - Graphics API
* [tobj](https://docs.rs/tobj/latest/tobj/) - OBJ loading
* [ultraviolet](https://docs.rs/ultraviolet/latest/ultraviolet/) - 3D math library
* [image](https://docs.rs/image/latest/image) - Image loading