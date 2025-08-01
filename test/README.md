# Carrot Integration Tests

This directory contains integration tests that verify the CLI and web interface produce identical outputs for the same inputs.

## Test Philosophy

The integration tests ensure that:
- CLI and web interface never diverge in their parsing output
- Changes to the parsing pipeline don't break either interface
- Both interfaces handle the same recipe URLs consistently

## Test Structure

- `test_cases.toml` - Test configuration and recipe URLs
- `../tests/integration_test.rs` - Main Rust integration tests
- `../tests/integration/mod.rs` - Test framework implementation

## Prerequisites

### For CLI-only tests:
```bash
# Just run Cargo tests
cargo test test_cli_only
```

### For full CLI vs Web tests:
1. **WebDriver**: Install geckodriver (Firefox)
   ```bash
   # For Firefox (geckodriver will be started automatically by tests)
   wget https://github.com/mozilla/geckodriver/releases/latest/download/geckodriver-v0.33.0-linux64.tar.gz
   tar -xzf geckodriver-*.tar.gz
   sudo mv geckodriver /usr/local/bin/
   ```

2. **Start Carrot dev server** (in another terminal):
   ```bash
   ./scripts/run.sh
   ```
   
   **Important**: The dev server must be running on http://localhost:8001 before running integration tests.

## Running Tests

### CLI-only tests (no browser required):
```bash
cargo test test_cli_only -- --nocapture
```

### Full integration tests (CLI vs Web):
```bash
# Make sure dev server and WebDriver are running first
cargo test test_cli_vs_web_output -- --nocapture
```

### All tests:
```bash
cargo test integration -- --nocapture
```

## Test Cases

Current test cases use Budget Bytes Orange Julius recipe in different formats:
- `https://www.budgetbytes.com/orange-julius/` (Markdown)  
- `https://www.budgetbytes.com/orange-julius/` (HTML)
- `https://www.budgetbytes.com/orange-julius/` (JSON)

Add more test cases by editing `test_cases.toml`.

## Test Output

Successful tests show:
```
✅ Budget Bytes Orange Julius - Markdown - PASSED
   CLI: 1250ms | Web: 2100ms
```

Failed tests show detailed diffs:
```
❌ Budget Bytes Orange Julius - Markdown - FAILED
🔍 Output Diff (CLI vs Web):
- This line only in CLI
+ This line only in Web
  This line is the same
```

## Adding New Test Cases

Edit `test_cases.toml`:
```toml
[[test_cases]]
name = "New Recipe - Markdown"
url = "https://example.com/recipe"
format = "markdown" 
description = "Description of test case"
```

## Troubleshooting

- **WebDriver connection failed**: Make sure geckodriver/chromedriver is running on port 4444
- **Dev server not found**: Make sure `./scripts/run.sh` is running on port 8001
- **CLI timeout**: Increase `timeout_ms` in `test_cases.toml`
- **Network issues**: Tests require internet access to fetch recipe URLs