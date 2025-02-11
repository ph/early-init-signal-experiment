use signal_hook::consts::{
    SIGABRT, SIGALRM, SIGBUS, SIGCHLD, SIGFPE, SIGILL, SIGINT, SIGKILL, SIGPIPE, SIGPROF, SIGQUIT,
    SIGSEGV, SIGSTOP, SIGSYS, SIGTERM, SIGTRAP, SIGTTIN, SIGTTOU, SIGURG, SIGUSR2, SIGVTALRM,
    SIGWINCH, SIGXCPU, SIGXFSZ,
};
use signal_hook::iterator::exfiltrator::WithOrigin;
use signal_hook::{
    consts::{SIGCONT, SIGHUP, SIGTSTP, SIGUSR1},
    iterator::SignalsInfo,
};
use std::process::{Command, ExitCode};
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

    // let output = Command::new("/app/oops.sh").spawn().unwrap();
    // let pid = output.id();
    // info!(pid=?pid, "spawn a background sh script");

    info!("starting!");
    // SIGSTOP,
    // SIGSEGV,
    // SIGKILL,
    // SIGFPE,
    // SIGILL,
    let sigs = vec![
        SIGHUP, SIGINT, SIGQUIT, SIGTRAP, SIGABRT, SIGBUS, SIGUSR1, SIGUSR2, SIGPIPE, SIGALRM,
        SIGTERM, SIGCHLD, SIGCONT, SIGTSTP, SIGTTIN, SIGTTOU, SIGURG, SIGXCPU, SIGXFSZ, SIGVTALRM,
        SIGPROF, SIGWINCH, SIGSYS,
    ];

    let mut signals = SignalsInfo::<WithOrigin>::new(&sigs).expect("signal check");

    for event in &mut signals {
        info!(event=?event, "received signal");
        match event.signal {
            SIGCHLD => info!("SIGCHLD received"),
            SIGHUP => info!("SIGHUP reload config"),
            SIGCONT => info!("SIGCONT received"),
            SIGTSTP => return ExitCode::SUCCESS,
            SIGTERM => info!("SIGTERM received"),
            _ => {}
        }
    }
    ExitCode::SUCCESS
}
