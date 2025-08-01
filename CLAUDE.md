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

### Core Components

**System Architecture:**
- **Static Site Frontend** - Renders parsed recipe results in a user-friendly interface
- **CLI Tool** - Command-line interface for parsing recipes from URLs
- **Ingredient Parser** - Core parsing engine that processes recipe content

**Parser Pipeline:**
1. **HTML Extraction** (`carrot/extractor.py`) - Converts HTML to clean markdown using html2text
2. **Content Scoring** (`carrot/scorer.py`) - Scores lines based on recipe likelihood using corpus matching
3. **Structure Parsing** (`carrot/parser.py`) - Identifies sections and extracts structured data

**Scoring Algorithm:**
- **Ingredient Detection**: Match against comprehensive ingredient corpus
- **Measurement Patterns**: Identify quantity + unit combinations (1 cup, 2 tsp, etc.)
- **Step Identification**: Detect numbered steps and instruction patterns
- **Section Clustering**: Group related lines into ingredients vs instructions sections
- **Metadata Extraction**: Extract title, cook times, servings using regex patterns

**Data Sources:**
- `corpus/ingredients.txt` - 3,600+ ingredient names for matching
- `corpus/measurements.txt` - Common cooking measurements and units
- `corpus/step_patterns.txt` - Recipe instruction verbs and patterns
- `corpus/section_headers.txt` - Common recipe section headers
