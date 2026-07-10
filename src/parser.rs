use crate::log::LogEntry;

pub fn count_logs(entries: &[LogEntry]) -> (usize, usize, usize) {
    let mut info = 0;
    let mut warning = 0;
    let mut error = 0;

    for entry in entries {
        match entry.level.as_str() {
            "INFO" => info += 1,
            "WARNING" => warning += 1,
            "ERROR" => error += 1,
            _ => {}
        }
    }

    (info, warning, error)
}
