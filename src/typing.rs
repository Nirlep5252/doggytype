pub mod typing {
    use ratatui::{
        style::{Color, Style},
        text::Span,
    };
    use std::{collections::HashSet, time::SystemTime};

    #[derive(Debug)]
    pub struct TypingGame {
        pub goal: String,
        pub current: String,
        pub start_time: Option<SystemTime>,
        pub end_time: Option<SystemTime>,
        pub word_count: usize,
        pub mistakes: u32,
        pub mistake_indices: HashSet<usize>,
    }

    impl TypingGame {
        pub fn new(words: usize) -> Self {
            let mut goal = rand_word::new(words);
            let cap = goal.len();

            while goal.contains("º") {
                goal = rand_word::new(words);
            }

            TypingGame {
                goal,
                current: "".to_string(),
                start_time: None,
                end_time: None,
                word_count: words,
                mistakes: 0,
                mistake_indices: HashSet::with_capacity(cap),
            }
        }

        pub fn reset(&mut self) {
            self.current = "".to_string();
            self.start_time = None;

            self.mistake_indices.clear();
            self.mistakes = 0;
        }

        pub fn new_goal(&mut self) {
            self.goal = TypingGame::new(self.word_count).goal;
        }

        pub fn accuracy(&mut self) -> f32 {
            let mut correct_count = 0;
            let goal_chars = self.goal.chars().collect::<Vec<char>>();

            for (i, ch) in self.current.char_indices() {
                if goal_chars[i] == ch {
                    correct_count += 1;
                } else if !self.mistake_indices.contains(&i) {
                    self.mistakes += 1;
                    self.mistake_indices.insert(i);
                }
            }

            let correct = u32::checked_sub(correct_count, self.mistakes).unwrap_or(0);

            // average correctly typed characters + correct but PREVIOUSLY incorrect characters
            // then change it into a percentage
            ((correct + correct_count) as f32 / 2f32 * 100f32) as f32 / self.current.len() as f32
        }

        pub fn wpm(&self) -> f32 {
            self.current.split_whitespace().count() as f32
                / (SystemTime::now()
                    .duration_since(self.start_time.unwrap())
                    .unwrap()
                    .as_secs_f32()
                    / 60f32)
        }

        pub fn curr_spans(&self) -> Vec<Span> {
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
            spans
        }

        pub fn cursor_spans(&self) -> Vec<Span> {
            let mut spans = vec![];
            let goal_chars = self.goal.chars().collect::<Vec<char>>();
            for (i, ch) in self.current.char_indices() {
                let style = if goal_chars[i] == ch {
                    Style::default()
                } else {
                    Style::default().fg(Color::Red)
                };
                spans.push(Span::styled(goal_chars[i].to_string(), style))
            }
            if !spans.is_empty() {
                spans.push(Span::styled("█", Style::default().fg(Color::White)));
            }
            spans
        }

        pub fn spans(&self) -> Vec<Span> {
            let mut spans = self.curr_spans();
            spans.push(Span::styled(
                &self.goal[self.current.len()..],
                Style::default().fg(Color::DarkGray),
            ));
            spans
        }
    }
}
