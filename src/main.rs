use std::fs::File;
use std::io::{self, Read, BufReader};
use md5::{Md5, Digest};
use sha1::Sha1;
use sha2::Sha256;

fn main() -> io::Result<()> {
    // Prompt user for the known checksum they want to compare against
    println!("Please enter the checksum to compare:");
    let mut input_checksum = String::new();
    io::stdin().read_line(&mut input_checksum)?;
    let input_checksum = input_checksum.trim();

    // Get the path to the file that needs to be verified
    println!("Please enter the file path:");
    let mut file_path = String::new();
    io::stdin().read_line(&mut file_path)?;
    let file_path = file_path.trim();

    // Display available hashing algorithms
    println!("\nSelect the algorithm:");
    println!("1. MD5");
    println!("2. SHA1");
    println!("3. SHA256");
    println!("4. Fletcher's Checksum");

    // Read user's algorithm choice
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;
    // Parse the choice to a number, defaulting to 0 if parsing fails
    let choice: u32 = choice.trim().parse().unwrap_or(0);

    // Calculate checksum based on the selected algorithm
    let calculated_checksum = match choice {
        1 => calculate_md5(file_path)?,
        2 => calculate_sha1(file_path)?,
        3 => calculate_sha256(file_path)?,
        4 => calculate_fletcher(file_path)?,
        _ => {
            println!("Invalid selection!");
            return Ok(());
        }
    };

    // Display both checksums for comparison
    println!("\nEntered checksum:  {}", input_checksum);
    println!("Calculated checksum: {}", calculated_checksum);

    // Compare checksums (case-insensitive) and show the result
    if input_checksum.eq_ignore_ascii_case(&calculated_checksum) {
        println!("\nChecksums match! ✓");
    } else {
        println!("\nChecksums do NOT match! ✗");
    }

    Ok(())
}

/// Calculates MD5 hash of a file
/// Takes a file path as input and returns the hex-encoded hash string
fn calculate_md5(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Md5::new();
    // Copy file contents into hasher
    io::copy(&mut file, &mut hasher)?;
    // Convert the hash result to a hexadecimal string
    Ok(format!("{:x}", hasher.finalize()))
}

/// Calculates SHA1 hash of a file
/// Takes a file path as input and returns the hex-encoded hash string
fn calculate_sha1(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha1::new();
    io::copy(&mut file, &mut hasher)?;
    Ok(format!("{:x}", hasher.finalize()))
}

/// Calculates SHA256 hash of a file
/// Takes a file path as input and returns the hex-encoded hash string
fn calculate_sha256(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    io::copy(&mut file, &mut hasher)?;
    Ok(format!("{:x}", hasher.finalize()))
}

/// Calculates Fletcher's checksum of a file
/// Takes a file path as input and returns the hex-encoded checksum string
/// Fletcher's checksum is a simple error-detection algorithm that uses two sums
fn calculate_fletcher(path: &str) -> io::Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    // Read entire file into buffer
    reader.read_to_end(&mut buffer)?;

    // Initialize the two sums used in Fletcher's algorithm
    let (mut sum1, mut sum2) = (0u32, 0u32);
    
    // Process each byte of the file
    for &byte in buffer.iter() {
        // Update sum1 with the byte value, keeping it within bounds
        sum1 = (sum1 + byte as u32) % 65535;
        // Update sum2 with the current sum1 value, keeping it within bounds
        sum2 = (sum2 + sum1) % 65535;
    }
    
    // Combine both sums into a single 32-bit checksum and format as hex
    Ok(format!("{:08x}", (sum2 << 16) | sum1))
}