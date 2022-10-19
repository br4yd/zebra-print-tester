use std::{
    fs::{self, File},
    io::Write,
    net::TcpStream,
    path::Path,
    process::exit,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    file: String,
    ip: String,
    port: String,
}

fn main() {
    // Check if the config.yaml exists and create it if not.
    config_check();

    let f = std::fs::File::open("config.yaml").expect("Could not read file.");
    let scrape_config: Config = serde_yaml::from_reader(f).expect("Could not read values");

    println!("Input-File: {}", scrape_config.file);
    println!("IP: {}", scrape_config.ip);
    println!("Port: {}", scrape_config.port);

    let conn_string = format!("{}:{}", scrape_config.ip, scrape_config.port);

    let file_path = scrape_config.file;
    println!("In file: {}", file_path);

    println!("Press ENTER to continue...");
    std::io::stdin().read_line(&mut String::new()).unwrap();

    let contents = fs::read_to_string(file_path).expect("Wasn't able to read the file.");

    let mut stream = TcpStream::connect(conn_string).expect("Couldn't connect.");
    stream
        .write(contents.to_string().as_bytes())
        .expect("Couldn't write.");

    println!("Press ENTER to exit...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}

fn config_check() {
    if !Path::new("config.yaml").exists() {
        println!("Generating empty config.yaml. Please fill out the config file and retry!");

        let mut file = File::create("config.yaml").unwrap();

        file.write("file:\nip:\nport:".to_string().as_bytes())
            .expect("Couldn't write file.");

        println!("Press ENTER to exit...");
        std::io::stdin().read_line(&mut String::new()).unwrap();
        exit(0);
    }
}
