use std::fs::File;
use std::io::{self, Read, BufReader};
use md5::{Md5, Digest};
use sha1::Sha1;
use sha2::Sha256;

fn main() -> io::Result<()> {
    // User inputs
    println!("Please enter the checksum to compare:");
    let mut input_checksum = String::new();
    io::stdin().read_line(&mut input_checksum)?;
    let input_checksum = input_checksum.trim();

    println!("Please enter the file path:");
    let mut file_path = String::new();
    io::stdin().read_line(&mut file_path)?;
    let file_path = file_path.trim();

    println!("\nSelect the algorithm:");
    println!("1. MD5");
    println!("2. SHA1");
    println!("3. SHA256");
    println!("4. Fletcher's Checksum");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;
    let choice: u32 = choice.trim().parse().unwrap_or(0);

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

    println!("\nEntered checksum:  {}", input_checksum);
    println!("Calculated checksum: {}", calculated_checksum);

    if input_checksum.eq_ignore_ascii_case(&calculated_checksum) {
        println!("\nChecksums match! ✓");
    } else {
        println!("\nChecksums do NOT match! ✗");
    }

    Ok(())
}

fn calculate_md5(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Md5::new();
    io::copy(&mut file, &mut hasher)?;
    Ok(format!("{:x}", hasher.finalize()))
}

fn calculate_sha1(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha1::new();
    io::copy(&mut file, &mut hasher)?;
    Ok(format!("{:x}", hasher.finalize()))
}

fn calculate_sha256(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    io::copy(&mut file, &mut hasher)?;
    Ok(format!("{:x}", hasher.finalize()))
}

fn calculate_fletcher(path: &str) -> io::Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let (mut sum1, mut sum2) = (0u32, 0u32);
    for &byte in buffer.iter() {
        sum1 = (sum1 + byte as u32) % 65535;
        sum2 = (sum2 + sum1) % 65535;
    }
    Ok(format!("{:08x}", (sum2 << 16) | sum1))
}