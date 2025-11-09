use std::{fs::File, io::Write};

#[derive(Default)]
pub struct EditorState {
    current_file: Option<File>,
    lines: Vec<String>,
}

impl EditorState {
    pub async fn write_to_file(&mut self) -> std::io::Result<()>{
        if let Some(file) = &mut self.current_file {
            let s = self.lines.join("\n");
            let to_write = s.as_bytes();
            file.write_all(to_write);
            Ok(())
        } else {
            let mut file = File::create("foo.txt")?;
            let s = self.lines.join("\n");
            let to_write = s.as_bytes();
            file.write_all(to_write);
            self.current_file = Some(file);
            Ok(())
        }
    }
    
}
