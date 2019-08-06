use std::io;
use termion::raw::IntoRawMode;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};

use crate::util::event::{Config, Event, Events};
use std::time::Duration;
use structopt::StructOpt;
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::screen::AlternateScreen;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "tick-rate", default_value = "250")]
    tick_rate: u64,
    #[structopt(long = "log")]
    log: bool,
}

pub fn render_tui() -> Result<(), io::Error> {
    let events = Events::with_config(Config {
      tick_rate: Duration::from_millis(cli.tick_rate),
      ..Config::default()
    });
    
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    loop {
      terminal.draw(|mut f| {
          let chunks = Layout::default()
              .direction(Direction::Vertical)
              .margin(1)
              .constraints(
                  [
                      Constraint::Percentage(10),
                      Constraint::Percentage(80),
                      Constraint::Percentage(10)
                  ].as_ref()
              )
              .split(f.size());
          Block::default()
              .title("Block")
              .borders(Borders::ALL)
              .render(&mut f, chunks[0]);
          Block::default()
              .title("Block 2")
              .borders(Borders::ALL)
              .render(&mut f, chunks[2]);
      });
    }

    Ok(())
}