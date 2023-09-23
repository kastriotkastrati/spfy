use std;
struct MacOS;

trait SpotifyInformer {
    fn get_spotify_data(&self) -> String;
}

impl SpotifyInformer for MacOS {
    fn get_spotify_data(&self) -> String {
        let script = [
            "tell application \"Spotify\"",
            "if player state is playing or player state is paused then",
            "return artwork url of current track",
            "end if",
            "end tell",
        ];

        let script_construction = script.iter().flat_map(|&part| ["-e", part]);
        let output = std::process::Command::new("osascript")
            .args(script_construction)
            .output()
            .expect("Could not run osascript:(");

        let data = String::from_utf8_lossy(&output.stdout).to_string();
        return data;
    }
}

pub fn run_command(os: &str) -> String {
    println!("os: {:?}", os);
    let os: &dyn SpotifyInformer = match os {
        "macos" => &MacOS,
        _ => panic!(""),
    };

    let spotify_data = os.get_spotify_data();
    return spotify_data;
}
