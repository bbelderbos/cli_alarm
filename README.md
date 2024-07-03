# CLI Alarm

This is a simple CLI alarm utility that plays a sound when the time is reached. Ideal to remind yourself to get up from the computer and stretch for example. You can run it once or repeatedly.

## Installation

```bash
cargo install cli-alarm
```
## Usage

```bash
A simple CLI alarm saying a message after a certain amount of time.

Usage: alarm [OPTIONS] --message <MESSAGE> <--seconds <SECONDS>|--minutes <MINUTES>>

Options:
  -s, --seconds <SECONDS>  Number of seconds to wait before playing the alarm
  -m, --minutes <MINUTES>  Number of minutes to wait before playing the alarm
  -r, --repeat             Repeat the alarm
  -M, --message <MESSAGE>  Message to speak instead of playing an audio file [default: "You set an alarm, time is up!"]
  -t, --times <TIMES>      Times to play the alarm sound [default: 3]
  -h, --help               Print help
  -V, --version            Print version
```

## Run in background

To run it permanently in the background I added this to my .zhrc file:

```bash
function run_alarm_if_not_running {
    if ! pgrep -f "alarm -m 60 -M" > /dev/null; then
        alarm -m 60 -M "go walk" -t 2 -r &
    fi
}
run_alarm_if_not_running
```

When opening a new terminal it checks if the alarm is running, if not it starts it.

This particular invocation will say "go walk" two times every hour, a good reminder to get up and stretch!
