use std::{
    fs::{File, read_to_string},
    sync::Arc,
};

use tokio::io;

#[derive(Default, Debug)]
pub struct EditorState {
    lines: Vec<String>,
    cursor_pos: (u32, u32),
}

impl EditorState {
    pub fn load_lines(&mut self, input_lines: &str) {
        self.lines = input_lines.split("\n").map(String::from).collect();
    }

    pub fn write_line(&mut self, char:&str) {
        let (pos, selected_line) = self.cursor_pos;
        let line = self.lines.get(selected_line as usize);
        if let Some(line) =  line {
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
        if let Some(line) =  line {
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
}

pub async fn load_file(path: String) -> Result<Arc<String>, io::ErrorKind> {
    println!("{path}");
    read_to_string(path)
        .map(Arc::new)
        .map_err(|error| error.kind())
}
