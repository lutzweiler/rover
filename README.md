# rover
**r**ust **O**FF **v**isualiz**er** - a simple tool for displaying bezier surfaces in [.off-file](
http://www.geomview.org/docs/html/OFF.html) representation

## Installation
On Linux or Windows you can download the executable from the **Releases** section in the sidebar

Alternatively you can build the application yourself:  
- Install the [Rust toolchain] (https://www.rust-lang.org/)
- On Windows, install the [Visual C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- install the nightly version: `rustup install nightly`
- clone this repository and open a terminal there
- build the application: `cargo build --release`
- view the example surface: `cargo run --release -- example_files/cbez333.off`
- optionally, move the executable from `rover/target/release/rover` to somewhere else

## Usage
call rover with an off file as the first argument  
`rover example_cbez333.off`

Rotate the camera with the mouse.  
Move the camera with `W` `A` `S` `D` as well as `Space` and `Shift` for vertical movement.  
Quit the app with `Esc` or `Ctrl`+`Q`.  

## Supported Primitives
- colored bezier surfaces in [CBEZ](http://www.geomview.org/docs/html/BBP-and-BEZ.html#BBP-and-BEZ) format
    - bilinear (_CBEZ113_), biquadratic (_CBEZ223_), bicubic (_CBEZ333_) and biquartic (_CBEZ443_)
- triangles [OFF](http://www.geomview.org/docs/html/OFF.html#OFF)
    - triangles for wich every vertex is supplied with color values will be be drawn with per-vertex coloring
    - other triangles will get a default color
    - coloring happens whether _OFF_ or _COFF_ header keyword is used
