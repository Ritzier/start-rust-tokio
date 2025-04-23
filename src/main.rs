use {{project-name}}::*;

fn main() -> Result<(), Error> {
    // Setup trace
    trace::setup_tracing();

    Ok(())
}
