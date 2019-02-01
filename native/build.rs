extern crate neon_build;

fn main() {
    neon_build::setup(); // must be called in build.rs

    // add project-specific build logic here...
    println!("cargo:rustc-flags=-l avformat  -l avcodec -l swscale -l avutil -l swresample -l z -l sdl2");
}
