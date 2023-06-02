use crate::model::{Entity, Tip};
use anyhow::anyhow;
use base64::{engine::general_purpose, Engine};
use home::home_dir;
use std::io::{stdout, Write};
use std::path::PathBuf;
use termimad::crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode::*, KeyEvent},
    queue,
    style::Color::*,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use termimad::*;

pub fn base64_decode(c: String) -> anyhow::Result<String> {
    // github api response always has a newline between each base64 part
    let c = c.replace('\n', "");
    let c = c.replace("\\n", "");

    let decoded = general_purpose::STANDARD.decode(c.as_bytes())?;

    String::from_utf8(decoded).map_err(|e| e.into())
}

pub fn parse_tips(c: String) -> anyhow::Result<Vec<Tip>> {
    Ok(base64_decode(c)?
        .lines()
        .fold((None, Vec::new()), process_line)
        .1)
}

fn process_line(mut state: (Option<Tip>, Vec<Tip>), line: &str) -> (Option<Tip>, Vec<Tip>) {
    match state.0.take() {
        Some(mut entity) => {
            if line.starts_with("###") {
                state.1.push(entity);
                let title = line.trim_start_matches("###").trim().to_string();
                state.0 = Some(Tip {
                    title,
                    content: String::new(),
                });
            } else {
                entity.content.push_str(line);
                entity.content.push('\n');
                state.0 = Some(entity);
            }
        }
        None => {
            if line.starts_with("###") {
                let title = line.trim_start_matches("###").trim().to_string();
                state.0 = Some(Tip {
                    title,
                    content: String::new(),
                });
            }
        }
    }
    state
}

pub fn normalize_path(suffix: String, path: Option<PathBuf>) -> anyhow::Result<String> {
    let laravel_dir = match path {
        Some(path) => path,
        None => create_default_laravel_directory()?,
    };

    if !laravel_dir.exists() {
        return Err(anyhow!("{} not exists", laravel_dir.to_string_lossy()));
    }

    let normalized_path = laravel_dir.join(suffix).to_string_lossy().to_string();

    Ok(normalized_path)
}

fn create_default_laravel_directory() -> anyhow::Result<PathBuf> {
    let home_dir = home_dir().ok_or_else(|| anyhow::anyhow!("Failed to get home directory"))?;
    let laravel_dir = home_dir.join(".laravel");

    if !laravel_dir.exists() {
        std::fs::create_dir(&laravel_dir)?;
    }

    Ok(laravel_dir)
}

pub fn pretty_tip(entity: Entity) -> anyhow::Result<()> {
    pretty_tips(vec![entity])
}

// @todo refactor this function to make it more clean and readable
pub fn pretty_tips(entities: Vec<Entity>) -> anyhow::Result<()> {
    let skin = make_skin();
    // we could also have used stderr
    let mut w = stdout();

    queue!(w, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    queue!(w, Hide)?;

    let size = &entities.len();
    let mut contents = entities
        .iter()
        .map(|entity| format!("### {}\n{}\n", entity.title, entity.content))
        .collect::<Vec<String>>()
        .join("\n");

    if size > &1 {
        contents = format!("## Found {} tips\n\n\n\n{}", size, contents);
    }

    let mut view = MadView::from(contents, view_area(), skin);
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
                view.resize(&view_area());
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

fn view_area() -> Area {
    let mut area = Area::full_screen();
    area.pad_for_max_width(120);
    area.pad(2, 2);
    area
}

fn make_skin() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.table.align = Alignment::Center;
    skin.set_headers_fg(AnsiValue(178));
    skin.bold.set_fg(Yellow);
    skin.italic.set_fg(Magenta);
    skin.scrollbar.thumb.set_fg(AnsiValue(178));

    skin
}

#[cfg(test)]
mod test_base {
    use super::*;

    #[test]
    fn test_base64_decode_from_local_file() {
        let encode_content = std::fs::read_to_string("testdata/api_base64.md");
        assert!(encode_content.is_ok());

        let x = base64_decode(encode_content.unwrap());
        assert!(x.is_ok());
    }

    #[test]
    fn test_parse_tips() {
        let encode_content = std::fs::read_to_string("testdata/api_base64.md");
        assert!(encode_content.is_ok());

        let x = parse_tips(encode_content.unwrap());
        assert!(x.is_ok());
    }

    #[test]
    fn test_can_normalize_path() {
        let path = normalize_path(".test".to_string(), None);
        let home = home_dir().unwrap().join(".laravel/.test");

        assert!(path.is_ok());
        assert_eq!(path.unwrap(), home.to_string_lossy().to_string());

        let path = normalize_path(".test".to_string(), Some(PathBuf::from("/tmp")));
        assert_eq!(path.unwrap(), "/tmp/.test");

        let path = normalize_path(".test".to_string(), Some(PathBuf::from("/not-exists/foo")));
        assert!(path.is_err());
    }
}
