use serde::Serialize;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Serialize)]
struct LogEntry<T: Serialize> {
    timestamp: u64,
    message: T,
}

pub fn write_log<T: Serialize>(value: &T) {
    let entry = LogEntry {
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64,
        message: value,
    };

    let ron_str = ron::ser::to_string_pretty(&entry, ron::ser::PrettyConfig::default())
        .unwrap_or_else(|e| format!("RON serialization error: {}", e));

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("debug.ron")
        .unwrap();

    writeln!(file, "{}\n", ron_str).unwrap();
}

#[macro_export]
macro_rules! flog_ron {
    ($val:expr) => {{
        $crate::utils::logging::write_log(&$val);
    }};
}

#[macro_export]
macro_rules! flog {
    ($($arg:tt)*) => {{
        use std::io::Write;
        let mut f = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("debug.log")
            .unwrap();
        writeln!(f, $($arg)*).unwrap();
    }};
}
