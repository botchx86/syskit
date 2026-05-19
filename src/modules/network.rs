//! Low-overhead network sockets, socket-handshake alternatives, and data validation handlers.

use std::fs::File;
use std::io::Write;
use std::net::{IpAddr, TcpStream, ToSocketAddrs};
use std::time::Duration;
use futures_util::StreamExt;
use serde::Deserialize;

/// Structure to map incoming geolocation JSON payloads from public IP APIs.
#[derive(Deserialize, Debug)]
struct IpTelemetry {
    ip: String,
    org: Option<String>,
    city: Option<String>,
    region: Option<String>,
    country_name: Option<String>,
}

/// Validates host routing address viability via a micro-timeout TCP handshake sequence.
///
/// # Arguments
/// * `host` - Remote destination host identifier string slice.
pub fn check_connection(host: &str) {
    println!("Initializing lookup check sequence to endpoint: {}...", host);

    let target = format!("{}:80", host);
    match target.to_socket_addrs() {
        Ok(mut addrs) => {
            if let Some(addr) = addrs.next() {
                println!("Resolved remote token '{}' to physical location: {}", host, addr.ip());
                
                let timeout_duration = Duration::from_secs(3);
                match TcpStream::connect_timeout(&addr, timeout_duration) {
                    Ok(_) => println!("Network status: Connection verified successfully. Handshake valid."),
                    Err(e) => eprintln!("Network connection fault: Interface unreachable. Context: {}", e),
                }
            } else {
                eprintln!("Error: Resolution array parsed empty. Address target unresolvable.");
            }
        }
        Err(e) => eprintln!("Error: Core DNS resolution engine fault for address target '{}': {}", host, e),
    }
}

/// Iterates sequentially through targeted networking sockets to verify opening socket states.
///
/// # Arguments
/// * `ip` - Text string representation of valid target IP tracking.
/// * `port_range` - Formatted range sequence parsing rules (e.g. single target '80' or grouped '80-443').
pub fn scan_ports(ip: &str, port_range: &str) {
    let target_ip: IpAddr = match ip.parse() {
        Ok(parsed) => parsed,
        Err(_) => {
            eprintln!("Error: Input format evaluation fault. Out-of-bounds IP specification.");
            return;
        }
    };

    let ports_collection: Vec<u16> = if port_range.contains('-') {
        let structural_segments: Vec<&str> = port_range.split('-').collect();
        if structural_segments.len() == 2 {
            let starting_index = structural_segments[0].parse::<u16>().unwrap_or(1);
            let ending_index = structural_segments[1].parse::<u16>().unwrap_or(1);
            (starting_index..=ending_index).collect()
        } else {
            eprintln!("Error: Structural compliance fault. Parse parameters match syntax: 'start-end'.");
            return;
        }
    } else {
        match port_range.parse::<u16>() {
            Ok(single_port) => vec![single_port],
            Err(_) => {
                eprintln!("Error: Numeric evaluation out of boundaries for port tracking.");
                return;
            }
        }
    };

    println!("Initiating network scan process across array profile on host: {}...", target_ip);
    let timeout_duration = Duration::from_millis(300);
    let mut total_open_ports = 0;

    for targeted_port in ports_collection {
        let connection_address = std::net::SocketAddr::new(target_ip, targeted_port);
        if TcpStream::connect_timeout(&connection_address, timeout_duration).is_ok() {
            println!("  [+] Port Status Validation Verified: {:<5} -> STATUS: OPEN", targeted_port);
            total_open_ports += 1;
        }
    }

    println!("Scan verification matrix completed. Discovered {} listening nodes.", total_open_ports);
}

/// Pulls live localization telemetry from external provider interfaces over HTTPS.
pub async fn fetch_ip_info() -> Result<(), Box<dyn std::error::Error>> {
    println!("Querying external edge transit systems for geolocation matrix data...");
    
    let client = reqwest::Client::new();
    let response = client
        .get("https://ipapi.co/json/")
        .header("User-Agent", "rust-multitool/1.0")
        .send()
        .await?;

    if response.status().is_success() {
        let telemetry = response.json::<IpTelemetry>().await?;
        
        println!("Telemetry Packet Payload:");
        println!("  Address Identity:  {}", telemetry.ip);
        println!("  Transit Provider:  {}", telemetry.org.unwrap_or_else(|| "Unknown Provider".to_string()));
        println!(
            "  Physical Hub:      {}, {}, {}",
            telemetry.city.unwrap_or_else(|| "Unknown City".to_string()),
            telemetry.region.unwrap_or_else(|| "Unknown Region".to_string()),
            telemetry.country_name.unwrap_or_else(|| "Unknown Country".to_string())
        );
    } else {
        eprintln!("Network response fault: Remote edge systems returned status code {}", response.status());
    }

    Ok(())
}

/// Downloads a file over HTTP/HTTPS, saving it to disk while dynamically calculating live progress milestones.
///
/// # Arguments
/// * `url` - Fully qualified URL pointing to remote host resource.
/// * `output_path` - File system target destination path.
pub async fn download_file(url: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing transfer protocol connection to source node: {}", url);
    println!("Binding asset output tracking to location:          {}", output_path);

    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    
    if !response.status().is_success() {
        return Err(format!("Server responded with bad error code: {}", response.status()).into());
    }

    let total_size = response.content_length().unwrap_or(0);
    let mut file = File::create(output_path)?;
    let mut stream = response.bytes_stream();
    
    let mut downloaded_bytes: u64 = 0;
    let mut last_reported_milestone = 0;

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result?;
        file.write_all(&chunk)?;
        downloaded_bytes += chunk.len() as u64;

        if total_size > 0 {
            let current_percentage = ((downloaded_bytes as f64 / total_size as f64) * 100.0) as u64;
            let milestone = (current_percentage / 25) * 25;
            if milestone > last_reported_milestone && milestone <= 100 {
                println!("  Download process metrics: {}% of total byte chunks verified.", milestone);
                last_reported_milestone = milestone;
            }
        }
    }

    if total_size == 0 {
        println!("  Download metrics uncalculated: {} bytes written (unknown total scale).", downloaded_bytes);
    }

    println!("Data processing verified: System transfer stream synchronized successfully.");
    Ok(())
}