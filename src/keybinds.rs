pub mod keybinds {
    use crate::typing::typing::TypingGame;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, KeyEventKind};
    use std::env::consts;

    pub fn handle_keybinds(game: &mut TypingGame, key: &KeyEvent) -> Result<(), ()> {
        if consts::OS == "windows" {
            if key.kind == KeyEventKind::Release {
                return Ok(());
            }
        }
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            return Err::<(), ()>(());
        } else if let KeyCode::Char(ch) = key.code {
            if game.current.len() < game.goal.len() {
                if game.current.is_empty() {
                    game.start_time = Some(std::time::SystemTime::now());
                }
                game.current.push(ch);
                if game.current.len() == game.goal.len() {
                    game.end_time = Some(std::time::SystemTime::now());
                }
            }
        } else if KeyCode::Backspace == key.code {
            if !game.current.is_empty() {
                if !key.modifiers.contains(KeyModifiers::ALT) {
                    game.current.pop();
                } else {
                    let words = game.current.split(" ").collect::<Vec<&str>>();
                    game.current = words[..words.len() - 1].join(" ").to_string();
                    game.current.push_str(" ");
                }
            }
        } else if KeyCode::Esc == key.code {
            game.reset();
        } else if KeyCode::Tab == key.code {
            game.new_goal();
            game.reset();
        }
        Ok(())
    }
}
