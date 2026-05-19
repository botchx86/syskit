use clap::{Arg, ArgMatches, Command};

/// Builds and returns the full argument matching hierarchy for the multitool.
pub fn build_cli() -> ArgMatches {
    Command::new("multitool")
        .author("botchx86")
        .version("1.0")
        .about("A cross-platform CLI swiss army knife to fully control your computer")
        .long_about("A unified tool combining file management, system administration, networking diagnostics, and cryptography tools.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        
        // --- CORE PILLAR: FILE SYSTEM ---
        .subcommand(
            Command::new("fs")
                .about("File system management (find, create, move, delete, etc.)")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("find")
                        .about("Deep search for a file by name")
                        .arg(Arg::new("name").required(true).help("The exact name or extension to look for"))
                        .arg(Arg::new("path").short('p').long("path").default_value(".").help("The starting directory for the search"))
                )
                .subcommand(
                    Command::new("create")
                        .about("Create an empty file or full directory path")
                        .arg(Arg::new("target_path").required(true).help("The path where the file or folder should be created"))
                        .arg(Arg::new("directory").short('d').long("directory").num_args(0).help("Create a directory structure instead of a file"))
                )
                .subcommand(
                    Command::new("move")
                        .about("Move or rename a file/directory")
                        .arg(Arg::new("source").required(true).help("Source path"))
                        .arg(Arg::new("destination").required(true).help("Destination path"))
                )
                .subcommand(
                    Command::new("delete")
                        .about("Delete a file safely (sends to system trash)")
                        .arg(Arg::new("target_path").required(true).help("The file or folder to delete"))
                )
        )
        
        // --- CORE PILLAR: SYSTEM CONTROL ---
        .subcommand(
            Command::new("sys")
                .about("System control and process management (kill, top, specs)")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(Command::new("ps").about("Show a list of all running processes"))
                .subcommand(
                    Command::new("kill")
                        .about("Terminate a process by PID or Name")
                        .arg(Arg::new("target").required(true).help("The Process ID (PID) or exact process name"))
                )
                .subcommand(Command::new("info").about("Print comprehensive OS, hardware, RAM, and CPU specs"))
        )
        
        // --- CORE PILLAR: NETWORKING ---
        .subcommand(
            Command::new("net")
                .about("Networking diagnostics and web tools (ping, portscan, download)")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("ping")
                        .about("Ping a host to check connection latency")
                        .arg(Arg::new("host").required(true).help("The target domain or IP address"))
                )
                .subcommand(
                    Command::new("portscan")
                        .about("Multi-threaded port scanner for a targeted IP")
                        .arg(Arg::new("ip").required(true).help("Target IP address"))
                        .arg(Arg::new("ports").short('p').long("ports").default_value("1-1024").help("The range of ports to scan"))
                )
                .subcommand(Command::new("ip").about("Display local interfaces, MAC addresses, and public WAN IP"))
                .subcommand(
                    Command::new("download")
                        .about("Download a file via URL (like curl/wget)")
                        .arg(Arg::new("url").required(true).help("The remote file URL"))
                        .arg(Arg::new("output").short('o').long("output").help("Optional local file path to save as"))
                )
        )
        
        // --- CORE PILLAR: SECURITY & CRYPTO ---
        .subcommand(
            Command::new("crypto")
                .about("Security, hashing, and cryptography (encrypt, wipe, passwords)")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("hash")
                        .about("Compute the cryptographic hash (MD5, SHA-256) of text or a file")
                        .arg(Arg::new("target").required(true).help("Text string or path to a file"))
                        .arg(Arg::new("file").long("file").num_args(0).help("Treat the target string as a file path instead of raw text"))
                )
                .subcommand(
                    Command::new("encrypt")
                        .about("Encrypt a file using AES-256-GCM")
                        .arg(Arg::new("file").required(true).help("The file path to secure"))
                )
                .subcommand(
                    Command::new("decrypt")
                        .about("Decrypt an encrypted file")
                        .arg(Arg::new("file").required(true).help("The file path to decrypt"))
                )
                .subcommand(
                    Command::new("wipe")
                        .about("Overwrite a file with random data multiple times before deleting")
                        .arg(Arg::new("file").required(true).help("The file to securely shred"))
                        .arg(Arg::new("passes").short('p').long("passes").default_value("3").help("Number of random overwrite cycles"))
                )
        )
        .get_matches()
}