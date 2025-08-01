# CLAUDE.md

This file provides guidance to Claude Code when working with the Carrot recipe parsing project.

## Project Overview

Carrot is a recipe parsing system consisting of a static site frontend and underlying ingredient parser. The parser uses a hybrid approach combining rule-based parsing with scoring algorithms to identify recipe components before optional LLM refinement.

**Key Features:**
- Static site frontend for rendering parsed recipe results
- CLI tool for recipe parsing (`<cli_tool> --url <recipe_url>`)
- Rule-based HTML parsing with content scoring
- Ingredient corpus matching (3,600+ ingredients)
- Measurement and quantity extraction
- Recipe step identification and clustering
- Clean JSON output compatible with recipe websites
- URL scraping and direct HTML processing

## Architecture

  ### Web UI Flow

  1. User enters URL in textarea, selects format, clicks parse
  2. Frontend JS calls: window.parse_recipe(url, selectedFormat)
  3. WASM parse_recipe function:
     a. Constructs proxy URL: `/proxy?url=${encodeURIComponent(url)}`
     b. Browser fetch(proxyUrl)
     c. Extract html_string from proxy response
     d. Call: parse_recipe_from_content(html_string, selectedFormat)
  4. Return result to frontend

  ### CLI Flow

  1. User runs: cargo run --bin carrot-cli --url https://example.com --format markdown
  2. CLI calls: parse_recipe_from_url(url, format)
  3. parse_recipe_from_url function:
     a. Use reqwest::blocking::get(url) to fetch html_string
     b. Call: parse_recipe_from_content(html_string, format)
     c. Return result
  4. CLI prints result

  Function Architecture:

  // Core parsing logic - does all the work
  pub fn parse_recipe_from_content(html: &str, format: &str) -> String {
      // All parsing logic here
  }

  // Thin wrapper for CLI convenience
  pub fn parse_recipe_from_url(url: &str, format: &str) -> Result<String, Error> {
      let html = reqwest::blocking::get(url)?.text()?;
      Ok(parse_recipe_from_content(&html, format))
  }

  Both flows converge on the same parse_recipe_from_content function - URL fetching is just the transport layer.


### Core Components

**Data Sources:**
- `corpus/ingredients.txt` - 3,600+ ingredient names for matching
- `corpus/measurements.txt` - Common cooking measurements and units
- `corpus/verbs.txt` - Recipe instruction verbs and patterns
- `corpus/navigation_noise.txt` - Navigation elements and UI noise to filter out
- `corpus/metadata_noise.txt` - Recipe metadata and site content to filter out  
- `corpus/html_noise.txt` - HTML/CSS terms and technical content to filter out

## Development Notes

- We have the @scripts/run.sh command to bring everything up
- No need to run the frontend or proxy, the user will host it themselves
- WASM can't access file system - use `include_str!()` to embed corpus files at compile time
- CLI uses `cargo run --bin carrot-cli -- --url <url> --score` for scoring analysis

## Integration Testing

**CRITICAL**: Always run full integration tests after significant changes to ensure CLI and web interfaces remain synchronized.

### Running Integration Tests

**Test Commands:**
- All integration tests: `cargo test integration -- --nocapture`

**When to Run Integration Tests:**
- After changes to HTML parsing logic
- After modifications to the unified pipeline (`parse_recipe_from_content`)  
- After updates to WASM interface
- Before releasing or deploying changes
- When adding new output formats

**Test Standards:**
- All integration tests must pass
- CLI and web outputs must be byte-for-byte identical for same inputs
- Performance differences are acceptable (web has browser overhead)
- Any discrepancies indicate pipeline divergence and must be fixed

The integration tests validate that our unified pipeline truly produces identical results across both interfaces.
