# cubedesu

Rubik's Cube simulator written in Rust. 

## Features
- Visual simulation of a 3x3 cube, allowing moves with keyboard input (csTimer's standard virtual cube keyboard mapping).

## Usage
Build the project (optionally in release mode):
```sh
$ cargo build [--release]
```

Run the project:
```sh
$ cargo run
```

## Project Structure
- ``src/lib``: Cube related types, such as a Face, Move, Turn, Movement (a Move associated with a Turn)
- ``src/geometric_model``: Geometric implementation of a cube, represented with an array of all the cube's stickers (3-dimensional points) and moves as rotations amongst some axis
- ``src/facelet_model``: Facelet implementation of a cube, represented as an array of stickers, ordered by U, R, F, D, L, then B face

## Resources Used
- [Onionhoney's extremely well written article on modelling Rubik's Cubes](https://observablehq.com/@onionhoney/how-to-model-a-rubiks-cube)
