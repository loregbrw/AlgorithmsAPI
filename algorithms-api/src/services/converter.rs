pub struct Converter;

const GB_IN_BYTES: f64 = 1_073_741_824 as f64;

impl Converter {
    pub fn seconds_to_format(seconds: u64) -> String {

        let days = seconds / 86400;
        let hours = (seconds % 86400) / 3600;
        let minutes = (seconds % 3600) / 60;
        let seconds = seconds % 60;

        format!("{:02}:{:02}:{:02}:{:02}", days, hours, minutes, seconds)
    }

    pub fn byte_to_gb(bytes: u64) -> f64 {
        (bytes as f64) / GB_IN_BYTES
    }
}
