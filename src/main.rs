mod config;
mod test_cases;

use console::{style, Emoji};
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use std::thread;
use std::time::{Duration, Instant};

static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍 ", "");
static CHECKMARK: Emoji<'_, '_> = Emoji("✅ ", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");

fn main() {
    let config = config::get_config();
    println!("config:\n{}", config);

    let started = Instant::now();
    let spinner_style = ProgressStyle::default_spinner()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
        .template("{prefix:.bold.dim} {spinner} {wide_msg}");

    println!(
        "{} {}Looking up test cases...",
        style("[1/3]").bold().dim(),
        LOOKING_GLASS
    );

    // Collect input files
    let cases = test_cases::collect_test_cases(&config.inputs, &config.outputs);

    // Iterate over cases
    let length = cases.len();
    for (name, _) in cases.iter() {
        println!("{}", name);
    }

    println!(
        "{} {}Running test cases...",
        style("[2/3]").bold().dim(),
        SPARKLE
    );

    let m = MultiProgress::new();
    for (i, case) in cases.iter().enumerate() {
        let pb = m.add(ProgressBar::new(1000));
        pb.set_style(spinner_style.clone());
        pb.set_prefix(format!("[{}/{}]", i + 1, length));

        let name = case.0.clone();
        let _ = thread::spawn(move || {
            for _ in 0..1000 {
                pb.set_message(format!("{}: {}", name, "running"));
                pb.inc(1);
                thread::sleep(Duration::from_millis(3));
            }
            pb.finish_with_message("waiting...");
        });
    }
    m.join_and_clear().unwrap();

    println!(
        "{} {}Done in {}",
        style("[3/3]").bold().dim(),
        CHECKMARK,
        HumanDuration(started.elapsed())
    );
}
