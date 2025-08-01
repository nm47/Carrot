use clap::Parser;
use std::error::Error;

// Import from the library part of this crate
extern crate carrot;
use carrot::parse_recipe_from_url;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Recipe URL to parse
    #[arg(short, long)]
    url: String,
    
    /// Output format
    #[arg(short, long, default_value = "markdown")]
    format: String,
    
    /// Show line-by-line scoring analysis
    #[arg(short, long)]
    score: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    
    println!("Fetching recipe from: {}", args.url);
    
    // Show debug message if requested
    if args.score {
        println!("\n--- DEBUG NOT IMPLEMENTED ---\n");
        return Ok(());
    }
    
    // Use unified parsing pipeline
    let result = parse_recipe_from_url(&args.url, &args.format)?;
    
    println!("\n--- PARSED RECIPE ---\n");
    println!("{}", result);
    
    Ok(())
}
