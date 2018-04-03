use std::io::Read;
use std::str;
use std::fmt;
use std::collections::vec_deque::VecDeque;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::fs;

pub struct TestCaseProducer {
    queue: VecDeque<String>
}

impl TestCaseProducer {
    pub fn new(filepath: &str) -> TestCaseProducer {
        let q = load_test_values(filepath).iter().map(|s| s.to_owned()).collect();
        TestCaseProducer { queue: q }
    }

    pub fn new_from_directory(directory_path: &str) -> TestCaseProducer {
        let mut filenames: Vec<String> = fs::read_dir(directory_path)
            .unwrap()
            .map(|result| result.unwrap().path().display().to_string())
            .collect();
        filenames.sort();

        let q = filenames
            .iter()
            .map(|filename| load_test_values(filename))
            .flat_map(|values| values.into_iter())
            .map(|value| value.to_owned())
            .collect();
        TestCaseProducer { queue: q }
    }

    pub fn next<T>(&mut self) -> T where T: str::FromStr, T::Err: fmt::Debug {
        self.queue.pop_front().unwrap().parse().ok().unwrap()
    }

    pub fn is_empty(&self) -> bool { self.queue.is_empty() }
}

fn load_test_values(filepath: &str) -> Vec<String> {
    let path = Path::new(filepath);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(_) => s.trim()
            .split(|c| c == '\n' || c == ' ')
            .map(|s| s.to_owned())
            .collect::<Vec<String>>()
    }
}
