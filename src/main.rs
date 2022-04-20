use time::UtcOffset;
use tracing::info;
use tracing_subscriber::fmt::time::OffsetTime;
use time::macros::format_description;

use clap::Parser;
use htom::retrieve::retrieve;

/// Retrieve arguments from the command line
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// url of retrieve url.
    #[clap(short, long)]
    url: String,

    /// Name of write file path.
    #[clap(short, long, default_value = "ok.md")]
    name: String,
}


#[tokio::main]
async fn main() {
    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"),
    );

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "htom=debug".to_string()),
        ))
        .with_timer(local_time)
        .init();

    info!("html to markdown.");

    let args = Args::parse();
    info!("{args:?}");

    retrieve(&args.url, &args.name).await.expect("retrieve failed");
}
