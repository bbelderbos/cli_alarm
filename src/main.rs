use clap::Parser;
use std::io::Read;
use rodio::{Decoder, OutputStream, source::Source};
use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::time::Duration;
use chrono::Local;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short = 'm', long, required = true)]
    minutes: u64,
    #[arg(short = 'r', long, default_value_t = false)]
    repeat: bool,
    #[arg(short, long, env = "ALARM_FILE")]
    file: Option<String>,
}

const DEFAULT_ALARM_URL: &str = "https://bites-data.s3.us-east-2.amazonaws.com/ny_vibes.mp3";

fn main() {
    let args = Cli::parse();

    let audio_file = match &args.file {
        Some(file) => file.clone(),
        None => {
            let default_file = "alarm.mp3";
            if !Path::new(default_file).exists() {
                download_default_alarm(default_file);
            }
            default_file.to_string()
        }
    };

    let minutes = args.minutes;
    let interval_duration = Duration::from_secs(minutes * 60);

    if args.repeat {
        println!("Recurring alarm set for every {} minutes.", minutes);
    } else {
        println!("Alarm set to go off in {} minutes.", minutes);
    }

    loop {
        thread::sleep(interval_duration);
        play_alarm(&audio_file);

        if !args.repeat {
            break;
        }
    }
}

fn play_alarm(audio_file: &str) {
    println!("Playing alarm at {}", Local::now().format("%Y-%m-%d %H:%M:%S"));

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = File::open(audio_file).unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();

    stream_handle.play_raw(source.convert_samples()).unwrap();

    thread::sleep(Duration::from_secs(10));
}

fn download_default_alarm(file_path: &str) {
    println!("Downloading default alarm sound...");
    let response = reqwest::blocking::get(DEFAULT_ALARM_URL).unwrap();
    let mut file = File::create(file_path).unwrap();
    std::io::copy(&mut response.take(10_000_000), &mut file).unwrap(); // Limit to 10MB
    println!("Default alarm sound downloaded to {}", file_path);
}
