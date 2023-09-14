fn main() {
    tracing_subscriber::fmt::init();
    s2protocol::cli::process_cli_request().unwrap();
}
