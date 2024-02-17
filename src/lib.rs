pub fn make_boomerang(
    input_path: &str,
    from_sec: &str,
    to_sec: &str,
    _repeat: Option<usize>,
    speed: Option<f64>,
    fps: Option<usize>,
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

    // if ffprobe output is empty then there is no audio
    let audio_present = !audio_presence.is_empty();
    println!(
        "audio_presence {}: {}",
        audio_present,
        String::from_utf8_lossy(&audio_presence)
    );

    let output_path = format!(
        "./output/{}_output.mp4",
        input_path.trim_end_matches(".mp4")
    );

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
        // "-an",
    ]);
    if let Some(speed) = speed {
        let video_speed_filter = format!(
            "[0:v]setpts={}*PTS[c];[c]split[a][b];[b]reverse[b];[a][b]concat[vout]",
            1.0 / speed
        );
        ffmpeg.args(["-filter_complex", &video_speed_filter, "-map", "[vout]"]);
        if audio_present {
            let audio_speed_filter = format!(
                "[0:a]atempo={}[d];[d]asplit[e][f];[f]areverse[f];[e][f]concat=v=0:a=1[aout]",
                speed
            );
            ffmpeg.args(["-filter_complex", &audio_speed_filter, "-map", "[aout]"]);
        }
    }

    ffmpeg.arg(&output_path);
    if let Some(fps) = fps {
        ffmpeg.args(["-r", &format!("{}", fps)]);
    }
    ffmpeg.args(["-y", "-hide_banner"]);

    let _output = ffmpeg
        .spawn()
        .expect("failed to execute process")
        .wait()
        // .wait_with_output()
        .unwrap();
    Ok(())
}
