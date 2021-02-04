# zendemo

OpenGL-based voxel terrain demo using the marching cubes algorithm, featuring an endless network of colourful caves.

## Getting Started
### Prerequisites

Before running, you'll need to have installed Rust. You can follow the Rust setup guide [here](https://www.rust-lang.org/learn/get-started) to get started.

### Building & Running

First, clone the repository.
```
git clone https://github.com/lcflint/zendemo.git
```
Then, to build the project, run the following commands:
```
cd zendemo
cargo build
```
Optionally, you can also use `cargo run` to run the project without building the binaries.

## Notes
You can navigate around using the WASD keys to move and the arrow keys to turn the camera. There isn't anything in place right now to deal with disposing of old buffer data, so you may experience high memory use if you go too far.

## Screenshots
![Cave Screenshot 1](https://i.imgur.com/m7nDEqm.png)
![Cave Screenshot 2](https://i.imgur.com/TTrpRvu.png)
![Cave Screenshot 3](https://i.imgur.com/Akj4rbo.png)
