use std::{collections::HashSet, fs::File, io::Write, path::Path};

use zip::{ZipWriter, result::ZipResult, write::SimpleFileOptions};

pub struct ResourcePackWriter {
    writer: ZipWriter<File>,
    sound_list: HashSet<String>,
}

impl ResourcePackWriter {
    pub fn new(writer: ZipWriter<File>) -> Self {
        Self {
            writer,
            sound_list: Default::default(),
        }
    }

    pub fn write_sound_file(
        &mut self,
        pokemon_name: &str,
        form_name: Option<&str>,
        sound_data: &[u8],
        dest_path: impl AsRef<Path>,
    ) -> ZipResult<()> {
        self.writer
            .start_file_from_path(&dest_path, SimpleFileOptions::default())?;
        self.writer.write_all(sound_data)?;
        let sound_file_name: String = form_name
            .map(|form_name| format!("{pokemon_name}-{form_name}.ogg"))
            .unwrap_or_else(|| format!("{pokemon_name}.ogg"));
        self.sound_list.insert(sound_file_name);
        Ok(())
    }

    pub fn into_inner(self) -> (ZipWriter<File>, HashSet<String>) {
        (self.writer, self.sound_list)
    }
}
