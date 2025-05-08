use std::{fs, io, sync::Arc};

pub struct Reader;

impl Reader {
    pub fn read_file(path: &str) -> io::Result<Arc<[u8]>> {
        let vec = fs::read(path)?;
        let arc = Arc::from(vec); // 🔥 zero-copy takeover
        Ok(arc)
    }
}
