use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[cfg(not(feature = "upgrade"))]
pub fn upgrade_gg(_force: bool) {
    let mut colored_stdout = StandardStream::stdout(ColorChoice::Always);
    colored_stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_italic(true))
        .expect("Failed to set color");

    println!("\n┌────────────────────────────────────────────┐");
    println!("│ Upgrade feature is not enabled.            │");
    println!("│ Please recompile with `upgrade` feature    │");
    println!("│ enabled to use this feature:               │");
    println!("│                                            │");
    println!("│ cargo install grip-grab --features=upgrade │");
    println!("└────────────────────────────────────────────┘\n");

    colored_stdout.reset().expect("Failed to reset color");
}

#[cfg(feature = "upgrade")]
pub fn upgrade_gg(force: bool) {
    use std::{
        io::{BufRead, BufReader},
        process::Command,
        thread::{self},
    };

    let mut colored_stdout = StandardStream::stdout(ColorChoice::Always);
    colored_stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Green)).set_italic(true))
        .expect("Failed to set color");

    println!("\n┌─────────────────────────────────────────┐");
    println!("│ Upgrading `gg` to its latest version... │");
    println!("└─────────────────────────────────────────┘\n");

    let mut command = Command::new("cargo");
    command.arg("install").arg("grip-grab");
    if force {
        command.arg("--force");
    }
    let mut output = command
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn cargo install command");

    colored_stdout
        .set_color(
            ColorSpec::new()
                .set_fg(Some(Color::Rgb(128, 128, 128)))
                .set_dimmed(true)
                .set_italic(true),
        )
        .expect("Failed to set color");

    println!("This may take a few moments..");

    let stdout = output.stdout.take().expect("Failed to capture stdout");
    let stderr = output.stderr.take().expect("Failed to capture stderr");

    let stdout_thread = thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            match line {
                Ok(line) => println!("{}", line),
                Err(e) => eprintln!("Error reading stdout: {}", e),
            }
        }
    });

    let stderr_thread = thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            match line {
                Ok(line) => eprintln!("{}", line),
                Err(e) => eprintln!("Error reading stderr: {}", e),
            }
        }
    });

    stdout_thread.join().expect("Failed to join stdout thread");
    stderr_thread.join().expect("Failed to join stderr thread");

    let status = output.wait().expect("Failed to wait on output");

    if status.success() {
        colored_stdout
            .set_color(
                ColorSpec::new()
                    .set_fg(Some(Color::Green))
                    .set_italic(true)
                    .set_dimmed(false),
            )
            .expect("Failed to set color");

        println!("\nYou're all set!");
    } else {
        colored_stdout
            .set_color(
                ColorSpec::new()
                    .set_fg(Some(Color::Red))
                    .set_italic(true)
                    .set_dimmed(false),
            )
            .expect("Failed to set color");

        eprintln!("\nFailed to upgrade crate. See errors above.");
    }
    colored_stdout.reset().expect("Failed to reset color");
}
