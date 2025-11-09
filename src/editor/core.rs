use std::{
    fs::{self, read_to_string},
    sync::Arc,
};

use tokio::io;

#[derive(Default, Debug)]
pub struct EditorState {
    lines: Vec<String>,
    cursor_pos: (i32, i32),
}

impl EditorState {
    pub fn load_lines(&mut self, input_lines: &str) {
        self.lines = input_lines.split("\n").map(String::from).collect();
    }

    pub fn write_line(&mut self, char: &str) {
        let (pos, selected_line) = self.cursor_pos;
        let line = self.lines.get(selected_line as usize);
        if let Some(line) = line {
            let mut before = line.to_owned();
            let after = before.split_off(pos as usize);
            before.push_str(char);
            before.push_str(&after);

            self.lines[selected_line as usize] = before;
        }
        self.cursor_pos = (pos + 1, selected_line);
    }

    pub fn backspace(&mut self) {
        let (pos, selected_line) = self.cursor_pos;
        let line = self.lines.get(selected_line as usize);
        if let Some(line) = line {
            let mut before = line.to_owned();
            let after = before.split_off(pos as usize);
            before.pop();
            before.push_str(&after);

            self.lines[selected_line as usize] = before;
        }
        if pos > 0 {
            self.cursor_pos = (pos - 1, selected_line);
        }
    }

    pub fn get_lines(&self) -> &Vec<String> {
        &self.lines
    }

    pub fn move_cursor(&mut self, offsets: (i32, i32)) {
        let (old_x, old_line) = self.cursor_pos;
        let (off_x, off_line) = offsets;

        let (mut new_x, mut new_line) = (old_x + off_x, old_line + off_line);

        if new_x < 0 {
            new_x = 0;
        }

        if new_line < 0 {
            new_line = 0;
        }

        new_line = new_line.min(self.lines.len() as i32 - 1);
        let line = self.lines.get(old_line as usize);
        if let Some(s) = line {
            new_x = new_x.min(s.len() as i32 - 1);
        }

        self.cursor_pos = (new_x, new_line);
    }
}

pub async fn write_to_file(lines: Vec<String>, path: String) -> Result<(), io::Error> {
    let bytes: Vec<u8> = lines.join("\n").bytes().collect();
    fs::write(path, bytes)?;
    Ok(())
}

pub async fn load_file(path: String) -> Result<Arc<String>, io::ErrorKind> {
    println!("{path}");
    read_to_string(path)
        .map(Arc::new)
        .map_err(|error| error.kind())
}
