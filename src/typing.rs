pub mod typing {
    use rand_word;
    use ratatui::{text::Span, style::{Color, Style}};
    use std::time::SystemTime;

    #[derive(Debug)]
    pub struct TypingGame {
        pub goal: String,
        pub current: String,
        pub start_time: Option<SystemTime>,
        pub end_time: Option<SystemTime>,
        pub word_count: usize,
    }

    impl TypingGame {
        pub fn new(words: usize) -> Self {
            let mut goal = rand_word::new(words);
            while goal.contains("º") {
                goal = rand_word::new(words);
            }
            TypingGame {
                goal,
                current: "".to_string(),
                start_time: None,
                end_time: None,
                word_count: words,
            }
        }

        pub fn reset(&mut self) {
            self.current = "".to_string();
            self.start_time = None;
        }

        pub fn new_goal(&mut self) {
            self.goal = TypingGame::new(self.word_count).goal;
        }

        pub fn accuracy(&self) -> f64 {
            let mut correct_count = 0;
            let goal_chars = self.goal.chars().collect::<Vec<char>>();

            for (i, ch) in self.current.char_indices() {
                if goal_chars[i] == ch {
                    correct_count += 1;
                }
            }
            correct_count as f64 * 100f64 / self.current.len() as f64
        }

        pub fn wpm(&self) -> f64 {
            self.current.split_whitespace().count() as f64
                / (SystemTime::now()
                    .duration_since(self.start_time.unwrap())
                    .unwrap()
                    .as_secs_f64()
                    / 60f64)
        }

        pub fn spans(&self) -> Vec<Span> {
            let mut spans = vec![];
            let goal_chars = self.goal.chars().collect::<Vec<char>>();
            for (i, ch) in self.current.char_indices() {
                let color = if goal_chars[i] == ch {
                    Color::Green
                } else {
                    Color::Red
                };
                spans.push(Span::styled(
                    if goal_chars[i] != ' ' {
                        goal_chars[i].to_string()
                    } else {
                        ch.to_string()
                    },
                    Style::default().fg(color),
                ))
            }
            spans.push(Span::styled(
                &self.goal[self.current.len()..],
                Style::default().fg(Color::Gray),
            ));
            spans
        }
    }
}
