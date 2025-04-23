use {{project-name}}::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Setup trace
    trace::setup_tracing();

    Ok(())
}
