mod commands;
mod setup;
mod spfy;

#[tokio::main]
async fn main() -> () {
    spfy::run_program().await;
    return ();
}
