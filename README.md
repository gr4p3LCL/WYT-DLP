# WYT-DLP

WYT-DLP is a simple wrapper for [yt-dlp](https://github.com/yt-dlp/yt-dlp), designed to download videos from multiple sites. It reads URLs from a specified file input, making bulk downloading easy and organized.

## Features

- Download videos from multiple websites supported by yt-dlp.
- Read URLs from a file input for batch processing.
- Simple and straightforward command-line interface.

## Requirements

- [yt-dlp](https://github.com/yt-dlp/yt-dlp)
- [FFmpeg](https://ffmpeg.org)

## Installation

1. Clone this repository:

    ```bash
    git clone https://github.com/gr4p3LCL/WYT-DLP.git
    cd WYT-DLP
    ```

2. Compile

    ```bash
    cargo build --release
    ```

## Usage

1. Prepare a text file containing the URLs you want to download, with each URL on a new line. For example, `urls.txt`:

    ```
    https://www.youtube.com/watch?v=example0
    https://www.youtube.com/watch?v=example1
    https://www.youtube.com/watch?v=example2
    ```

2. Run WYT-DLP with the file as an argument:

    ```bash
    cargo run --release -- urls.txt
    
    # or move the executable from `WYT-DLP/target/release/WYT-DLP.exe` somewhere else and run:
    WYT-DLP urls.txt
    ```

WYT-DLP will read the URLs from `urls.txt` and download each video using yt-dlp.

## License

See the [LICENSE](LICENSE) file for details.
