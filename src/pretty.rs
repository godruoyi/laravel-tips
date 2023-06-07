use crate::model::Entity;
use crate::OutputFormat;
use std::io::{stdout, Write};
use termimad::crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode::*, KeyEvent},
    queue,
    style::Color::*,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use termimad::{Alignment, Area, MadSkin, MadView};

pub struct Pretty {
    format: OutputFormat,
}

impl Pretty {
    pub fn new(format: OutputFormat) -> Self {
        Self { format }
    }

    fn printer(&self) -> Box<dyn Printable> {
        match self.format {
            OutputFormat::Text => Box::new(TextPrinter {}),
            OutputFormat::Terminal => Box::new(TerminalPrinter {}),
            OutputFormat::Json => Box::new(JsonPrinter {}),
        }
    }

    /// print single tip to stdout or terminal(controlled by `format` flag)
    pub fn print_tip(&self, tip: Entity) -> anyhow::Result<()> {
        self.print_tips(vec![tip])
    }

    /// print tips to stdout or terminal(controlled by `format` flag)
    pub fn print_tips(&self, tips: Vec<Entity>) -> anyhow::Result<()> {
        self.printer().print(tips)
    }
}

trait Printable {
    fn print(&self, tips: Vec<Entity>) -> anyhow::Result<()>;
}

struct TextPrinter {}

struct TerminalPrinter {}

struct JsonPrinter {}

impl Printable for TextPrinter {
    fn print(&self, tips: Vec<Entity>) -> anyhow::Result<()> {
        for tip in tips {
            println!("### {}\n{}\n", tip.title, tip.content);
        }

        Ok(())
    }
}

impl Printable for JsonPrinter {
    fn print(&self, tips: Vec<Entity>) -> anyhow::Result<()> {
        println!("{}", serde_json::to_string(&tips)?);

        Ok(())
    }
}

impl TerminalPrinter {
    fn new_view_area() -> Area {
        let mut area = Area::full_screen();
        area.pad_for_max_width(120);
        area.pad(2, 2);
        area
    }

    fn new_skin() -> MadSkin {
        let mut skin = MadSkin::default();
        skin.table.align = Alignment::Center;
        skin.set_headers_fg(AnsiValue(178));
        skin.bold.set_fg(Yellow);
        skin.italic.set_fg(Magenta);
        skin.scrollbar.thumb.set_fg(AnsiValue(178));

        skin
    }
}

impl Printable for TerminalPrinter {
    fn print(&self, tips: Vec<Entity>) -> anyhow::Result<()> {
        let skin = Self::new_skin();
        // we could also have used stderr
        let mut w = stdout();

        queue!(w, EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;
        queue!(w, Hide)?;

        let size = &tips.len();
        let mut contents = tips
            .iter()
            .map(|entity| format!("### {}\n{}\n", entity.title, entity.content))
            .collect::<Vec<String>>()
            .join("\n");

        if size > &1 {
            contents = format!("## Found {} tips\n\n\n\n{}", size, contents);
        }

        let mut view = MadView::from(contents, Self::new_view_area(), skin);
        loop {
            view.write_on(&mut w)?;
            w.flush()?;
            match event::read() {
                Ok(Event::Key(KeyEvent { code, .. })) => match code {
                    Up | Char('k') | Char('K') => view.try_scroll_lines(-1),
                    Down | Char('j') | Char('J') => view.try_scroll_lines(1),
                    PageUp => view.try_scroll_pages(-1),
                    PageDown => view.try_scroll_pages(1),
                    _ => break,
                },
                Ok(Event::Resize(..)) => {
                    queue!(w, Clear(ClearType::All))?;
                    view.resize(&Self::new_view_area());
                }
                _ => {}
            }
        }

        terminal::disable_raw_mode()?;
        queue!(w, Show)?; // we must restore the cursor
        queue!(w, LeaveAlternateScreen)?;
        w.flush()?;

        Ok(())
    }
}
