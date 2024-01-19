fn main() -> Result<(), ffmpeg_sidecar::error::Error> {
    ffmpeg_sidecar::download::auto_download()?;

    let input_path = std::env::args().nth(1).expect("no video file provided");
    let from_sec = std::env::args().nth(2).unwrap_or("0".to_string());
    let to_sec = std::env::args().nth(3).unwrap_or_default();

    let vid_length = std::process::Command::new("ffprobe")
        .args([
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
            &input_path,
        ])
        .output()
        .expect("failed to execute process")
        .stdout;
    let vid_length = String::from_utf8(vid_length)
        .unwrap()
        .trim()
        .parse::<f64>()
        .unwrap();
    println!("vid_length: {}", vid_length);
    let to_sec = if to_sec.is_empty() || to_sec.parse::<f64>().unwrap() > vid_length {
        vid_length.to_string()
    } else {
        to_sec
    };

    let output_path = format!(
        "./output/{}_output.mp4",
        input_path.trim_end_matches(".mp4")
    );

    let filter = "[0]split[a][b];[b]reverse[b];[a][b]concat";
    let mut ffmpeg = std::process::Command::new("ffmpeg");
    ffmpeg.args([
        "-ss",
        &from_sec,
        "-to",
        &to_sec,
        "-i",
        &input_path,
        "-filter_complex",
        &filter,
        "-an",
        &output_path,
        "-y",
        "-hide_banner",
    ]);

    let _output = ffmpeg
        .spawn()
        .expect("failed to execute process")
        .wait()
        // .wait_with_output()
        .unwrap();

    Ok(())
}
