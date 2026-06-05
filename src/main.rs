#[cfg(unix)]
fn reset_sigpipe() {
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
}

#[cfg(not(unix))]
fn reset_sigpipe() {
    // no-op
}

fn main() {
    reset_sigpipe();
    #[cfg(feature = "dep_cli")]
    s2protocol::cli::process_cli_request().unwrap();
}
