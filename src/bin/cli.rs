use clap::Parser;
use html2text;
use reqwest;
use std::error::Error;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Recipe URL to parse
    #[arg(short, long)]
    url: String,
    
    /// Output format
    #[arg(short, long, default_value = "markdown")]
    format: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    
    println!("Fetching recipe from: {}", args.url);
    
    // Fetch HTML content
    let response = reqwest::blocking::get(&args.url)?;
    
    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()).into());
    }
    
    let html = response.text()?;
    println!("Fetched {} bytes of HTML", html.len());
    
    // Convert to markdown/text
    let markdown = html2text::from_read(html.as_bytes(), 80);
    
    match args.format.as_str() {
        "markdown" | "text" => {
            println!("\n--- PARSED RECIPE ---\n");
            println!("{}", markdown);
        }
        "raw" => {
            println!("\n--- RAW HTML ---\n");
            println!("{}", html);
        }
        _ => {
            println!("Unknown format: {}. Using markdown.", args.format);
            println!("\n--- PARSED RECIPE ---\n");
            println!("{}", markdown);
        }
    }
    
    Ok(())
}