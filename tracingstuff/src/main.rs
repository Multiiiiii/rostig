
#[tokio::main]
pub async fn main() -> mini_redis::Result<()> {
    // construct a subscriber that prints formatted traces to stdout
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
    
    /*
    let subscriber = tracing_subscriber::fmt()
        // Use a more compact, abbreviated log fromat
        .compact()
        // Display source file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Dont't display the event's target (module path)
        .with_target(false)
        // Build the subscriber
        .finish();
    */
    
}

#[tracing::instrument]
fn trace_me(a: u32,b: u32) -> u32 {
    a + b
}

use tracing::instrument;

impl Handler {
    /// Process a single connection.
    #[instrument(
        name = "Handler::run",
        skip(self),
        fields(
            // '%' serializes the peer ID addr with 'Display'
            peer_addr = %self.connection.peer_addr().unwrap()
        ),
    )]
    async fn run(&mut self) -> mini_redis::Result<()> {
        Ok(())
    }
}

// Convert the redis frame into a command struct. This returns an 
// error if the frame is not a valid redis command.
let cmd =match Comand::from_frame(frame){
    Ok(cmd) => cmd,
    Err(cause) => {
        // The frame was malfromed and could not be parsed. This is 
        // probably indicative of an issue with the client (as opposed 
        // to our server), so wen (1) emit a warning...

        // The syntax here is a shorthand provided by the 'tracing'
        // carte. It can be thought of as similar to: 
        // tracing::warn! {
        // cause = format!("{}", cause),
        // "failed to parse command from frame"
        // };
        // 'tracing' provides structured logging, so information is 
        // "logged" as key-value pairs.
        tracing:: warn! {
            %cause,
            "failed to parse command from frame"
        };
        // ... and (2) respond to the client with the error:
        Command::from_error(cause)
    }
}