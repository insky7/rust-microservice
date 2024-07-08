use std::net::{Ipv4Addr, SocketAddrV4};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// IP address for this service to listen on
    #[arg(short = 'H', long, env, default_value = "0.0.0.0")]
    host:        Ipv4Addr,
    /// Port for this service to listen on
    #[arg(short = 'p', long, env, default_value_t = 5000)]
    port:        u16,
    /// What environment the current configuration is
    /// running on; development or production for now
    #[arg(long, env, default_value = "dev")]
    env:         String,
    /// Service-wide logging filter. See `tracing` and
    /// `tracing-subscriber` and how RUST_LOG environment
    /// variable works to learn more.
    #[arg(long, env, default_value = "info,backend=trace")]
    log_filter:  String,
    /// Name of the local environment file to use. This can
    /// be updated/changed for running different
    /// configurations locally or potentially on live.
    #[arg(long, env, default_value = ".env")]
    dotenv_file: String,
}

// A proc macro provided by `tokio` to decorate the `main`
// function to initialize the runtime instance
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Check if `ENV`` environment variable exists and is set to
    // 'dev', but default the result to dev anyways to ensure
    // production isn't flagged
    if std::env::var("ENV").map(|env| env == "dev").unwrap_or(true) {
        // Load `DOTENV_FILE` environment variable if it exists to
        // override the default path, otherwise just load the
        // default .env file that should exist.
        let dotenv_path = std::env::var("DOTENV_FILE").unwrap_or(String::from(".env"));
        // Load dotenv_path file
        dotenvy::from_path(dotenv_path).expect("should have loaded a .env type file");
    }

    // Parse CLI args
    let args = Args::parse();

    // Setup logging
    {
        // Subscriber configuration for our logging configuration
        let subscriber = tracing_subscriber::fmt()
            .with_env_filter(args.log_filter) // EnvFilter to use for our logs
            .with_file(true) // Display source file in logs
            .with_line_number(true) // Display source file line number in logs
            .finish();

        // Set our logging configuration using our custom subscriber
        tracing::subscriber::set_global_default(subscriber)
            .expect("should have set the global default tracing subscriber");
    }

    // Serve axum configuration
    {
        // Build Router to house all endpoints, middleware, and
        // server state
        let router = axum::Router::new();

        // Build Server listening address
        let listener =
            tokio::net::TcpListener::bind(SocketAddrV4::new(args.host, args.port)).await?;

        // Run the server with the corresponding listening address
        // and router configuration
        axum::serve(listener, router).await?;
    }

    // In theory this point in the application will never be
    // reached unless axum::serve() fails/panics at any point
    // during its runtime
    Ok(())
}
