use std::ops::RangeInclusive;
use std::process::Command;

use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(author, version, verbatim_doc_comment)]
/// Wrapper around `pmset schedule` for easy near-term sleep and shutdown time setting.
/// 
///      scheduleoff sleep 45
/// 
/// will put your computer to sleep in 45 + 10 minutes. Why the +10? Because your Mac will pop-up a 10-minute timer, giving you the option to cancel.
/// 
/// If you'd like to cancel a sleep/shutdown before the pop-up pops up, you can use `sudo pmset schedule cancelall` (see `pmset` docs, or better yet, https://www.macos.utah.edu/documentation/administration/pmset.html)
struct Cli {
    /// Whether to set a sleep or shutdown time
    #[arg(value_enum)]
    action: Action,

    /// Minutes until action initiated. Must be greater than 0
    #[arg(value_parser = minutes_in_range)]
    minutes: u16,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Action {
    /// Put your computer to sleep
    Sleep,
    /// Shutdown your computer
    Shutdown,
}


const MINUTES_RANGE: RangeInclusive<usize> = 1..=65535;

fn minutes_in_range(s: &str) -> Result<u16, String> {
    let minutes: usize = s
        .parse()
        .map_err(|_| format!("`{s}` isn't a number"))?;
    if MINUTES_RANGE.contains(&minutes) {
        Ok(minutes as u16)
    } else {
        Err(format!(
            "desired minutes not in range {}-{}",
            MINUTES_RANGE.start(),
            MINUTES_RANGE.end()
        ))
    }
}

fn main() {
    let cli = Cli::parse();

    // Yields datetime + `cli.minutes` in format suitable for `pmset`
    let date_offset_arg = format!("-v+{}M", cli.minutes);

    let date_output = Command::new("date")
        .args([&date_offset_arg, "+%D %T"])
        .output()
        .expect("`date` call failed.");
    
    if !date_output.status.success() {
        panic!("`date` command failed. \nExit Code: {:?}, \nError Msg: {:?}", 
            date_output.status.code(),
            String::from_utf8(date_output.stderr).unwrap());
    }

    let stdout_text = String::from_utf8(date_output.stdout).unwrap();
    let datetime = stdout_text.trim_end();

    // pmset
    let action = match cli.action {
        Action::Sleep => {
            "sleep"
        }
        Action::Shutdown => {
            "shutdown"
        }
    };
 
    let pmset_output = Command::new("sudo")
        .args([
            "pmset",
            "schedule",
            action,
            datetime
        ])
        .output()
        .expect("`pmset` failed to output.");
    
    if !pmset_output.status.success() {
        panic!("\n`pmset` output an error. 
                \nExit Code: {:?} 
                \nError Msg: {:?} 
                \nOutput Msg: {:?}
                \nSupplied action: {}
                \nSupplied datetime: {}\n", 
            pmset_output.status.code(),
            String::from_utf8(pmset_output.stderr).unwrap(),
            String::from_utf8(pmset_output.stdout).unwrap(),
            action,
            datetime)
    } else {
        println!("{} 10-minute countdown scheduled for {}.", action, datetime);
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}