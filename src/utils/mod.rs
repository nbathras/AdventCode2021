use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path
};

pub fn get_lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub struct Input {
    reader: BufReader<File>,
}

impl Input {
    pub fn init(file_name: &str) -> Input {
        let file = File::open(file_name).expect("file could not be found/opened in current directory!");
        Input {
            reader: BufReader::new(file),
        }
    }

    pub fn read_line(&mut self) -> String {
        let mut tmp_line = String::new();
        self.reader.read_line(&mut tmp_line).expect("Could not read line from file!");
        tmp_line
    }
    
    pub fn read_all(&mut self) -> String {
        let mut tmp_text = String::new();
        self.reader.read_to_string(&mut tmp_text).expect("Could not read all lines from file!");
        tmp_text
    }

    pub fn split_and_parse<T: std::str::FromStr>(string: String, pattern: &str) -> Vec<T>
        where <T as std::str::FromStr>::Err: std::fmt::Debug {
        string.trim_end()
              .split(pattern)
              .map(|s| {
                  s.trim_end().parse::<T>().expect("Could not parse element in file!")
               })
               .collect()
    }
}