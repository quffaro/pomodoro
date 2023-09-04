use colored::Colorize;
use notify_rust::{Notification, Timeout};
use std::env;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

#[derive(Copy, Clone, Debug)]
enum Status {
    TimerOn,
    TimerPaused,
    ShortBreak,
    LongBreak,
}

fn countdown(time: &mut u16, status: Status, pomodoros: u8) {
    *time -= 1;
    let seconds = *time % 60;
    let minutes = (*time / 60) % 60;
    print!(
        "\r#{}: {:?}: {:0>2}:{:0>2}\r",
        pomodoros.to_string().red(),
        status,
        minutes,
        seconds
    );
    let _ = std::io::stdout().flush();
}

fn parse_args(args: &[String]) -> u8 {
    match args.len() {
        1 => 0,
        2 => args[1].parse::<u8>().unwrap(),
        _ => panic!("Too many"),
    }
}

fn main() {
    let duration_timer = 25 * 60;
    let duration_short = 5 * 60;
    let duration_long = 15 * 60;
    let interval = Duration::from_secs(1);

    let args: Vec<String> = env::args().collect();
    let initial_count = parse_args(&args);
    let mut pomodoros = initial_count;

    let mut status = Status::TimerOn;
    let mut time = duration_timer;

    loop {
        sleep(interval);
        countdown(&mut time, status, pomodoros);
        match status {
            Status::ShortBreak | Status::LongBreak => {
                match time {
                    59 => {
                        Notification::new()
                            .summary("Pomodoro")
                            .body(format!("1 minute left!").as_str())
                            // .icon("firefox")
                            .timeout(Timeout::Milliseconds(6000)) //milliseconds
                            .show()
                            .unwrap();
                    }
                    0 => {
                        status = Status::TimerOn;
                        time = duration_timer;
                        Notification::new()
                            .summary("Pomodoro")
                            .body(
                                format!("Time for go back!\n Pomodoro count: {}", pomodoros)
                                    .as_str(),
                            )
                            // .icon("firefox")
                            .timeout(Timeout::Milliseconds(6000)) //milliseconds
                            .show()
                            .unwrap();
                    }
                    _ => {}
                }
            }
            Status::TimerOn => {
                if time == 0 {
                    pomodoros += 1;
                    if pomodoros % 4 == 0 {
                        status = Status::LongBreak;
                        time = duration_long;
                        Notification::new()
                            .summary("Pomodoro")
                            .body(
                                format!(
                                    "You have completed {} pomodoros! Time for a long break.",
                                    pomodoros
                                )
                                .as_str(),
                            )
                            // .icon("firefox")
                            .timeout(Timeout::Milliseconds(6000)) //milliseconds
                            .show()
                            .unwrap();
                    } else {
                        status = Status::ShortBreak;
                        time = duration_short;
                        Notification::new()
                            .summary("Pomodoro")
                            .body(format!("You have completed {} pomodoros!", pomodoros).as_str())
                            // .icon("~/Downloads/food-tomato.svg")
                            .timeout(Timeout::Milliseconds(6000)) //milliseconds
                            .show()
                            .unwrap();
                    };
                }
            }
            Status::TimerPaused => {}
        }
    }
}
