use clap::{ArgGroup, Parser};
use std::process::Command;
use std::thread;
use std::time::Duration;

const TIMES_TO_PLAY: usize = 3;
const DEFAULT_MESSAGE: &str = "You set an alarm, time is up!";

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
    #[arg(short = 'M', long, required = true, default_value = DEFAULT_MESSAGE)]
    message: String,

    /// Times to play the alarm sound
    #[arg(short, long, default_value_t = TIMES_TO_PLAY)]
    times: usize,
}

pub fn humanize_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    if secs < 60 {
        format!("{} second{}", secs, if secs == 1 { "" } else { "s" })
    } else {
        let mins = secs / 60;
        let remaining_secs = secs % 60;
        if remaining_secs > 0 {
            format!(
                "{} minute{} and {} second{}",
                mins,
                if mins == 1 { "" } else { "s" },
                remaining_secs,
                if remaining_secs == 1 { "" } else { "s" }
            )
        } else {
            format!("{} minute{}", mins, if mins == 1 { "" } else { "s" })
        }
    }
}

pub fn speak_message(message: &str) -> Result<(), Box<dyn std::error::Error>> {
    if cfg!(target_os = "macos") {
        Command::new("say")
            .arg(message)
            .output()
            .expect("Failed to execute say command on macOS");
    } else if cfg!(target_os = "windows") {
        Command::new("powershell")
            .arg("-Command")
            .arg(&format!("Add-Type –TypeDefinition \"using System.Speech; var synth = new Speech.Synthesis.SpeechSynthesizer(); synth.Speak('{}');\"", message))
            .output()
            .expect("Failed to execute PowerShell TTS on Windows");
    } else if cfg!(target_os = "linux") {
        Command::new("espeak")
            .arg(message)
            .output()
            .expect("Failed to execute espeak on Linux");
    } else {
        eprintln!("Unsupported operating system for TTS");
    }
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

    let humanized_duration = humanize_duration(interval_duration);

    if args.repeat {
        println!("Recurring alarm set to play every {}.", humanized_duration);
    } else {
        println!("Alarm set to play after {}.", humanized_duration);
    }

    loop {
        thread::sleep(interval_duration);

        for _ in 0..args.times {
            speak_message(&args.message).unwrap();
        }

        if !args.repeat {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_short_durations() {
        assert_eq!(humanize_duration(Duration::from_secs(0)), "0 seconds");
        assert_eq!(humanize_duration(Duration::from_secs(1)), "1 second");
        assert_eq!(humanize_duration(Duration::from_secs(30)), "30 seconds");
    }

    #[test]
    fn test_exact_minute_durations() {
        assert_eq!(humanize_duration(Duration::from_secs(60)), "1 minute");
        assert_eq!(humanize_duration(Duration::from_secs(180)), "3 minutes");
        assert_eq!(humanize_duration(Duration::from_secs(3600)), "60 minutes");
    }

    #[test]
    fn test_minute_and_second_durations() {
        assert_eq!(
            humanize_duration(Duration::from_secs(61)),
            "1 minute and 1 second"
        );
        assert_eq!(
            humanize_duration(Duration::from_secs(122)),
            "2 minutes and 2 seconds"
        );
        assert_eq!(
            humanize_duration(Duration::from_secs(333)),
            "5 minutes and 33 seconds"
        );
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(humanize_duration(Duration::from_secs(59)), "59 seconds");
        assert_eq!(
            humanize_duration(Duration::from_secs(119)),
            "1 minute and 59 seconds"
        );
        assert_eq!(
            humanize_duration(Duration::from_secs(3599)),
            "59 minutes and 59 seconds"
        );
    }
}
