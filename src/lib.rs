use std::collections::VecDeque;

pub fn add_command(history: &mut VecDeque<String>, command: String, k: usize) {
    history.push_back(command);
    while history.len() > k {
        history.pop_front();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filling_and_overflow() {
        let mut history = VecDeque::new();

        add_command(&mut history, "ls".to_string(), 2);
        assert_eq!(history, VecDeque::from(vec!["ls".to_string()]));

        add_command(&mut history, "cd src".to_string(), 2);
        assert_eq!(
            history,
            VecDeque::from(vec!["ls".to_string(), "cd src".to_string()])
        );

        // История заполнена (k = 2), самая старая команда ("ls") должна удалиться
        add_command(&mut history, "cargo test".to_string(), 2);
        assert_eq!(
            history,
            VecDeque::from(vec!["cd src".to_string(), "cargo test".to_string()])
        );
    }

    #[test]
    fn test_zero_capacity() {
        let mut history = VecDeque::new();
        add_command(&mut history, "pwd".to_string(), 0);
        assert!(history.is_empty());
    }
}
