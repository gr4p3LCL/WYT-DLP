pub fn main() {
    let mut child = std::process::Command::new("yt-dlp")
        .arg("-U")
        .spawn()
        .unwrap();

    let _result = child.wait().unwrap();
}
