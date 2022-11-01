type Bit = bool;

pub struct Display;
impl Display {
    pub const WIDTH: usize = 64;
    pub const HEIGHT: usize = 32;

    pub fn print_framebuffer(framebuffer: [Bit; 2048]) {
        // clear
        print!("{esc}c", esc = 27 as char);

        for y in 0..Display::HEIGHT {
            for x in 0..Display::WIDTH {
                match framebuffer[y * Display::WIDTH + x] {
                    true => print!("* "),
                    false => print!("  "),
                }
            }
            println!();
        }
    }
}
