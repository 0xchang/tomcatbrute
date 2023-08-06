use base64::{engine::general_purpose, Engine};

pub fn encode(s: &str) -> String {
    let b64s = general_purpose::STANDARD.encode(s);
    b64s
}
