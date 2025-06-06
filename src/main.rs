use std::env;
use std::fs::{OpenOptions, read_to_string};
use std::io::{Write, BufRead, BufReader};
use std::process::{Command, Stdio};
use x11rb::connection::Connection;
use x11rb::rust_connection::RustConnection;
use std::path::PathBuf;

fn get_config_path() -> PathBuf {
    let mut config_dir = dirs::config_dir().expect("Could not find config directory");
    config_dir.push("wire");
    std::fs::create_dir_all(&config_dir).expect("Failed to create config dir");
    config_dir.push("startup.conf");
    config_dir
}

fn add_program_to_startup(prog: &str) -> std::io::Result<()> {
    let config_path = get_config_path();
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(config_path)?;
    writeln!(file, "{}", prog)?;
    Ok(())
}

fn run_startup_programs() -> std::io::Result<()> {
    let config_path = get_config_path();
    if !config_path.exists() {
        println!("No startup.conf found. Create one or add programs with `wire --add PROGRAM`.");
        return Ok(());
    }
    let file = OpenOptions::new().read(true).open(config_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let cmd = line?;
        if cmd.trim().is_empty() {
            continue;
        }
        println!("Starting: {}", cmd);
        // Spawn program detached
        let _child = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // Simple CLI handling
    if args.len() > 1 {
        match args[1].as_str() {
            "--add" => {
                if args.len() < 3 {
                    eprintln!("Usage: wire --add PROGRAM");
                    return Ok(());
                }
                add_program_to_startup(&args[2])?;
                println!("Added '{}' to startup.conf", args[2]);
                return Ok(());
            }
            "--init" => {
                let config_path = get_config_path();
                if config_path.exists() {
                    println!("startup.conf already exists at {}", config_path.display());
                } else {
                    std::fs::write(&config_path, "# Add programs here, one per line\n")?;
                    println!("Created new startup.conf at {}", config_path.display());
                }
                return Ok(());
            }
            _ => {
                eprintln!("Unknown argument: {}", args[1]);
                eprintln!("Usage:\n  wire --add PROGRAM\n  wire --init");
                return Ok(());
            }
        }
    }

    // No args: run startup programs and launch Wire core

    run_startup_programs()?;

    // Connect to the X server
    let (conn, screen_num): (RustConnection, usize) = RustConnection::connect(None)?;

    // Get the screen
    let screen = &conn.setup().roots[screen_num];

    println!("Connected to X server. Root window ID: {}", screen.root);

    // Event loop
    loop {
        let event = conn.wait_for_event()?;
        println!("Got event: {:?}", event);

        // Flush after each event to keep things smooth
        conn.flush()?;
    }
}
