/// Clears the screen for use with Alacritty terminal on Windows.
fn main() {  
    unsafe {
        libc::system((b"cls" as * const u8) as * const i8);
    }
}
