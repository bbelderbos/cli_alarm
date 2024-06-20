use std::thread;
use clap::Parser;
use clap::ArgGroup;
use std::time::Duration;
use tts::Tts;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(group = ArgGroup::new("time").args(&["seconds", "minutes"]).required(true))]
struct Cli {
    /// Number of seconds to wait before playing the alarm
    #[arg(short = 's', long)]
    seconds: Option<u64>,

    /// Number of minutes to wait before playing the alarm
    #[arg(short = 'm', long)]
    minutes: Option<u64>,

    /// Repeat the alarm
    #[arg(short, long, default_value_t = false)]
    repeat: bool,

    /// Message to speak instead of playing an audio file
    #[arg(short = 'M', long, required = true, default_value = "hi, I am your alarm")]
    message: String,
}

pub fn speak_message(message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut tts = Tts::default()?;
    tts.speak(message, false)?;
    Ok(())
}

fn main() {
    let args = Cli::parse();

    let interval_duration = if let Some(seconds) = args.seconds {
        Duration::from_secs(seconds)
    } else if let Some(minutes) = args.minutes {
        Duration::from_secs(minutes * 60)
    } else {
        unreachable!("One of `seconds` or `minutes` must be provided")
    };

    if args.repeat {
        println!("Recurring alarm set.");
    } else {
        println!("Alarm set.");
    }

    loop {
        thread::sleep(interval_duration);
        speak_message(&args.message).unwrap();
        if !args.repeat {
            break;
        }
    }
}
