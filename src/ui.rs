use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::fmt::Write;

macro_rules! success {
    ($msg:literal) => {{
        use console::{style, Emoji};
        println!();
        println!();
        println!(
            "{} {}",
            style(Emoji("🌽", "🐕‍🦺")).green(),
            style($msg).green()
        );
    }};
}

macro_rules! error {
    ($msg:literal) => {{
        use console::{style, Emoji};
        println!("{} {}", style(Emoji("🦧", "x")).red(), style($msg).yellow());
    }};
}

macro_rules! pretty_tip {
    ($title:expr, $content:expr) => {{
        bat::PrettyPrinter::new()
            .input_from_bytes($title.as_bytes())
            .grid(false)
            .theme("zenburn")
            .line_numbers(false)
            .header(false)
            .print()
            .unwrap();
        println!();
        bat::PrettyPrinter::new()
            .language("markdown")
            .input_from_bytes($content.as_bytes())
            .theme("zenburn")
            .grid(false)
            .line_numbers(false)
            .colored_output(true)
            .true_color(true)
            .header(false)
            .print()
            .unwrap();
    }};
}

macro_rules! log {
    ($msg:literal) => {{
        use console::{style, Emoji};

        println!(
            "{} {}",
            style(Emoji("🦙", "✓")).green(),
            style($msg).green()
        );
    }};
}

// @todo refactor this to macro
pub fn progress_bar(total: u64, callback: impl Fn(&mut ProgressBar)) {
    let mut pb = ProgressBar::new(total);

    pb.set_style(
        ProgressStyle::with_template(
            "🐘 [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>7}/{len:7} ({eta}) {msg}",
        )
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
        })
        .progress_chars("#>-"),
    );

    callback(&mut pb);

    pb.finish_with_message("Done");
}
