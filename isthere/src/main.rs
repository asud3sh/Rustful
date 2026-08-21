use colored::Colorize;
use std::env;
use std::net::{IpAddr, TcpStream, ToSocketAddrs};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

struct PingStatus {
    successful: u32,
    failed: u32,
    total_time: Duration,
    min_time: Duration,
    max_time: Duration,
}

impl PingStatus {
    const fn new() -> Self {
        Self {
            successful: 0,
            failed: 0,
            total_time: Duration::ZERO,
            min_time: Duration::MAX,
            max_time: Duration::ZERO,
        }
    }

    fn record_success(&mut self, elapsed: Duration) {
        self.successful += 1;
        self.total_time += elapsed;
        if elapsed < self.min_time {
            self.min_time = elapsed;
        }
        if elapsed > self.max_time {
            self.max_time = elapsed;
        }
    }

    const fn record_failure(&mut self) {
        self.failed += 1;
    }

    fn print_summary(&self, host: &str) {
        let total_packets = self.successful + self.failed;
        if total_packets == 0 {
            println!("\nNo packets sent.");
            return;
        }

        println!("\n{}", "-".repeat(45).dimmed());
        println!("{} ping statistics", host.bold());
        println!("{}", "-".repeat(45).dimmed());

        let loss_percentage = if total_packets > 0 {
            (f64::from(self.failed) / f64::from(total_packets)) * 100.0
        } else {
            0.0
        };
        println!(
            "Transmitted {total_packets} packets, reveived {}, {} {:.1}%",
            self.successful.to_string().green(),
            "packet_lost".red(),
            loss_percentage
        );

        if self.successful > 0 {
            let average_duration = self.total_time / self.successful;
            println!(
                "rtt {}/{}/{} = {:.2?}/{:.2?}/{:.2?}",
                "min".green(),
                "avg".yellow(),
                "max".red(),
                self.min_time,
                average_duration,
                self.max_time
            );
        }
    }
}

fn print_usage() {
    println!("{}", "Usage:".bold().underline());
    println!("  isthere <hostname|ip> [port] [count]");
    println!("  isthere <hostname:port> [count]");
    println!("\n{}", "Examples:".bold().underline());
    println!("  isthere google.com           # Domain default: port 443, 5 pings");
    println!("  isthere 1.1.1.1              # IP default: port 80, 5 pings");
    println!("  isthere google.com:80        # Inline port 80, 5 pings");
    println!("  isthere github.com 8080 10    # Custom port & count");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // Parse arguments with fallbacks
    let Some(raw_target) = args.get(1) else {
        print_usage();
        return;
    };
    // let port: u16 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(80);
    // let count: u32 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(5);

    // Extract host and inline port if present
    let (host, inline_port) = if let Some((h, p)) = raw_target.rsplit_once(':') {
        (h, p.parse::<u16>().ok())
    } else {
        (raw_target.as_str(), None)
    };

    let is_ip = host.parse::<IpAddr>().is_ok();
    let default_port = if is_ip { 80 } else { 443 };

    let (port, count) = inline_port.map_or_else(
        || {
            // Syntax: isthere domain [port] [count]
            let port = args
                .get(2)
                .and_then(|s| s.parse().ok())
                .unwrap_or(default_port);
            let count = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(5);
            (port, count)
        },
        |p| {
            // Syntax: isthere domain:443 [count]
            let count = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(5);
            (p, count)
        },
    );

    // Set up Ctrl+C Signal Handler
    let running = Arc::new(AtomicBool::new(true));
    let r = Arc::clone(&running);

    ctrlc::set_handler(move || {
        println!("\n{}", "Received interrupt, shutting down...".yellow());
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl+C handler");

    // Print Fancy Header Banner
    println!("{}", "╔══════════════════════════════════════════╗".cyan());
    println!("{}", "║    isthere: A simple TCP Ping Tool v0.1  ║".cyan());
    println!("{}", "╚══════════════════════════════════════════╝".cyan());
    println!();
    println!(
        "Testing {}:{} with {} attempts...\n",
        host.cyan(),
        port.to_string().cyan(),
        count
    );

    // Resolve domain name
    let target = format!("{host}:{port}");
    let Ok(mut address) = target.to_socket_addrs() else {
        println!("{} Failed to resolve host: {host}", "✗".red());
        return;
    };

    let Some(addr) = address.next() else {
        println!("{} No IP addresses found for {host}", "✗".red());
        return;
    };

    println!(
        "Pinging {} [{}] on port {}...\n",
        host.cyan().bold(),
        addr.ip().to_string().yellow(),
        port.to_string().cyan()
    );

    let mut stats = PingStatus::new();

    // Ping loop
    for i in 1..=count {
        // Exit loop early if user hits Ctrl+C
        if !running.load(Ordering::SeqCst) {
            break;
        }

        let start: Instant = Instant::now();
        if let Ok(_stream) = TcpStream::connect_timeout(&addr, Duration::from_secs(2)) {
            let elapsed = start.elapsed();
            stats.record_success(elapsed);

            // Dynamic latency color coding
            let time_str = format!("{elapsed:.2?}");
            let styled_time = if elapsed < Duration::from_millis(50) {
                time_str.green()
            } else if elapsed < Duration::from_millis(150) {
                time_str.yellow()
            } else {
                time_str.red()
            };

            println!(
                "[{}/{}] {} Reply from {}: time={}",
                i,
                count,
                "✓".green(),
                addr.ip(),
                styled_time
            );
        } else {
            stats.record_failure();
            println!("[{}/{}] {} Connection timed out", i, count, "✗".red());
        }

        if i < count && running.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_millis(500));
        }
    }
    // Print final summary
    stats.print_summary(host);
}
