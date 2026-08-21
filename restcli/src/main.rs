use clap::{Parser, ValueEnum};
use reqwest::{Client, Method, Response};
use std::error::Error;
use std::str::FromStr;
use std::time::Instant;
use colored::Colorize;

#[derive(Parser, Debug)]
#[command(
    name = "restcli",
    version = "0.1.0",
    about = "A simple REST client for the command line",
    long_about = "A simple REST client for the command line.\n\n\
                  Send HTTP requests and display responses with timing information,\n\
                  and JSON formatting.",
    after_help = "EXAMPLES:\n\
                  \x20 # GET request\n\
                  \x20 restcli https://api.example.com/users\n\n\
                  \x20 # POST with JSON\n\
                  \x20 restcli -m POST -H 'Content-Type: application/json' -d '{\"name\":\"John\"}' https://api.example.com/users\n\n\
                  \x20 # POST from file with auth\n\
                  \x20 restcli -m POST -H 'Authorization: Bearer token' -f body.json https://api.example.com/users\n\n\
                  \x20 # Verbose with timing\n\
                  \x20 restcli -v -t https://api.example.com/users\n\n\
                  \x20 # Save response to file\n\
                  \x20 restcli -o response.json https://api.example.com/users"
)]
struct Cli {
    /// The URL to make a request to
    #[arg(value_name = "URL")]
    url: String,

    /// HTTP method to use (GET, POST, PUT, DELETE, etc.)
    #[arg(
        short, 
        long, 
        default_value = "GET",
        value_name = "METHOD",
        value_parser = parse_method,
        help_heading = "Request Options"
    )]
    method: Method,

    /// Custom headers in the format "Key: Value" (can be used multiple times)
    #[arg(
        short = 'H', 
        long = "header", 
        value_name = "KEY:VALUE",
        help_heading = "Request Options"
    )]
    headers: Vec<String>,

    /// Data to send in the request body
    #[arg(
        short = 'd', 
        long = "data", 
        value_name = "DATA",
        conflicts_with = "file",
        help_heading = "Request Options"
    )]
    data: Option<String>,

    /// Read request body from a file
    #[arg(
        short = 'f', 
        long = "file", 
        value_name = "FILE",
        help_heading = "Request Options"
    )]
    file: Option<String>,

    /// Show detailed request/response information
    #[arg(
        short = 'v', 
        long = "verbose",
        help_heading = "Display Options"
    )]
    verbose: bool,

    /// Show response time
    #[arg(
        short = 't', 
        long = "time",
        help_heading = "Display Options"
    )]
    show_time: bool,

    /// Output format for response
    #[arg(
        long = "format", 
        value_enum, 
        default_value_t = OutputFormat::Auto,
        value_name = "FORMAT",
        help_heading = "Display Options"
    )]
    format: OutputFormat,

    /// Save response to a file
    #[arg(
        short = 'o', 
        long = "output", 
        value_name = "FILE",
        help_heading = "Display Options"
    )]
    output: Option<String>,
}

#[derive(Debug, Clone, ValueEnum)]
enum OutputFormat {
    /// Auto-detect (JSON if possible, otherwise raw)
    Auto,
    /// Force JSON output
    Json,
    /// Force raw text output
    Raw,
    /// Pretty print JSON with colors
    Pretty,
}

fn parse_method(s: &str) -> Result<Method, String> {
    Method::from_str(&s.to_uppercase())
        .map_err(|_| format!("Invalid HTTP method: '{s}'. Use GET, POST, PUT, DELETE, PATCH, etc."))
}

fn parse_header(header_str: &str) -> Result<(String, String), String> {
    header_str
        .split_once(':')
        .map(|(key, value)| (key.trim().to_string(), value.trim().to_string()))
        .filter(|(key, _)| !key.is_empty())
        .ok_or_else(|| format!("Invalid header format '{header_str}'. Expected 'Key: Value'"))
}

fn pretty_print_json(value: &serde_json::Value) -> String {
    serde_json::to_string_pretty(value)
        .unwrap_or_else(|_| value.to_string())
}

fn create_client() -> Result<Client, Box<dyn Error>> {
    Ok(Client::builder()
        .user_agent(format!("restcli/{}", env!("CARGO_PKG_VERSION")))
        .timeout(std::time::Duration::from_secs(30))
        .build()?)
}

fn get_body_data(cli: &Cli) -> Result<Option<String>, Box<dyn Error>> {
    if let Some(data) = &cli.data {
        return Ok(Some(data.clone()));
    }
    
    if let Some(file_path) = &cli.file {
        return match std::fs::read_to_string(file_path) {
            Ok(content) => {
                if cli.verbose {
                    println!("{} body from file: {}", "→".green(), file_path);
                }
                Ok(Some(content))
            }
            Err(e) => {
                eprintln!("{} Cannot read file '{}': {}", "Error:".red(), file_path, e);
                Err(Box::<dyn Error>::from(e))
            }
        };
    }
    
    Ok(None)
}

fn build_request(client: &Client, cli: &Cli) -> Result<reqwest::RequestBuilder, Box<dyn Error>> {
    let mut request = client.request(cli.method.clone(), &cli.url);
    
    // Add headers
    for header_str in &cli.headers {
        match parse_header(header_str) {
            Ok((key, value)) => {
                if cli.verbose {
                    println!("{} {}: {}", "→".green(), key.bold(), value);
                }
                request = request.header(key, value);
            }
            Err(e) => {
                eprintln!("{} {}", "Warning:".yellow(), e);
            }
        }
    }
    
    // Add body
    if let Some(data) = get_body_data(cli)? {
        if cli.verbose {
            println!("{} body: {}", "→".green(), data);
        }
        request = request.body(data);
    }
    
    Ok(request)
}

fn print_request_info(cli: &Cli) {
    println!("{} {}", cli.method.to_string().bold(), cli.url);
    
    if cli.verbose {
        println!("{} {} custom header(s)", "→".green(), cli.headers.len());
    }
}

fn print_response_info(response: &Response, response_time: std::time::Duration, show_time: bool, verbose: bool) {
    let status = response.status();
    let status_str = format!("{status}");
    let status_colored = match status.as_u16() {
        200..=299 => status_str.green(),
        400..=499 => status_str.yellow(),
        500..=599 => status_str.red(),
        _ => status_str.normal(),
    };

    println!("\n{} {}", "Status:".bold(), status_colored.bold());

    if show_time || verbose {
        println!("{} {:.2?}", "Time:".bold(), response_time);
    }

    if verbose {
        println!("\n{}", "Response Headers:".bold());
        for (name, value) in response.headers() {
            println!("  {}: {}", name.to_string().cyan(), value.to_str().unwrap_or("<binary>"));
        }
    }
}

fn format_body(body: &str, format: &OutputFormat) -> Result<String, Box<dyn Error>> {
    Ok(match format {
        OutputFormat::Raw => body.to_string(),
        OutputFormat::Json | OutputFormat::Auto | OutputFormat::Pretty => {
            match serde_json::from_str::<serde_json::Value>(body) {
                Ok(parsed) => {
                    if matches!(format, OutputFormat::Pretty) {
                        pretty_print_json(&parsed)
                    } else {
                        serde_json::to_string_pretty(&parsed)?
                    }
                }
                Err(_) => body.to_string(),
            }
        }
    })
}

fn process_output(body: &str, format: &OutputFormat, output_file: Option<&str>) -> Result<(), Box<dyn Error>> {
    let formatted = format_body(body, format)?;
    
    // Save to file if requested
    if let Some(file_path) = output_file {
        std::fs::write(file_path, &formatted)?;
        println!("{} Response saved to: {}", "✓".green(), file_path);
    }
    
    // Print to console
    println!("\n{formatted}");
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let start_time = Instant::now();

    // Create client and build request
    let client = create_client()?;
    let request = build_request(&client, &cli)?;

    // Print request info
    print_request_info(&cli);

    // Send request
    let response = request.send().await?;
    let response_time = start_time.elapsed();

    // Print response info
    print_response_info(&response, response_time, cli.show_time, cli.verbose);

    // Extract and process body
    let body = response.text().await?;
    process_output(&body, &cli.format, cli.output.as_deref())?;

    Ok(())
}