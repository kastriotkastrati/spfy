use dotenv;
use rspotify::{self, prelude::BaseClient};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let creds = rspotify::Credentials::from_env().unwrap();
    let spotify = rspotify::ClientCredsSpotify::new(creds);

    spotify.request_token().await.unwrap();

    // Running the requests
    let birdy_uri =
        rspotify::model::AlbumId::from_uri("spotify:album:0sNOF9WDwhWunNAHPD3Baj").unwrap();
    let albums = spotify.album(birdy_uri).await;

    println!("Response: {albums:#?}");
}

fn establish_connection() {}
