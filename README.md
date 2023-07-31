# Wallpaper Engine KDE - Rust

## Description

I am trying to create a wallpaper engine for KDE using Rust for some reasons:

1. The current version uses Vulkan as a backend, which is not supported by Asahi Linux.
2. The installation puts the .so file in the system level directory, which is not necessary.
3. It uses rpc to communicate between the frontend and the backend. I dont like this approach. 
4. Having fun with Rust.

## Roadmap

- [x] Make a KDE wallpaper plugin which can be found in the wallpaper settings.
- [x] Provide our functionality to the QML frontend as a .so file.
- [x] Render the wallpaper using Qt GSNode.
- [x] Render a triangle using OpenGL and Qt.
- [ ] Hardcode to render a walllpaper from the steam workshop.

## Issues

- Probably unnecessary to all-in Rust. Remake the rendering part in C++ and port it to Rust. Because the Qt bindings are not stable yet. Indeed, I can port more Qt APIs, but it is not worth it.
- If above, do the config parsing part, and other parts, which are not related to Qt, in Rust.
