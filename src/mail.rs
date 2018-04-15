use mailstrom::{Mailstrom, Config, MemoryStorage};
use std::sync::{Arc, Mutex};

pub type Mail = Arc<Mutex<Mailstrom<MemoryStorage>>>;

pub fn init() -> Mail {
    let mut mailstrom = Mailstrom::new(
        Config {
            helo_name: "geschenke.oli-obk.de".to_owned(),
            smtp_timeout_secs: 30,
            ..Default::default()
        },
        MemoryStorage::new());
    mailstrom.start().unwrap();
    Arc::new(Mutex::new(mailstrom))
}

