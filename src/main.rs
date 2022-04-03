mod sdl;

fn main() {
    if let Err(e) = sdl::sdl_init() {
        println!("{}", e);
        std::process::exit(1);
    }
}
