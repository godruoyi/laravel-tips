macro_rules! success {
    ($msg:literal) => {{
        use console::{style, Emoji};
        println!(
            "{} {}",
            style(Emoji("🌽", "🐕‍🦺")).green(),
            style($msg).green()
        );
    }};
}

macro_rules! error {
    ($msg:expr) => {{
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

#[macro_export]
macro_rules! log {
    ($msg:expr) => {{
        use console::{style, Emoji};
        use rand::prelude::SliceRandom;

        let emojis = vec![
            Emoji("🍡", "✓"),
            Emoji("🐞", "✓"),
            Emoji("🐕‍🦺", "✓"),
            Emoji("🐘", "✓"),
            Emoji("🍅", "✓"),
            Emoji("🐫", "✓"),
            Emoji("🐻", "✓"),
        ];

        let mut rng = rand::thread_rng();
        let emoji = emojis.choose(&mut rng).unwrap();

        println!("{} {}", style(emoji).green(), style($msg).dim());
    }};
}
