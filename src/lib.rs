pub fn make_boomerang(
    input_path: &str,
    from_sec: &str,
    to_sec: &str,
    _repeat: Option<usize>,
    speed: Option<f64>,
) -> Result<(), ffmpeg_sidecar::error::Error> {
    ffmpeg_sidecar::download::auto_download()?;

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
    let vid_length_str = String::from_utf8(vid_length).unwrap();
    let vid_length_str = vid_length_str.trim();
    let vid_length = vid_length_str.parse::<f64>().unwrap();
    println!("vid_length: {}", vid_length);
    let to_sec = if to_sec.is_empty() || to_sec.parse::<f64>().unwrap() > vid_length {
        vid_length_str
    } else {
        to_sec
    };

    let audio_presence = std::process::Command::new("ffprobe")
        .args([
            "-i",
            input_path,
            "-show_streams",
            "-select_streams",
            "a",
            "-loglevel",
            "error",
        ])
        .output()
        .expect("failed to execute process")
        .stdout;

    let output_path = format!(
        "./output/{}_output.mp4",
        input_path.trim_end_matches(".mp4")
    );

    let filter = "[0]split[a][b];[b]reverse[b];[a][b]concat";
    let mut ffmpeg = std::process::Command::new("ffmpeg");
    // if let Some(speed) = speed {
    //     ffmpeg.args(["-itsscale", &format!("{}", 1.0 / speed)]);
    //     // ffmpeg.args(["-filter:v", &format!("setpts={}/PTS", speed)]);
    // }
    ffmpeg.args([
        "-ss",
        &from_sec,
        "-to",
        &to_sec,
        "-i",
        &input_path,
        "-filter_complex",
        &filter,
        // "-an",
    ]);
    if let Some(speed) = speed {
        // -filter_complex "[0:v]setpts=0.5*PTS[v];[0:a]atempo=2.0[a]" -map "[v]"
        ffmpeg.args([
            "-filter_complex",
            &format!(
                "[0:v]setpts={}/PTS[v];[0:a]atempo={}[a]",
                speed,
                1.0 / speed
            ),
            "-map",
            "[v]",
            "-map",
            "[a]",
        ]);
    }

    ffmpeg.args([&output_path, "-y", "-hide_banner"]);

    let _output = ffmpeg
        .spawn()
        .expect("failed to execute process")
        .wait()
        // .wait_with_output()
        .unwrap();
    println!(
        "audio_presence: {}",
        String::from_utf8_lossy(&audio_presence)
    );
    Ok(())
}
