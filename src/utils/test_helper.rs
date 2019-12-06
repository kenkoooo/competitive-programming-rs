use crate::utils::scanner::IO;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::str;

pub(crate) struct Tester {
    input: Vec<Vec<u8>>,
    output: Vec<Vec<u8>>,
}

impl Tester {
    pub(crate) fn new(input_directory: &str, output_directory: &str) -> Tester {
        let input = read_from_directory(input_directory);
        let output = read_from_directory(output_directory);
        Tester { input, output }
    }

    pub(crate) fn test_solution<F>(self, solution: F)
    where
        F: Fn(&mut IO<&[u8], &mut Vec<u8>>),
    {
        for (input, output) in self.input.into_iter().zip(self.output) {
            let mut writer = vec![];
            {
                let mut sc = IO::new(&input[..], &mut writer);
                solution(&mut sc);
            }
            assert_eq!(writer, output);
        }
    }
}

fn read_file(filepath: &str) -> Vec<u8> {
    let mut file = File::open(filepath).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    buf
}

fn read_from_directory(directory_path: &str) -> Vec<Vec<u8>> {
    let mut filenames: Vec<String> = fs::read_dir(directory_path)
        .unwrap()
        .map(|result| result.unwrap().path().display().to_string())
        .collect();
    filenames.sort();
    filenames.into_iter().map(|file| read_file(&file)).collect()
}
