pub fn main(url_string: String, out_dir: &String) {
    let path = format!("{}/{}", out_dir, "%(title)s [%(id)s].%(ext)s");
    let mut child = std::process::Command::new("yt-dlp")
        .arg("-q")
        .arg("--progress")
        .arg("--merge-output-format")
        .arg("mp4")
        .arg("-f")
        .arg("bestvideo+bestaudio/best")
        .arg("-o")
        .arg(path)
        .arg(url_string)
        .spawn()
        .unwrap();

    let _result = child.wait().unwrap();
}
