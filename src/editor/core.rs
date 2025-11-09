use std::{fs::{read_to_string, File}, io::Write, path::Path, sync::Arc};

use tokio::io;

#[derive(Default, Debug)]
pub struct EditorState {
    lines: Vec<String>,
}

impl EditorState {
    pub fn load_lines(&mut self, input_lines: &str) {
        self.lines = input_lines.split("\n").map(String::from).collect();
    }

    pub fn get_lines(&self) -> &Vec<String> {
        &self.lines
    }
    
}

pub async fn load_file(path: String ) -> Result<Arc<String>, io::ErrorKind> {
    println!("{path}");
    read_to_string(path)
        .map(Arc::new)
        .map_err(|error| error.kind())
}
