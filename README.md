# CLI Alarm

This is a simple CLI alarm utility that plays a sound when the time is reached. Ideal to remind yourself to get up from the computer and stretch for example. You can run it once or repeatedly.

## Installation

```bash
cargo install cli-alarm
```
## Usage

```bash
$ alarm
A simple CLI alarm clock (should help programmers stand up and stretch more)

Usage: cli-alarm [OPTIONS] --minutes <MINUTES>

Options:
  -m, --minutes <MINUTES>
  -r, --repeat
  -f, --file <FILE>        [env: ALARM_FILE=]
  -h, --help               Print help
  -V, --version            Print version
```

## Example

```bash
$ alarm -m 1
Alarm set to go off in 1 minutes.
...
plays sound once after 1 minute
...

$ alarm -m 1 -r
Recurring alarm set for every 1 minutes.
...
plays sound every minute
...
```

## Run in background

Not built-in to the tool, but on Unix systems you can run it in the background like this:

```bash
$ alarm -m 1 -r &
```

## Change the sound file

By default, the alarm sound is the one that comes with the utility (NY vibes from the Pybites podcast). You can change it by providing a path to a sound file or set an environment variable:

```bash
$ alarm -m 1 -f /path/to/sound/file
# or:
$ export ALARM_FILE=/path/to/sound/file
```

## Ideas for improvement

- [ ] Store the default sound file in a more accessible location like `~/.config/cli-alarm/alarm.mp3` or similar.
- [ ] Play an configurable audio message so you know what the alarm is for.
