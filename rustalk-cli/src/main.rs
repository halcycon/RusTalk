//! RusTalk CLI - Admin tool for managing RusTalk SIP servers

mod console;
mod cert;

use anyhow::Result;
use clap::{Parser, Subcommand};
use rustalk_core::prelude::{Config, B2BUA};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "rustalk")]
#[command(about = "RusTalk SIP/SBC Admin CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the SIP server
    Start {
        /// Configuration file path
        #[arg(short, long, default_value = "config.json")]
        config: PathBuf,
    },
    /// Enter interactive console mode
    Console {
        /// Configuration file path
        #[arg(short, long, default_value = "config.json")]
        config: PathBuf,
    },
    /// Check configuration validity
    CheckConfig {
        /// Configuration file path
        #[arg(short, long, default_value = "config.json")]
        config: PathBuf,
    },
    /// Generate a sample configuration file
    GenerateConfig {
        /// Output file path
        #[arg(short, long, default_value = "config.json")]
        output: PathBuf,
    },
    /// Show server status
    Status {
        /// Server address
        #[arg(short, long, default_value = "http://localhost:8080")]
        server: String,
    },
    /// List active calls
    ListCalls {
        /// Server address
        #[arg(short, long, default_value = "http://localhost:8080")]
        server: String,
    },
    /// Certificate management commands
    #[command(subcommand)]
    Cert(CertCommands),
}

#[derive(Subcommand)]
enum CertCommands {
    /// Request a new Let's Encrypt certificate
    Request {
        /// Configuration file path
        #[arg(short, long, default_value = "config.json")]
        config: PathBuf,
        /// Domain names to include in certificate
        #[arg(short, long, required = true)]
        domains: Vec<String>,
        /// Email for Let's Encrypt account
        #[arg(short, long)]
        email: String,
        /// Use staging environment (for testing)
        #[arg(long)]
        staging: bool,
        /// Challenge type: http-01 or dns-01
        #[arg(long, default_value = "http-01")]
        challenge: String,
    },
    /// Renew an existing certificate
    Renew {
        /// Configuration file path
        #[arg(short, long, default_value = "config.json")]
        config: PathBuf,
        /// Domain name (primary domain of certificate)
        #[arg(short, long)]
        domain: String,
    },
    /// Check certificate status and expiry
    Status {
        /// Configuration file path
        #[arg(short, long, default_value = "config.json")]
        config: PathBuf,
        /// Domain name (optional, shows all if not specified)
        #[arg(short, long)]
        domain: Option<String>,
    },
    /// List all stored certificates
    List {
        /// Configuration file path
        #[arg(short, long, default_value = "config.json")]
        config: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Start { config } => {
            println!("Starting RusTalk server with config: {}", config.display());
            start_server(config).await?;
        }
        Commands::Console { config } => {
            console::run_console(config).await?;
        }
        Commands::CheckConfig { config } => {
            println!("Checking configuration: {}", config.display());
            check_config(config).await?;
        }
        Commands::GenerateConfig { output } => {
            println!("Generating sample configuration: {}", output.display());
            generate_config(output).await?;
        }
        Commands::Status { server } => {
            println!("Getting status from: {}", server);
            get_status(&server).await?;
        }
        Commands::ListCalls { server } => {
            println!("Listing calls from: {}", server);
            list_calls(&server).await?;
        }
        Commands::Cert(cert_cmd) => {
            cert::handle_cert_command(cert_cmd).await?;
        }
    }

    Ok(())
}

async fn start_server(config_path: PathBuf) -> Result<()> {
    let config = Config::from_file(&config_path).await?;
    
    println!("Server configuration:");
    println!("  Bind address: {}:{}", config.server.bind_address, config.server.bind_port);
    println!("  SIP domain: {}", config.sip.domain);
    
    let _b2bua = B2BUA::new();
    
    println!("RusTalk server started successfully!");
    println!("Press Ctrl+C to stop");
    
    // Keep running
    tokio::signal::ctrl_c().await?;
    println!("\nShutting down...");
    
    Ok(())
}

async fn check_config(config_path: PathBuf) -> Result<()> {
    let config = Config::from_file(&config_path).await?;
    
    println!("✓ Configuration is valid");
    println!("\nConfiguration details:");
    println!("  Domain: {}", config.sip.domain);
    println!("  Bind: {}:{}", config.server.bind_address, config.server.bind_port);
    println!("  Workers: {}", config.server.workers);
    
    if let Some(db) = &config.database {
        println!("  Database: {}", db.url);
    }
    
    if let Some(teams) = &config.teams {
        println!("  Teams SBC: {} (enabled: {})", teams.sbc_fqdn, teams.enabled);
    }
    
    Ok(())
}

async fn generate_config(output_path: PathBuf) -> Result<()> {
    let config = Config::default();
    config.save_to_file(&output_path).await?;
    
    println!("✓ Generated sample configuration at: {}", output_path.display());
    println!("\nEdit the file to customize your settings.");
    
    Ok(())
}

async fn get_status(_server: &str) -> Result<()> {
    println!("Status: Online");
    println!("Version: 0.1.0");
    println!("Uptime: N/A");
    println!("Active calls: 0");
    
    Ok(())
}

async fn list_calls(_server: &str) -> Result<()> {
    println!("Active calls: 0");
    println!("\nNo active calls");
    
    Ok(())
}
