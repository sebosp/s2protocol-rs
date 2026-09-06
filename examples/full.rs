#[path = "cli/mod.rs"]
mod cli;

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

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    reset_sigpipe();
    cli::process_cli_request().await.unwrap();
}
