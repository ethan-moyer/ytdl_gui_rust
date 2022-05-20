use druid::{Data, Lens, EventCtx, Env, ExtEventSink, Selector, Target, AppDelegate, Handled};
use std::{process::{Command, ExitStatus}, fmt, thread};

const FINISH_DOWNLOAD_VIDEO: Selector<ExitStatus> = Selector::new("finish_download_video");

#[derive(Clone, Data, Lens)]
pub struct AppState {
    video_url: String,
    preferred_quality: QualityOptions,
    video_format: VideoFormats,
    audio_format: AudioFormats,
    audio_only: bool,
    busy: bool,
    progress: f64,
    output: String,
}

impl AppState {
    pub fn new() -> Self{
        Self {
            video_url: "".into(),
            preferred_quality: QualityOptions::Q1080,
            video_format: VideoFormats::mkv,
            audio_format: AudioFormats::mp3,
            audio_only: false,
            busy: false,
            progress: 0.0,
            output: "".into(),
        }
    }

    pub fn click_download(ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        if !data.busy {
            wrapped_download_video(ctx.get_external_handle(), (data.video_url.clone(), data.preferred_quality, data.video_format, data.audio_format, data.audio_only));
            data.output = format!("Attempting to download {}...", data.video_url);
            data.busy = true;
        }        
    }

    pub fn click_stream(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        data.output = format!("Attempting to stream {} at {:?} quality", data.video_url, data.preferred_quality);
    }
}

fn wrapped_download_video(sink: ExtEventSink, options: (String, QualityOptions, VideoFormats, AudioFormats, bool)) {
    thread::spawn(move || {
        let status = download_video(options);

        sink.submit_command(FINISH_DOWNLOAD_VIDEO, status, Target::Auto).expect("command failed to submit");
    });
}

fn download_video(options: (String, QualityOptions, VideoFormats, AudioFormats, bool)) -> ExitStatus {
    let video_res = match options.1 {
        QualityOptions::Q144 => "bv[height<=144]+ba",
        QualityOptions::Q240 => "bv[height<=240]+ba",
        QualityOptions::Q360 => "bv[height<=360]+ba",
        QualityOptions::Q480 => "bv[height<=480]+ba",
        QualityOptions::Q720 => "bv[height<=720]+ba",
        QualityOptions::Q1080 => "bv[height<=1080]+ba",
        QualityOptions::Q1440 => "bv[height<=1440]+ba",
        QualityOptions::Q2160 => "bv[height<=2160]+ba",
    };
    
    let video_format_str = format!("{}", options.2);
    let audio_format_str = format!("{}", options.3);

    let args = match options.4 {
        true => vec!["-f", "ba", "--audio-format", &audio_format_str, "-x", &options.0],
        false => vec!["-f", &video_res, "--recode-video", &video_format_str, &options.0],
    };

    let cmd_output = Command::new("yt-dlp")
        .args(args)
        .output()
        .expect("failed to execute");
    
    match String::from_utf8(cmd_output.stdout) {
        Ok(v) => println!("{}", v),
        Err(_) => println!("failed to convert output to utf"),
    };

    cmd_output.status
}

pub struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn command(&mut self, ctx: &mut druid::DelegateCtx, _target: Target, cmd: &druid::Command, data: &mut AppState, env: &Env) -> druid::Handled {
        if let Some(status) = cmd.get(FINISH_DOWNLOAD_VIDEO) {
            data.busy = false;
            match status.code() {
                Some(code) => {
                    if code == 0 {
                        data.output = "Video downloaded successfully.".to_string();
                    } else {
                        data.output = "There was an issue while downloading the video.".to_string();
                    }
                },
                None => data.output = "Download was terminated.".to_string()
            }
            Handled::Yes
        } else {
            Handled::No
        }
    }
}

#[derive(Clone, Data, PartialEq, Debug, Copy)]
pub enum QualityOptions {
    Q144,
    Q240,
    Q360,
    Q480,
    Q720,
    Q1080,
    Q1440,
    Q2160,
}

#[derive(Clone, Data, PartialEq, Debug, Copy)]
pub enum VideoFormats {
    mp4,
    mkv,
    mov,
}

impl fmt::Display for VideoFormats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VideoFormats::mp4 => write!(f, "mp4"),
            VideoFormats::mkv => write!(f, "mkv"),
            VideoFormats::mov => write!(f, "mov"),
        }
    }
}

#[derive(Clone, Data, PartialEq, Debug, Copy)]
pub enum AudioFormats {
    mp3,
    wav,
    ogg
}

impl fmt::Display for AudioFormats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AudioFormats::mp3 => write!(f, "mp3"),
            AudioFormats::wav => write!(f, "wav"),
            AudioFormats::ogg => write!(f, "ogg"),
        }
    }
}