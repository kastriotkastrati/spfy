use std;
pub struct MacOS;

pub trait SpotifyInformer {
    fn get_spotify_data(&self) -> String;
    fn set_wallpaper(&self, path: &std::path::Path) -> ();
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

    fn set_wallpaper(&self, path: &std::path::Path) -> () {
        let path = path.to_str().unwrap();
        let set_desktop_picture = format!("set desktop picture to POSIX file \"{}\"", path);
        let script = [
            "tell application \"Finder\"",
            set_desktop_picture.as_str(),
            "end tell",
        ];

        let script_construction = script.iter().flat_map(|&part| ["-e", part]);
        let _ = std::process::Command::new("osascript")
            .args(script_construction)
            .output()
            .expect("Could not run osascript.");
    }
}

pub fn get_impl_commands<'a>(os: &'a str) -> &'a dyn SpotifyInformer {
    let os_impl: &dyn SpotifyInformer = match os {
        "macos" => &MacOS,
        _ => panic!(""),
    };

    return os_impl;
}
