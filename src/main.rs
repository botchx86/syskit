//! Main execution entry point for the syskit application.
//! Enforces defensive argument parsing and clean execution dispatching with an elegant dashboard interface.

use clap::{Arg, ArgAction, Command};

// Tell Rust to look for a subfolder module directory named 'modules'
mod modules;
use modules::{network, security, system};

// Global application version track
const VERSION: &str = "v0.9.1";

#[tokio::main]
async fn main() {
    let mut app = Command::new("syskit")
        .about("A modern CLI Swiss Army Knife")
        .version(VERSION)
        .subcommand(
            Command::new("ps")
                .about("List running processes")
        )
        .subcommand(
            Command::new("kill")
                .about("Terminate a process by name")
                .arg(
                    Arg::new("target")
                        .short('t')
                        .long("target")
                        .required(true)
                        .help("The name of the target process to terminate"),
                ),
        )
        .subcommand(
            Command::new("sysinfo")
                .about("Show system diagnostics and uptime")
        )
        .subcommand(
            Command::new("ping")
                .about("Check a remote host connection")
                .arg(
                    Arg::new("host")
                        .short('s')
                        .long("host")
                        .required(true)
                        .help("The hostname or IP address to ping"),
                ),
        )
        .subcommand(
            Command::new("scan")
                .about("Scan target ports for connectivity")
                .arg(
                    Arg::new("ip")
                        .short('i')
                        .long("ip")
                        .required(true)
                        .help("The target IP address to scan"),
                )
                .arg(
                    Arg::new("ports")
                        .short('p')
                        .long("ports")
                        .required(true)
                        .help("The port range to scan (e.g., 80-443)"),
                ),
        )
        .subcommand(
            Command::new("ipinfo")
                .about("Fetch public IP geolocation information")
        )
        .subcommand(
            Command::new("download")
                .about("Download a remote file")
                .arg(
                    Arg::new("url")
                        .short('u')
                        .long("url")
                        .required(true)
                        .help("The remote source URL string"),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .required(true)
                        .help("The local destination file path saving target"),
                ),
        )
        .subcommand(
            Command::new("hash")
                .about("Compute cryptographic signatures")
                .arg(
                    Arg::new("target")
                        .short('t')
                        .long("target")
                        .required(true)
                        .help("The text string or file path data source to hash"),
                )
                .arg(
                    Arg::new("file")
                        .short('f')
                        .long("file")
                        .action(ArgAction::SetTrue)
                        .help("Treat the target value as a file path instead of literal text"),
                ),
        )
        .subcommand(
            Command::new("encrypt")
                .about("Encrypt a file using AES-256-GCM")
                .arg(
                    Arg::new("file")
                        .short('f')
                        .long("file")
                        .required(true)
                        .help("The source file system path to encrypt"),
                ),
        )
        .subcommand(
            Command::new("decrypt")
                .about("Decrypt an encrypted file")
                .arg(
                    Arg::new("file")
                        .short('f')
                        .long("file")
                        .required(true)
                        .help("The encrypted input payload file path"),
                ),
        )
        .subcommand(
            Command::new("wipe")
                .about("Securely shred a file from storage")
                .arg(
                    Arg::new("file")
                        .short('f')
                        .long("file")
                        .required(true)
                        .help("The precise file path destination to overwrite and destroy"),
                )
                .arg(
                    Arg::new("passes")
                        .short('p')
                        .long("passes")
                        .value_parser(clap::value_parser!(u32))
                        .default_value("3")
                        .help("Number of consecutive overwrite execution routines"),
                ),
        );

    let matches = app.clone().get_matches();

    match matches.subcommand() {
        Some(("ps", _)) => {
            system::list_processes();
        }
        Some(("kill", sub_m)) => {
            if let Some(target) = sub_m.get_one::<String>("target") {
                system::kill_process_by_name(target);
            }
        }
        Some(("sysinfo", _)) => {
            system::print_system_info();
        }
        Some(("ping", sub_m)) => {
            if let Some(host) = sub_m.get_one::<String>("host") {
                network::check_connection(host);
            }
        }
        Some(("scan", sub_m)) => {
            if let (Some(ip), Some(ports)) = (sub_m.get_one::<String>("ip"), sub_m.get_one::<String>("ports")) {
                network::scan_ports(ip, ports);
            }
        }
        Some(("ipinfo", _)) => {
            if let Err(e) = network::fetch_ip_info().await {
                eprintln!("Error fetching IP metrics: {}", e);
            }
        }
        Some(("download", sub_m)) => {
            if let (Some(url), Some(output)) = (sub_m.get_one::<String>("url"), sub_m.get_one::<String>("output")) {
                if let Err(e) = network::download_file(url, output).await {
                    eprintln!("Error executing data acquisition pipeline: {}", e);
                }
            }
        }
        Some(("hash", sub_m)) => {
            if let Some(target) = sub_m.get_one::<String>("target") {
                let is_file = sub_m.get_flag("file");

                if is_file {
                    match std::fs::read(target) {
                        Ok(bytes) => security::hash_data(&bytes),
                        Err(e) => eprintln!("Error: Failed to read target file context: {}", e),
                    }
                } else {
                    security::hash_data(target.as_bytes());
                }
            }
        }
        Some(("encrypt", sub_m)) => {
            if let Some(file) = sub_m.get_one::<String>("file") {
                match std::fs::read(file) {
                    Ok(bytes) => {
                        let key = security::generate_secure_key();
                        match security::encrypt_bytes(&bytes, &key) {
                            Ok(encrypted) => {
                                let target_path = format!("{}.enc", file);
                                if std::fs::write(&target_path, encrypted).is_ok() {
                                    println!("Encrypted file written to: {}", target_path);
                                    println!("Encryption Key (Hex String representation): {:?}", key);
                                }
                            }
                            Err(e) => eprintln!("Error: Cryptographic fault during encryption processing: {}", e),
                        }
                    }
                    Err(e) => eprintln!("Error: Target resource file unreadable: {}", e),
                }
            }
        }
        Some(("decrypt", _)) => {
            println!("Decryption feature pipeline triggered.");
        }
        Some(("wipe", sub_m)) => {
            if let Some(file) = sub_m.get_one::<String>("file") {
                let passes = *sub_m.get_one::<u32>("passes").unwrap_or(&3);
                security::wipe_file(file, passes);
            }
        }
        None => {
            // ANSI escape sequences for colorization
            let cyan = "\x1b[36m";
            let green = "\x1b[32m";
            let bold = "\x1b[1m";
            let reset = "\x1b[0m";

            // Print beautiful ASCII art header
            println!("{}", cyan);
            println!(r#"     _______  __   __  _______  ___   _  ___   _______ "#);
            println!(r#"    |       ||  | |  ||       ||   | | ||   | |       |"#);
            println!(r#"    |  _____||  |_|  ||  _____||   |_| ||   | |_     _|"#);
            println!(r#"    | |_____ |       || |_____ |      _||   |   |   |  "#);
            println!(r#"    |_____  ||_     _||_____  ||     |_ |   |   |   |  "#);
            println!(r#"     _____| |  |   |   _____| ||    _  ||   |   |   |  "#);
            println!(r#"    |_______|  |___|  |_______||___| |_||___|   |___|  "#);
            println!("{}", reset);

            println!("    {}syskit {}{} - A modern CLI Swiss Army Knife for systems administration", bold, VERSION, reset);
            println!("    {}{}{}", green, "    =========================================================================", reset);
            println!();

            // Run system diagnostic summary directly inside the landing page
            println!("    {}{}► CURRENT SYSTEM STATUS SUMMARY:{}", green, bold, reset);
            system::print_system_info();
            println!();

            // Print the auto-generated Clap usage documentation block cleanly underneath 
            println!("    {}{}► AVAILABLE UTILITY COMMANDS:{}", green, bold, reset);
            let _ = app.print_help();
            println!();
        }
        _ => {
            eprintln!("Error: Unrecognized command fallback routing executed.");
        }
    }
}