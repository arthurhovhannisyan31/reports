use std::io::Read;

#[derive(Debug, Default)]
pub struct TextReportReader {
  data: Vec<u8>,
  position: usize,
}

impl TextReportReader {
  pub fn new(data: Vec<u8>) -> Self {
    Self { data, position: 0 }
  }
}

impl Read for TextReportReader {
  fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
    let bytes_to_read = (self.data.len() - self.position).min(buf.len());

    if bytes_to_read > 0 {
      buf[..bytes_to_read].copy_from_slice(
        &self.data[self.position..self.position + bytes_to_read],
      );
      self.position += bytes_to_read;
    }

    Ok(bytes_to_read)
  }
}
