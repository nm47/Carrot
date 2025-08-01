use serde::Deserialize;
use std::process::{Command, Child, Stdio};
use std::time::{Duration, Instant};
use similar::{ChangeTag, TextDiff};
use std::thread;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub timeout_ms: u64,
    pub dev_server_url: String,
}

#[derive(Debug, Deserialize)]
pub struct TestCase {
    pub name: String,
    pub url: String,
    pub format: String,
}

#[derive(Debug, Deserialize)]
pub struct TestCases {
    pub config: Config,
    pub test_cases: Vec<TestCase>,
}

#[derive(Debug)]
pub struct TestResult {
    pub name: String,
    pub success: bool,
    pub cli_duration: Duration,
    pub web_duration: Duration,
    pub error: Option<String>,
}

/// Load test cases from test/test_cases.toml
pub fn load_test_cases() -> Result<TestCases, Box<dyn std::error::Error>> {
    let test_file = std::fs::read_to_string("test/test_cases.toml")?;
    let test_cases: TestCases = toml::from_str(&test_file)?;
    Ok(test_cases)
}

/// Start a WebDriver instance (geckodriver)
pub fn start_webdriver() -> Result<Child, String> {
    println!("🚀 Starting geckodriver on port 4444...");
    
    let child = Command::new("geckodriver")
        .args(&["--port", "4444", "--log", "fatal"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("Failed to start geckodriver: {}. Make sure geckodriver is installed and in PATH.", e))?;
    
    // Give geckodriver time to start up
    thread::sleep(Duration::from_secs(2));
    
    // Test if it's actually running by trying to connect
    match std::process::Command::new("curl")
        .args(&["-s", "http://localhost:4444/status"])
        .output() 
    {
        Ok(output) if output.status.success() => {
            println!("✅ WebDriver started successfully");
            Ok(child)
        }
        _ => {
            // Try to kill the child process if curl failed
            Err("Failed to verify WebDriver is running".to_string())
        }
    }
}

/// Stop a WebDriver instance
pub fn stop_webdriver(mut child: Child) {
    println!("🛑 Stopping WebDriver...");
    let _ = child.kill();
    let _ = child.wait();
}

/// Run the CLI with given URL and format
pub fn run_cli(url: &str, format: &str, _timeout: Duration) -> Result<(String, Duration), String> {
    let start = Instant::now();
    
    let output = Command::new("cargo")
        .args(&["run", "--bin", "carrot-cli", "--", "--url", url, "--format", format])
        .output()
        .map_err(|e| format!("Failed to execute CLI: {}", e))?;
    
    let duration = start.elapsed();
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok((stdout, duration))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Err(format!("CLI failed: {}", stderr))
    }
}

/// Run web interface test using fantoccini (headless browser)
pub async fn run_web_test(url: &str, format: &str, server_url: &str, _timeout: Duration) 
    -> Result<(String, Duration), String> 
{
    use fantoccini::{ClientBuilder, Locator};
    
    let start = Instant::now();
    
    // Connect to WebDriver with headless Firefox
    use serde_json::json;
    
    let caps = json!({
        "moz:firefoxOptions": {
            "args": ["--headless"]
        }
    });
    
    let client = ClientBuilder::native()
        .capabilities(caps.as_object().unwrap().clone())
        .connect("http://localhost:4444")
        .await
        .map_err(|e| format!("Failed to connect to WebDriver: {}", e))?;

    // Navigate to our dev server
    client.goto(server_url).await
        .map_err(|e| format!("Failed to navigate to dev server: {}", e))?;
    
    // Find URL input and enter the test URL
    let url_input = client.find(Locator::Id("html-input")).await
        .map_err(|e| format!("Failed to find URL input: {}", e))?;
    
    url_input.clear().await
        .map_err(|e| format!("Failed to clear URL input: {}", e))?;
    
    url_input.send_keys(url).await
        .map_err(|e| format!("Failed to enter URL: {}", e))?;
    
    // Set format if not markdown (default)
    if format != "markdown" {
        // Click dropdown button
        let dropdown_button = client.find(Locator::Css(".dropdown-button")).await
            .map_err(|e| format!("Failed to find dropdown button: {}", e))?;
        
        dropdown_button.click().await
            .map_err(|e| format!("Failed to click dropdown: {}", e))?;
        
        // Click the desired format
        let format_selector = format!("button[data-value='{}']", format);
        let format_option = client.find(Locator::Css(&format_selector)).await
            .map_err(|e| format!("Failed to find format option: {}", e))?;
        
        format_option.click().await
            .map_err(|e| format!("Failed to select format: {}", e))?;
    }
    
    // Click parse button
    let parse_button = client.find(Locator::Css(".send-button")).await
        .map_err(|e| format!("Failed to find parse button: {}", e))?;
    
    parse_button.click().await
        .map_err(|e| format!("Failed to click parse button: {}", e))?;
    
    // Wait for result to appear
    client.wait().for_element(Locator::Css(".result-content")).await
        .map_err(|e| format!("Failed to find result content: {}", e))?;
    
    // Get the result text
    let result_element = client.find(Locator::Css(".result-content")).await
        .map_err(|e| format!("Failed to find result element: {}", e))?;
    
    let result_text = result_element.text().await
        .map_err(|e| format!("Failed to get result text: {}", e))?;
    
    let duration = start.elapsed();
    
    // Close the browser
    client.close().await.ok();
    
    Ok((result_text, duration))
}

/// Compare CLI and web outputs and generate a diff if they differ
pub fn compare_outputs(cli_output: &str, web_output: &str) -> Option<String> {
    if cli_output.trim() == web_output.trim() {
        return None; // Outputs match
    }
    
    let diff = TextDiff::from_lines(cli_output, web_output);
    let mut diff_output = String::new();
    
    diff_output.push_str("🔍 Output Diff (CLI vs Web):\n");
    diff_output.push_str("─".repeat(50).as_str());
    diff_output.push('\n');
    
    for change in diff.iter_all_changes() {
        let sign = match change.tag() {
            ChangeTag::Delete => "- ",
            ChangeTag::Insert => "+ ",
            ChangeTag::Equal => "  ",
        };
        diff_output.push_str(&format!("{}{}", sign, change));
    }
    
    Some(diff_output)
}

/// Print test results in a nice format
pub fn print_test_results(results: &[TestResult]) {
    println!("\n🧪 Integration Test Results");
    println!("═══════════════════════════════════════════════════════════════");
    
    let mut passed = 0;
    let mut failed = 0;
    
    for result in results {
        if result.success {
            println!("✅ {} - PASSED", result.name);
            println!("   CLI: {}ms | Web: {}ms", 
                result.cli_duration.as_millis(), 
                result.web_duration.as_millis());
            passed += 1;
        } else {
            println!("❌ {} - FAILED", result.name);
            if let Some(ref error) = result.error {
                println!("   Error: {}", error);
            }
            failed += 1;
        }
        println!();
    }
    
    println!("Summary: {} passed, {} failed", passed, failed);
    
    if failed > 0 {
        std::process::exit(1);
    }
}