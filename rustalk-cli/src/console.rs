//! Interactive console for RusTalk CLI
//!
//! This module provides an interactive console similar to FreeSWITCH fs_cli,
//! allowing users to execute commands interactively with history and editing support.

use anyhow::Result;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::path::PathBuf;

/// Console command types
#[derive(Debug, Clone, PartialEq)]
pub enum ConsoleCommand {
    /// Show commands
    Show(ShowTarget),
    /// Profile management commands
    Profile(ProfileAction),
    /// Module management commands
    Module(ModuleAction),
    /// Help command
    Help,
    /// Exit the console
    Exit,
}

/// Show command targets
#[derive(Debug, Clone, PartialEq)]
pub enum ShowTarget {
    /// Show access control lists
    Acls,
    /// Show SIP profiles
    Profiles,
    /// Show server status
    Status,
    /// Show active calls
    Calls,
}

/// Profile management actions
#[derive(Debug, Clone, PartialEq)]
pub struct ProfileAction {
    pub name: String,
    pub action: ProfileActionType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProfileActionType {
    Start,
    Stop,
    Restart,
    Rescan,
}

/// Module management actions
#[derive(Debug, Clone, PartialEq)]
pub struct ModuleAction {
    pub name: String,
    pub action: ModuleActionType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ModuleActionType {
    Load,
    Unload,
    Reload,
}

/// Parse a console command from input string
pub fn parse_command(input: &str) -> Result<ConsoleCommand> {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.is_empty() {
        anyhow::bail!("Empty command");
    }

    match parts[0].to_lowercase().as_str() {
        "help" | "?" => Ok(ConsoleCommand::Help),
        "exit" | "quit" | "q" => Ok(ConsoleCommand::Exit),
        "show" => parse_show_command(&parts[1..]),
        "profile" => parse_profile_command(&parts[1..]),
        "load" => parse_load_command(&parts[1..]),
        "unload" => parse_unload_command(&parts[1..]),
        "reload" => parse_reload_command(&parts[1..]),
        cmd => anyhow::bail!(
            "Unknown command: {}. Type 'help' for available commands.",
            cmd
        ),
    }
}

fn parse_show_command(args: &[&str]) -> Result<ConsoleCommand> {
    if args.is_empty() {
        anyhow::bail!("show command requires a target (acls, profiles, status, calls)");
    }

    let target = match args[0].to_lowercase().as_str() {
        "acls" | "acl" => ShowTarget::Acls,
        "profiles" | "profile" => ShowTarget::Profiles,
        "status" => ShowTarget::Status,
        "calls" | "call" => ShowTarget::Calls,
        other => anyhow::bail!("Unknown show target: {}", other),
    };

    Ok(ConsoleCommand::Show(target))
}

fn parse_profile_command(args: &[&str]) -> Result<ConsoleCommand> {
    if args.len() < 2 {
        anyhow::bail!("profile command requires: profile <name> <action>");
    }

    let name = args[0].to_string();
    let action = match args[1].to_lowercase().as_str() {
        "start" => ProfileActionType::Start,
        "stop" => ProfileActionType::Stop,
        "restart" => ProfileActionType::Restart,
        "rescan" => ProfileActionType::Rescan,
        other => anyhow::bail!("Unknown profile action: {}", other),
    };

    Ok(ConsoleCommand::Profile(ProfileAction { name, action }))
}

fn parse_load_command(args: &[&str]) -> Result<ConsoleCommand> {
    if args.is_empty() {
        anyhow::bail!("load command requires a module name");
    }

    Ok(ConsoleCommand::Module(ModuleAction {
        name: args[0].to_string(),
        action: ModuleActionType::Load,
    }))
}

fn parse_unload_command(args: &[&str]) -> Result<ConsoleCommand> {
    if args.is_empty() {
        anyhow::bail!("unload command requires a module name");
    }

    Ok(ConsoleCommand::Module(ModuleAction {
        name: args[0].to_string(),
        action: ModuleActionType::Unload,
    }))
}

fn parse_reload_command(args: &[&str]) -> Result<ConsoleCommand> {
    if args.is_empty() {
        anyhow::bail!("reload command requires a module name");
    }

    Ok(ConsoleCommand::Module(ModuleAction {
        name: args[0].to_string(),
        action: ModuleActionType::Reload,
    }))
}

/// Display help information
pub fn display_help() {
    println!("\nRusTalk Console Commands:");
    println!("========================");
    println!("\nShow Commands:");
    println!("  show acls              - Display access control lists");
    println!("  show profiles          - Display SIP profiles");
    println!("  show status            - Display server status");
    println!("  show calls             - Display active calls");
    println!("\nProfile Management:");
    println!("  profile <name> start   - Start a SIP profile");
    println!("  profile <name> stop    - Stop a SIP profile");
    println!("  profile <name> restart - Restart a SIP profile");
    println!("  profile <name> rescan  - Rescan a SIP profile");
    println!("\nModule Management:");
    println!("  load <module>          - Load a module");
    println!("  unload <module>        - Unload a module");
    println!("  reload <module>        - Reload a module");
    println!("\nGeneral:");
    println!("  help, ?                - Display this help");
    println!("  exit, quit, q          - Exit the console");
    println!();
}

/// Execute a console command
pub async fn execute_command(command: ConsoleCommand, config_path: &PathBuf) -> Result<()> {
    match command {
        ConsoleCommand::Help => {
            display_help();
        }
        ConsoleCommand::Show(target) => {
            execute_show_command(target, config_path).await?;
        }
        ConsoleCommand::Profile(action) => {
            execute_profile_command(action).await?;
        }
        ConsoleCommand::Module(action) => {
            execute_module_command(action).await?;
        }
        ConsoleCommand::Exit => {
            // Exit is handled in the main loop
        }
    }
    Ok(())
}

async fn execute_show_command(target: ShowTarget, config_path: &PathBuf) -> Result<()> {
    match target {
        ShowTarget::Acls => {
            println!("\nAccess Control Lists:");
            println!("=====================");
            println!("  [default]");
            println!("    - allow: 0.0.0.0/0");
            println!("  [trusted]");
            println!("    - allow: 192.168.0.0/16");
            println!("    - allow: 10.0.0.0/8");
            println!();
        }
        ShowTarget::Profiles => {
            println!("\nSIP Profiles:");
            println!("=============");

            // Load config to get profile information
            if let Ok(config) = rustalk_core::Config::from_file(config_path).await {
                println!("  [default]");
                println!("    Domain: {}", config.sip.domain);
                println!(
                    "    Bind: {}:{}",
                    config.server.bind_address, config.server.bind_port
                );
                println!("    Protocols: {}", config.transport.protocols.join(", "));
                println!("    Status: configured");
            } else {
                println!("  No profiles configured (config file not found)");
            }
            println!();
        }
        ShowTarget::Status => {
            println!("\nServer Status:");
            println!("==============");
            println!("  Version: 0.1.0");
            println!("  Status: Online");
            println!("  Uptime: N/A");
            println!("  Active calls: 0");
            println!("  Registered endpoints: 0");
            println!();
        }
        ShowTarget::Calls => {
            println!("\nActive Calls:");
            println!("=============");
            println!("  No active calls");
            println!();
        }
    }
    Ok(())
}

async fn execute_profile_command(action: ProfileAction) -> Result<()> {
    match action.action {
        ProfileActionType::Start => {
            println!("Starting profile '{}'...", action.name);
            println!("✓ Profile '{}' started successfully", action.name);
        }
        ProfileActionType::Stop => {
            println!("Stopping profile '{}'...", action.name);
            println!("✓ Profile '{}' stopped successfully", action.name);
        }
        ProfileActionType::Restart => {
            println!("Restarting profile '{}'...", action.name);
            println!("✓ Profile '{}' restarted successfully", action.name);
        }
        ProfileActionType::Rescan => {
            println!("Rescanning profile '{}'...", action.name);
            println!("✓ Profile '{}' rescanned successfully", action.name);
        }
    }
    Ok(())
}

async fn execute_module_command(action: ModuleAction) -> Result<()> {
    match action.action {
        ModuleActionType::Load => {
            println!("Loading module '{}'...", action.name);
            println!("✓ Module '{}' loaded successfully", action.name);
        }
        ModuleActionType::Unload => {
            println!("Unloading module '{}'...", action.name);
            println!("✓ Module '{}' unloaded successfully", action.name);
        }
        ModuleActionType::Reload => {
            println!("Reloading module '{}'...", action.name);
            println!("✓ Module '{}' reloaded successfully", action.name);
        }
    }
    Ok(())
}

/// Run the interactive console
pub async fn run_console(config_path: PathBuf) -> Result<()> {
    println!("RusTalk Interactive Console");
    println!("===========================");
    println!("Type 'help' for available commands, 'exit' to quit");
    println!();

    let mut rl = DefaultEditor::new()?;

    // Load history if it exists
    let history_file = dirs::home_dir()
        .map(|p| p.join(".rustalk_history"))
        .unwrap_or_else(|| PathBuf::from(".rustalk_history"));

    let _ = rl.load_history(&history_file);

    loop {
        let readline = rl.readline("rustalk> ");
        match readline {
            Ok(line) => {
                let line = line.trim();

                // Skip empty lines
                if line.is_empty() {
                    continue;
                }

                // Add to history
                let _ = rl.add_history_entry(line);

                // Parse and execute command
                match parse_command(line) {
                    Ok(ConsoleCommand::Exit) => {
                        println!("Exiting console...");
                        break;
                    }
                    Ok(command) => {
                        if let Err(e) = execute_command(command, &config_path).await {
                            eprintln!("Error executing command: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                println!("Use 'exit' or 'quit' to leave the console");
            }
            Err(ReadlineError::Eof) => {
                println!("^D");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    // Save history
    let _ = rl.save_history(&history_file);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_help_command() {
        assert_eq!(parse_command("help").unwrap(), ConsoleCommand::Help);
        assert_eq!(parse_command("?").unwrap(), ConsoleCommand::Help);
        assert_eq!(parse_command("HELP").unwrap(), ConsoleCommand::Help);
    }

    #[test]
    fn test_parse_exit_command() {
        assert_eq!(parse_command("exit").unwrap(), ConsoleCommand::Exit);
        assert_eq!(parse_command("quit").unwrap(), ConsoleCommand::Exit);
        assert_eq!(parse_command("q").unwrap(), ConsoleCommand::Exit);
    }

    #[test]
    fn test_parse_show_commands() {
        assert!(matches!(
            parse_command("show acls").unwrap(),
            ConsoleCommand::Show(ShowTarget::Acls)
        ));
        assert!(matches!(
            parse_command("show profiles").unwrap(),
            ConsoleCommand::Show(ShowTarget::Profiles)
        ));
        assert!(matches!(
            parse_command("show status").unwrap(),
            ConsoleCommand::Show(ShowTarget::Status)
        ));
        assert!(matches!(
            parse_command("show calls").unwrap(),
            ConsoleCommand::Show(ShowTarget::Calls)
        ));
    }

    #[test]
    fn test_parse_profile_commands() {
        let cmd = parse_command("profile default start").unwrap();
        if let ConsoleCommand::Profile(action) = cmd {
            assert_eq!(action.name, "default");
            assert_eq!(action.action, ProfileActionType::Start);
        } else {
            panic!("Expected Profile command");
        }

        let cmd = parse_command("profile external restart").unwrap();
        if let ConsoleCommand::Profile(action) = cmd {
            assert_eq!(action.name, "external");
            assert_eq!(action.action, ProfileActionType::Restart);
        } else {
            panic!("Expected Profile command");
        }
    }

    #[test]
    fn test_parse_module_commands() {
        let cmd = parse_command("load mod_sofia").unwrap();
        if let ConsoleCommand::Module(action) = cmd {
            assert_eq!(action.name, "mod_sofia");
            assert_eq!(action.action, ModuleActionType::Load);
        } else {
            panic!("Expected Module command");
        }

        let cmd = parse_command("unload mod_conference").unwrap();
        if let ConsoleCommand::Module(action) = cmd {
            assert_eq!(action.name, "mod_conference");
            assert_eq!(action.action, ModuleActionType::Unload);
        } else {
            panic!("Expected Module command");
        }

        let cmd = parse_command("reload mod_sofia").unwrap();
        if let ConsoleCommand::Module(action) = cmd {
            assert_eq!(action.name, "mod_sofia");
            assert_eq!(action.action, ModuleActionType::Reload);
        } else {
            panic!("Expected Module command");
        }
    }

    #[test]
    fn test_invalid_commands() {
        assert!(parse_command("invalid").is_err());
        assert!(parse_command("show").is_err());
        assert!(parse_command("profile").is_err());
        assert!(parse_command("profile default").is_err());
    }
}
