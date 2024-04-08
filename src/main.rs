use crossterm::{
    event, execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use keybinds::keybinds::handle_keybinds;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Wrap},
};
use std::{error::Error, io, time::Duration};
use typing::typing::TypingGame;

mod keybinds;
mod typing;

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_terminal()?;
    run(&mut terminal, &mut TypingGame::new(20))?;
    restore_terminal(&mut terminal)?;
    Ok(())
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    game: &mut TypingGame,
) -> Result<(), Box<dyn Error>> {
    Ok(loop {
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(3), Constraint::Percentage(80)])
                .split(
                    Layout::default()
                        .direction(Direction::Horizontal)
                        .margin(2)
                        .constraints([
                            Constraint::Percentage(20),
                            Constraint::Percentage(60),
                            Constraint::Percentage(20),
                        ])
                        .split(frame.size())[1],
                );
            let small_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(chunks[0]);
            let big_chunk = Layout::default()
                .direction(Direction::Horizontal)
                .margin(2)
                .constraints([
                    Constraint::Percentage(20),
                    Constraint::Percentage(60),
                    Constraint::Percentage(20),
                ])
                .split(frame.size())[1];
            if game.goal.len() != game.current.len() {
                // the paragraph isn't finished yet
                let mut info = vec![];
                if game.start_time.is_some() {
                    let acc = game.accuracy();
                    let acc_str = if acc.is_nan() {
                        "ACC: --".to_string()
                    } else {
                        format!("ACC: {:.2}%", acc)
                    };

                    info.push(format!("WPM: {:.2}", game.wpm()));
                    info.push(acc_str);
                } else {
                    info.push("WPM: --".to_string());
                    info.push("ACC: --".to_string());
                }
                for (index, stat) in info.iter().enumerate() {
                    frame.render_widget(
                        Paragraph::new(Line::from(Span::from(stat.clone())))
                            .block(Block::new().borders(Borders::all())),
                        small_chunks[index],
                    );
                }
                let text = Paragraph::new(Line::from(game.spans()))
                    .wrap(Wrap { trim: true })
                    .block(
                        Block::default()
                            .borders(Borders::all())
                            .title(Span::styled("DoggyType", Style::default().bold().italic())),
                    )
                    .alignment(Alignment::Left);
                frame.render_widget(text, chunks[1]);
                frame.render_widget(
                    Paragraph::new(Line::from(game.cursor_spans()))
                        .wrap(Wrap { trim: true })
                        .block(Block::default().borders(Borders::all()).title("DoggyType")),
                    chunks[1],
                )
            } else {
                // we're in the endgame now
                assert!(game.end_time.is_some() && game.start_time.is_some());
                let time_diff = game
                    .end_time
                    .unwrap()
                    .duration_since(game.start_time.unwrap());
                let text = Paragraph::new(vec![
                    Line::from(format!(
                        "WPM: {:.2}",
                        game.word_count as f64 / (time_diff.unwrap().as_secs_f64() / 60f64)
                    )),
                    Line::from(format!("ACC: {:.2}%", game.accuracy())),
                    Line::from("".to_string()),
                    Line::from(format!("Commands: ")),
                    Line::from(format!("<Ctrl-C> - quit")),
                    Line::from(format!("<Tab>    - new paragraph")),
                    Line::from(format!("<Esc>    - repeat paragraph")),
                ])
                .block(
                    Block::default()
                        .borders(Borders::all())
                        .title(Span::styled("DoggyType", Style::default().bold().italic())),
                )
                .wrap(Wrap { trim: true });
                frame.render_widget(text, big_chunk);
            }
        })?;

        if event::poll(Duration::from_millis(200))? {
            if let event::Event::Key(key) = event::read()? {
                match handle_keybinds(game, &key) {
                    Err(_) => break,
                    _ => (),
                }
            }
        }
    })
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, Box<dyn Error>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    Ok(terminal.show_cursor()?)
}
