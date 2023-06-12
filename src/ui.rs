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
        eprintln!("{} {}", style(Emoji("🦧", "x")).red(), style($msg).yellow());
    }};
}

#[macro_export]
macro_rules! log {
    ($msg:expr) => {{
        use console::{style, Emoji};
        use rand::prelude::SliceRandom;

        let emojis = vec![
            Emoji("🍡", "✓"),
            Emoji("🍋", "✓"),
            Emoji("🍅", "✓"),
            Emoji("🍺", "✓"),
            Emoji("🍓", "✓"),
            Emoji("🥑", "✓"),
            Emoji("🥦", "✓"),
        ];

        let mut rng = rand::thread_rng();
        let emoji = emojis.choose(&mut rng).unwrap();
        let space = &$msg.starts_with(' ');

        println!(
            "{}{} {}",
            if *space { "  " } else { "" },
            style(emoji).green(),
            style($msg.trim()).dim(),
        );
    }};
}
