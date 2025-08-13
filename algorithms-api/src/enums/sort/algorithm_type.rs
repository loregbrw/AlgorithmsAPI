pub enum AlgorithmType {
    BUBBLE
}

impl AlgorithmType {
    pub fn from_string(string: &str) -> Option<Self> {
        match string.to_lowercase().as_str() {
            "bubble" | "bubble-sort" => Some(Self::BUBBLE),
            _ => None
        }
    }
}