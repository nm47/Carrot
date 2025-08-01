pub mod carrot_frontend;

use std::error::Error;
use std::fmt;

// Re-export WASM functions for the library interface
pub use carrot_frontend::wasm::*;

// Error type for recipe parsing
#[derive(Debug)]
pub struct RecipeError {
    message: String,
}

impl fmt::Display for RecipeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Recipe parsing error: {}", self.message)
    }
}

impl Error for RecipeError {}

impl From<reqwest::Error> for RecipeError {
    fn from(err: reqwest::Error) -> Self {
        RecipeError {
            message: format!("HTTP request failed: {}", err),
        }
    }
}

/// Core parsing logic - processes HTML content and returns formatted result
pub fn parse_recipe_from_content(html: &str, format: &str) -> String {
    match format {
        "text" => {
            "Plaintext format not yet implemented".to_string()
        }
        "json" => {
            // TODO: Implement JSON format
            "JSON format not yet implemented".to_string()
        }
        _ => {
            // Default to markdown
            "Markdown not yet implemented".to_string()
        }
    }
}

/// Thin wrapper for CLI convenience - fetches URL and calls core parsing
#[cfg(not(target_arch = "wasm32"))]
pub fn parse_recipe_from_url(url: &str, format: &str) -> Result<String, RecipeError> {
    let response = reqwest::blocking::get(url)?;
    
    if !response.status().is_success() {
        return Err(RecipeError {
            message: format!("HTTP error: {}", response.status()),
        });
    }
    
    let html = response.text()?;
    Ok(parse_recipe_from_content(&html, format))
}
