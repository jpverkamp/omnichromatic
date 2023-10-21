use std::{process::{Command, Stdio, ChildStdout, ChildStdin}, io::{Write, BufRead}};
use anyhow::Result;

pub struct Provider { 
    reader: std::io::BufReader<ChildStdout>,
    writer: std::io::BufWriter<ChildStdin>,
}

impl Provider { 
    pub fn new(cmd: String) -> Result<Provider> { 
        let mut command = Command::new("bash")
            .arg("-c")
            .arg(cmd)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let stdin = command.stdin.take().unwrap();
        let writer = std::io::BufWriter::new(stdin);

        let stdout = command.stdout.take().unwrap();
        let reader = std::io::BufReader::new(stdout);

        Ok(Provider { 
            reader,
            writer,
        })
    }

    pub fn send(&mut self, cmd: String) -> Result<()> { 
        log::debug!("SEND {:p}: {}", &self, cmd);

        self.writer.write(cmd.as_bytes())?;
        self.writer.write("\n".as_bytes())?;
        self.writer.flush()?;

        Ok(())
    }

    pub fn ask(&mut self, cmd: String, answer: &str) -> Result<Vec<String>> { 
        self.send(cmd)?;

        loop {
            let mut line = String::new();
            self.reader.read_line(&mut line)?;

            // TODO: Why is this so ugly to strip newline?
            line = line.strip_suffix("\n").unwrap_or(line.as_str()).to_string();
            
            if line == "" {
                continue;
            }

            log::debug!("RECV {:p}: {}", &self, line);

            // Allow providers to send clients that will be logged then ignored
            if line.starts_with("#") {
                continue;
            }

            let mut parts = line
                .split_whitespace()
                .map(|s| String::from(s))
                .collect::<Vec<String>>();

            // TODO: Allow the requester to send commands back and handle those
            // Example, ask for a color, client asks for the neighboring color back: 
            // SEND 5: get-color 10 10
            // RECV 5: get-color 10 11
            // SEND 5: color 255 255 255
            // RECV 5: color 255 255 254
            if parts[0] != answer {
                log::warn!("RECV {:p}: Expected {}, got {}, response ignored", &self, answer, parts[0]);
                continue
            }

            parts.remove(0);
            return Ok(parts);
        }
    }
}