
#[tokio::main]
pub async fn main() -> mini_redis::Result<()> {
    // construct a subscriber that prints formatted traces to stdout
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber)?;
    ...
}




/*use tracing::trace;
use tracing_subscriber;

fn main() {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    // This will print a trace message.
    trace!("This is a trace message");
}*/







/*
use tracing::warn;
use tracing_subscriber;

fn main() {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    // This will print a warning message.
    warn!("This is a warning message");
}*/