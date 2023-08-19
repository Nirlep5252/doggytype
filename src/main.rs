use crossterm::{
    event, execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Wrap},
};
use std::{error::Error, io, time::Duration};
use typing::typing::TypingGame;

mod typing;

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_terminal()?;
    run(&mut terminal, &mut TypingGame::new())?;
    restore_terminal(&mut terminal)?;
    Ok(())
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    game: &mut TypingGame,
) -> Result<(), Box<dyn Error>> {
    Ok(loop {
        terminal.draw(|frame| {
            let mut spans = vec![];
            let goal_chars = game.goal.chars().collect::<Vec<char>>();
            for (i, ch) in game.current.char_indices() {
                let color = if goal_chars[i] == ch {
                    Color::Green
                } else {
                    Color::Red
                };
                spans.push(Span::styled(
                    goal_chars[i].to_string(),
                    Style::default().fg(color),
                ))
            }
            spans.push(Span::styled(
                &game.goal[game.current.len()..],
                Style::default().fg(Color::Gray),
            ));
            let text = Paragraph::new(Line::from(spans))
                .wrap(Wrap { trim: true })
                .block(Block::default().borders(Borders::all()).title(Span::styled(
                    "Welcome to DoggyType",
                    Style::default().add_modifier(Modifier::BOLD).italic(),
                )))
                .alignment(Alignment::Left);
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(5)
                .constraints([
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                ])
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(5),
                    Constraint::Percentage(5),
                    Constraint::Percentage(80),
                    Constraint::Percentage(5),
                    Constraint::Percentage(5),
                ])
                .split(frame.size());
            frame.render_widget(text, chunks[2]);
            let x = game.current.len() as u16 + 1;
            let y = chunks[2].y + 1;
            frame.set_cursor(chunks[2].x + x % chunks[2].width, y + x / chunks[2].width);
        })?;
        if event::poll(Duration::from_millis(20))? {
            if let event::Event::Key(key) = event::read()? {
                if key.modifiers.contains(event::KeyModifiers::CONTROL)
                    && key.code == event::KeyCode::Char('c')
                {
                    break;
                } else if let event::KeyCode::Char(ch) = key.code {
                    if game.current.len() == game.goal.len() {
                        // TODO: we're in the endgame now
                    } else {
                        game.current.push(ch);
                    }
                } else if event::KeyCode::Backspace == key.code {
                    if !game.current.is_empty() {
                        game.current.pop();
                    }
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
