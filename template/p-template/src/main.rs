#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::io::{stdin, stdout, BufWriter, Write};

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

#[allow(unused_must_use)]
pub fn main() {
    let mut input = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    for _ in 0..input.next() {
        let n: usize = input.next();
        let v: Vec<i32> = (0..n).map(|_| input.next()).collect();
        writeln!(
            out,
            "{}",
            v.iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    use std::io::Write;
    use std::process::{Command, Stdio};

    fn run_with_input(input: &str) -> String {
        // Simulate stdin by creating a pipe to the child process
        let mut child = Command::new("cargo")
            .arg("run")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start the process");

        {
            // Write input to stdin
            let stdin = child.stdin.as_mut().expect("Failed to open stdin");
            stdin
                .write_all(input.as_bytes())
                .expect("Failed to write to stdin");
        }

        // Capture the output from stdout
        let output = child.wait_with_output().expect("Failed to read stdout");
        String::from_utf8_lossy(&output.stdout).to_string()
    }

    fn remove_spaces_around_newlines(s: &str) -> String {
        let re = Regex::new(r" *\n *").unwrap();
        re.replace_all(s, "\n").into_owned() // Ensure it returns a String
    }

    #[test]
    fn test_case_1() {
        let input = "
            2
            3
            1 2 3
            5
            6 3 1 6 3
            ";
        let expected_output = "
            1 2 3
            6 3 1 6 3
            ";
        let actual_output = remove_spaces_around_newlines(&run_with_input(input));
        let expected_output = remove_spaces_around_newlines(expected_output);
        assert_eq!(actual_output.trim(), expected_output.trim());
    }
}
