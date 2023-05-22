use std::io::{self, Write};
use colored::*;

struct User<T>
where
    T: AsRef<str>
{
    user_name: T,
    colour: (u8, u8, u8),
}

struct Connection {
    ip: String,
    port: usize,
}

fn get_user_creds() {
    let mut s = String::new();
    print!("Please enter your username: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read input");

    let mut r = String::new();
    let mut g = String::new();
    let mut b = String::new();

    print!("User colour setup (RGB) - R: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut r)
        .expect("Failed to read input");

    print!("User colour setup (RGB) - G: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut g)
        .expect("Failed to read input");

    print!("User colour setup (RGB) - B: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut b)
        .expect("Failed to read input");

    let r: u8 = r.trim().parse().expect("Failed to parse input");
    let g: u8 = g.trim().parse().expect("Failed to parse input");
    let b: u8 = b.trim().parse().expect("Failed to parse input");
    rgb_check(r, g, b);

    let current_user = User {
        user_name: s,
        colour: (r, g, b),
    };
    println!("Welcome {}!", current_user.user_name.truecolor((current_user.colour.0),(current_user.colour.1), (current_user.colour.2)));
}


fn rgb_check(r: u8, g: u8, b: u8) {
    let r = if r > 255 { 255 } else { r };
    let g = if g > 255 { 255 } else { g };
    let b = if b > 255 { 255 } else { b };
}

fn get_connection_info() {
    let mut ip = String::new();
    print!("Please enter the server ip: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut ip)
        .expect("Failed to read input");

    let mut port = String::new();
    print!("Port number: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut port)
        .expect("Failed to read input");
}









fn main() {
    get_user_creds();
}
