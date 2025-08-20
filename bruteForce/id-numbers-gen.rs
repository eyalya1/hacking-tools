use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;
use std::io;

/// Validates an Israeli ID number using the official checksum algorithm
fn is_valid_israeli_id(digits: &[u32; 9]) -> bool {
    let weights = [1, 2, 1, 2, 1, 2, 1, 2];
    
    let sum: u32 = digits[0..8]
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| d * w)
        .map(|n| if n > 9 { n / 10 + n % 10 } else { n })
        .sum();

    (sum + digits[8]) % 10 == 0
}

/// Generates all valid Israeli IDs for a given prefix
fn generate_ids_for_prefix(prefix: u32, prefix_length: usize) -> Vec<String> {
    let mut results = Vec::new();
    let remaining_digits = 9 - prefix_length;
    let range_size = 10_u32.pow(remaining_digits as u32);
    
    for i in 0..range_size {
        let num = prefix * (10_u32.pow(remaining_digits as u32)) + i;
        let digits = [
            (num / 100_000_000) as u32 % 10,
            (num / 10_000_000) as u32 % 10,
            (num / 1_000_000) as u32 % 10,
            (num / 100_000) as u32 % 10,
            (num / 10_000) as u32 % 10,
            (num / 1_000) as u32 % 10,
            (num / 100) as u32 % 10,
            (num / 10) as u32 % 10,
            (num % 10) as u32,
        ];
        
        if is_valid_israeli_id(&digits) {
            let id_str = format!("{:09}", num);
            results.push(id_str);
        }
    }
    
    results
}

fn main() {
    println!("Israeli ID Generator with Custom Prefixes");
    println!("=========================================");
    println!();
    
    // Get user input for prefixes
    println!("Enter the prefixes you want to generate (comma-separated):");
    println!("Examples:");
    println!("  - Single digits: 0,2,3");
    println!("  - Two-digit patterns: 21,32");
    println!("  - Three-digit patterns: 215,302");
    println!("  - Mix: 0,21,215,3");
    println!();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    // Parse input
    let prefixes: Vec<&str> = input.trim().split(',').map(|s| s.trim()).collect();
    
    // Validate prefixes
    let mut valid_prefixes = Vec::new();
    for prefix in &prefixes {
        if prefix.chars().all(|c| c.is_ascii_digit()) && !prefix.is_empty() && prefix.len() <= 9 {
            valid_prefixes.push(prefix.to_string());
        } else {
            println!("Skipping invalid prefix: {}", prefix);
        }
    }
    
    if valid_prefixes.is_empty() {
        println!("No valid prefixes provided. Exiting.");
        return;
    }
    
    println!("Generating IDs for prefixes: {:?}", valid_prefixes);
    println!("This may take a while depending on the prefix lengths...");
    
    let start_time = Instant::now();
    let mut all_ids = Vec::new();
    
    // Generate IDs for each prefix
    for prefix in &valid_prefixes {
        let prefix_value = prefix.parse::<u32>().unwrap();
        let prefix_length = prefix.len();
        
        println!("Generating IDs for prefix: {}...", prefix);
        let prefix_ids = generate_ids_for_prefix(prefix_value, prefix_length);
        println!("Found {} valid IDs for prefix {}", prefix_ids.len(), prefix);
        
        all_ids.extend(prefix_ids);
    }
    
    // Write to file
    let filename = "israeli_ids_custom.txt";
    let file = File::create(filename).expect("Unable to create file");
    let mut writer = BufWriter::with_capacity(64 * 1024, file);
    
    for id in &all_ids {
        writeln!(writer, "{}", id).expect("Unable to write to file");
    }
    
    let duration = start_time.elapsed();
    
    println!("\nGenerated {} valid IDs", all_ids.len());
    println!("Execution time: {:?}", duration);
    println!("Results saved to {}", filename);
    
    // Show some sample IDs
    println!("\nSample of first 10 IDs:");
    for id in all_ids.iter().take(10) {
        println!("{}", id);
    }
    
    // Show counts by prefix
    println!("\nCounts by prefix:");
    for prefix in &valid_prefixes {
        let count = all_ids.iter().filter(|id| id.starts_with(prefix)).count();
        println!("Prefix {}: {} IDs", prefix, count);
    }
}