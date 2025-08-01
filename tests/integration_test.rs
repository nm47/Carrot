mod integration;

use integration::*;
use std::time::Duration;

#[tokio::test]
async fn test_cli_vs_web_output() {
    // Load test cases
    let test_cases = load_test_cases().expect("Failed to load test cases");
    let timeout = Duration::from_millis(test_cases.config.timeout_ms);
    
    println!("🚀 Starting integration tests...");
    println!("Server URL: {}", test_cases.config.dev_server_url);
    println!("Timeout: {}ms", test_cases.config.timeout_ms);
    
    // Start WebDriver - REQUIRED for integration tests
    let webdriver = start_webdriver().expect("WebDriver is required for integration tests. Please install geckodriver and ensure it's in your PATH.");
    println!();
    
    let mut results = Vec::new();
    
    for test_case in &test_cases.test_cases {
        println!("🧪 Testing: {}", test_case.name);
        
        // Run CLI test
        let cli_result = run_cli(&test_case.url, &test_case.format, timeout);
        
        // Run web test
        let web_result = run_web_test(
            &test_case.url, 
            &test_case.format, 
            &test_cases.config.dev_server_url,
            timeout
        ).await;
        
        let test_result = match (cli_result, web_result) {
            (Ok((cli_output, cli_duration)), Ok((web_output, web_duration))) => {
                let diff = compare_outputs(&cli_output, &web_output);
                
                if diff.is_none() {
                    TestResult {
                        name: test_case.name.clone(),
                        success: true,
                        cli_duration,
                        web_duration,
                        error: None,
                    }
                } else {
                    TestResult {
                        name: test_case.name.clone(),
                        success: false,
                        cli_duration,
                        web_duration,
                        error: diff,
                    }
                }
            }
            (Err(cli_error), _) => {
                TestResult {
                    name: test_case.name.clone(),
                    success: false,
                    cli_duration: Duration::from_secs(0),
                    web_duration: Duration::from_secs(0),
                    error: Some(format!("CLI failed: {}", cli_error)),
                }
            }
            (_, Err(web_error)) => {
                TestResult {
                    name: test_case.name.clone(),
                    success: false,
                    cli_duration: Duration::from_secs(0),
                    web_duration: Duration::from_secs(0),
                    error: Some(format!("Web test failed: {}", web_error)),
                }
            }
        };
        
        results.push(test_result);
    }
    
    // Clean up WebDriver
    stop_webdriver(webdriver);
    
    print_test_results(&results);
}

#[test]
fn test_cli_only() {
    // Test just the CLI without web comparison (useful for debugging)
    let test_cases = load_test_cases().expect("Failed to load test cases");
    let timeout = Duration::from_millis(test_cases.config.timeout_ms);
    
    println!("🔧 Testing CLI only...");
    
    for test_case in &test_cases.test_cases {
        println!("Testing CLI: {}", test_case.name);
        
        match run_cli(&test_case.url, &test_case.format, timeout) {
            Ok((output, duration)) => {
                println!("✅ Success ({}ms)", duration.as_millis());
                println!("📝 Output length: {} characters", output.len());
                
                // Show a sample of the output
                let sample = if output.len() > 200 {
                    format!("{}...", &output[..200])
                } else {
                    output
                };
                println!("🔍 Sample: {}", sample.replace('\n', "\\n"));
            }
            Err(error) => {
                println!("❌ Failed: {}", error);
            }
        }
        println!();
    }
}