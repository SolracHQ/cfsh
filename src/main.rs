use std::io::Write;

mod config;
mod shell;
mod logger;

fn main() {

    // Get target shell from command line arguments
    let target: shell::Shell = std::env::args()
        .nth(1)
        .unwrap_or_else(|| {
            log_and_exit!("Usage: cfsh <shell>");
        })
        .parse::<shell::Shell>()
        .unwrap_or_else(|err| {
            log_and_exit!("{}", err);
        });

    // Load config
    let config = config::Config::load().unwrap_or_else(|err| {
        log_and_exit!("{}", err);
    });

    let mut stdout = std::io::stdout().lock();
    for alias in config.aliases() {
        if let Some(result) = alias.compile(target) {
            writeln!(stdout, "{}", result).unwrap();
        }
    }
    for envar in config.envars() {
        if let Some(result) = envar.compile(target) {
            writeln!(stdout, "{}", result).unwrap();
        }
    }
}
