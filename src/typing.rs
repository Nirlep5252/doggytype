pub mod typing {
    use rand_word;
    use std::time::SystemTime;

    pub struct TypingGame {
        pub goal: String,
        pub current: String,
        pub start_time: Option<SystemTime>,
    }

    impl TypingGame {
        pub fn new() -> Self {
            TypingGame {
                goal: rand_word::new(50),
                current: "".to_string(),
                start_time: None,
            }
        }
    }
}
