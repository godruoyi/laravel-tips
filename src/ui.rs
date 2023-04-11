macro_rules! success {
    ($fmt:literal, $ex:expr) => {{
        use console::{style, Emoji};
        let formatstr = format!($fmt, $ex);
        println!(
            "{} {}",
            style(Emoji("✅", "✓")).green(),
            style(formatstr).green()
        );
    }};
}

macro_rules! log {
    ($fmt:literal, $ex:expr) => {{
        use console::{style, Emoji};
        let formatstr = format!($fmt, $ex);

        println!(
            "{} {}",
            style(Emoji("🦙", "✓")).green(),
            style(formatstr).green()
        );
    }};
}
