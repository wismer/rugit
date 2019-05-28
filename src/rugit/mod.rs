pub mod workspace {
  use std::fs;
  use std::path::Path;
  use sha1::Sha1;

  use flate2::Compression;
  use flate2::write::ZlibEncoder;
  use std::io::prelude::*;

  pub struct Workspace {
    pub path: String
  }

  impl Workspace {
    pub fn current_directory(&self) {
      println!("{:?}", &self.path);
    }

    pub fn to_path(&self) -> &Path {
      Path::new(&self.path)
    }

    pub fn process_file(&self, filepath: &Path) -> Result<String, std::io::Error> {
      let mut buffer = String::new();
      buffer.push_str("object ");
      println!("ATTEMPT TO READ");
      let filecontents = fs::read_to_string(filepath)?;
      println!("READ");
      let bytesize = filecontents.len();
      buffer.push_str(&bytesize.to_string());
      buffer.push_str("\0");
      buffer.push_str(&filecontents);

      // let sha = Sha1::from(buffer.into_bytes());
      let mut sha = Sha1::new();
      let data = &buffer.into_bytes();
      sha.update(data);
      let sha_hash = sha.digest().to_string();

      let mut compressor = ZlibEncoder::new(Vec::new(), Compression::default());
      compressor.write_all(data);
      let bytes = compressor.finish();
      println!("compresssor: {:?}", bytes);

      Ok(sha_hash)
    }

    pub fn files(&self) -> std::io::Result<()> {
      let path = Path::new(&self.path);
      for entry in fs::read_dir(&path)? {
        println!("{:?}", entry);
        // determine the type
        // read the file into a string
        // bytesize
        // null byte
        // contents
        // so - {type} {bytesize}{null-byte}{contents}
        // then digest into sha-1 hash
        // compress it
        // write to.... /objects/first-two-bytes/remaining-hash
      }

      Ok(())
    }
  }
}
