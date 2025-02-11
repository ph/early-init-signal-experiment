use signal_hook::iterator::exfiltrator::WithOrigin;
use signal_hook::{
    consts::{SIGCONT, SIGHUP, SIGTSTP, SIGUSR1, SIGWINCH},
    iterator::SignalsInfo,
};
use std::process::ExitCode;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;

fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .event_format(
            tracing_subscriber::fmt::format()
                .with_file(true)
                .with_line_number(true),
        )
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    info!("starting!");
    let sigs = vec![SIGTSTP, SIGCONT, SIGWINCH, SIGHUP, SIGUSR1];
    let mut signals = SignalsInfo::<WithOrigin>::new(&sigs).expect("signal check");

    for event in &mut signals {
        info!(event=?event, "received signal");

        match event.signal {
            SIGHUP => info!("SIGHUP reload config"),
            SIGTSTP => return ExitCode::SUCCESS,
            _ => {}
        }
    }
    ExitCode::SUCCESS
}
