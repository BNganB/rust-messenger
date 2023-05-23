use std::io::{self, Write};
use colored::*;
use std::process::Command;

fn main() {
    get_user_creds();
    make_connection(get_connection_info());
}

struct User {
    user_name: String,
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
    let processed_r: u8 = r.trim().parse::<u8>().expect("Failed to parse R");

    print!("User colour setup (RGB) - G: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut g)
        .expect("Failed to read input");
    let processed_g: u8 = g.trim().parse::<u8>().expect("Failed to parse G");

    print!("User colour setup (RGB) - B: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut b)
        .expect("Failed to read input");
    let processed_b: u8 = b.trim().parse::<u8>().expect("Failed to parse B");

    let current_user = User {
        user_name: s.trim().to_string(),
        colour: (processed_r, processed_g, processed_b),
    };
    println!(
        "Welcome {}!",
        current_user
            .user_name
            .truecolor(current_user.colour.0, current_user.colour.1, current_user.colour.2)
    );
}

pub fn get_connection_info() -> (String, usize) {
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

    let port = port.trim().parse::<usize>().expect("Failed to parse port");

    (ip.trim().to_string(), port)
}

fn make_connection((ip, port): (String, usize)) {
    let current_connection = Connection { ip, port };
    let command = format!("nc localhost {}", current_connection.port);  //change from localhost later

    println!("Establishing connection...");

    

    let output = Command::new("sh")
    .arg("-c")
    .arg(command)
    .output()
    .expect("Failed to connect");

    io::stdout().write_all(&output.stdout).unwrap()
}


