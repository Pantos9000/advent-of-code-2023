use crate::part1::{BitSprings, GroupSprings};

use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn run(input: &str) -> usize {
    let pool = ThreadPool::new(input);
    pool.stop_threads()
}

type SpringThread = thread::JoinHandle<usize>;

#[derive(Default)]
struct ThreadPoolState {
    /// threads started / stopped
    state: Arc<Mutex<(usize, usize)>>,
}

impl Clone for ThreadPoolState {
    fn clone(&self) -> Self {
        let state = Arc::clone(&self.state);
        Self { state }
    }
}

impl ThreadPoolState {
    fn register_start(&self) {
        let mut state = self.state.lock().unwrap();
        let (mut threads_started, threads_stopped) = *state;
        threads_started += 1;
        *state = (threads_started, threads_stopped);
        println!("Threads finished: {threads_stopped} / {threads_started}");
    }

    fn register_stop(&self) {
        let mut state = self.state.lock().unwrap();
        let (threads_started, mut threads_stopped) = *state;
        threads_stopped += 1;
        *state = (threads_started, threads_stopped);
        println!("Threads finished: {threads_stopped} / {threads_started}");
    }
}

struct ThreadPool {
    threads: Vec<SpringThread>,
}

impl ThreadPool {
    fn new(input: &str) -> Self {
        let state = ThreadPoolState::default();
        let threads = Self::start_threads(input, state.clone());
        Self { threads }
    }

    fn start_threads(input: &str, state: ThreadPoolState) -> Vec<SpringThread> {
        input
            .lines()
            .map(|line| Self::spawn_thread(line, state.clone()))
            .collect()
    }

    fn spawn_thread(line: &str, state: ThreadPoolState) -> SpringThread {
        let line = line.to_owned();
        thread::spawn(move || {
            state.register_start();
            let ret = count_per_line_with_unfolding(&line);
            state.register_stop();
            ret
        })
    }

    fn stop_threads(self) -> usize {
        self.threads
            .into_iter()
            .map(|thread| thread.join().unwrap())
            .sum()
    }
}

fn count_per_line_with_unfolding(line: &str) -> usize {
    let mut springs = BitSprings::from_str(line).unwrap();
    let mut groups = GroupSprings::from_str(line).unwrap();

    springs.unfold(5);
    groups.unfold(5);

    springs.count_possible_arrangements(groups)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unfold_example_1_springs() {
        let line = ".# 1";

        let mut springs = BitSprings::from_str(line).unwrap();
        springs.unfold(5);
        assert_eq!(format!("{springs:?}"), ".#?.#?.#?.#?.#");
    }

    #[test]
    fn test_unfold_example_1_groups() {
        let line = ".# 1";

        let mut groups = GroupSprings::from_str(line).unwrap();
        groups.unfold(5);
        assert_eq!(format!("{groups:?}"), "1,1,1,1,1");
    }

    #[test]
    fn test_unfold_example_2_springs() {
        let line = "???.### 1,1,3";

        let mut springs = BitSprings::from_str(line).unwrap();
        springs.unfold(5);
        assert_eq!(
            format!("{springs:?}"),
            "???.###????.###????.###????.###????.###"
        );
    }

    #[test]
    fn test_unfold_example2_groups() {
        let line = "???.### 1,1,3";

        let mut groups = GroupSprings::from_str(line).unwrap();
        groups.unfold(5);
        assert_eq!(format!("{groups:?}"), "1,1,3,1,1,3,1,1,3,1,1,3,1,1,3");
    }

    #[test]
    #[should_panic]
    fn test_unfold_springs_panics_if_too_long() {
        let line = "...............";

        let mut groups = GroupSprings::from_str(line).unwrap();
        groups.unfold(8);
    }
}
