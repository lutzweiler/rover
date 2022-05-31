# rover
**r**ust **O**FF **v**isualiz**er** - a simple tool for displaying bezier surfaces in [.off-file](
http://www.geomview.org/docs/html/OFF.html) representation

## Installation
On Linux or Windows you can download the executable from the **Releases** section in the sidebar

Alternatively you can build the application yourself:  
- Install the [Rust](https://www.rust-lang.org/)
- On Windows, install the [Visual C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- install the nightly version: `rustup install nightly`
- clone this repository and open a terminal there
- build the application: `cargo build --release`
- view the example surface: `cargo run --release -- example_cbez333.off`
- optionally, move the executable from `rover/target/release/rover` to somewhere else

## Usage
call rover with an off file as the first argument  
`rover example_cbez333.off`

Rotate the camera with the mouse  
Move the camera with `W` `A` `S` `D` as well as `Space` and `Shift` for vertical movement  

## Supported Primitives
Currently only the colored bicubic bezier surface [CBEZ333](http://www.geomview.org/docs/html/BBP-and-BEZ.html#BBP-and-BEZ) is supported
