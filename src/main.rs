mod log;
mod parser;

use log::LogEntry;
use parser::count_logs;

fn main() {

    let logs = vec![
        LogEntry::new("INFO", "Application started"),
        LogEntry::new("INFO", "Configuration loaded"),
        LogEntry::new("WARNING", "Low memory"),
        LogEntry::new("ERROR", "Connection failed"),
        LogEntry::new("ERROR", "Timeout reached"),
    ];

    let (info, warning, error) = count_logs(&logs);

    println!("===== LOG REPORT =====\n");

    println!("INFO: {}", info);
    println!("WARNING: {}", warning);
    println!("ERROR: {}", error);

    println!("\nTotal entries: {}", logs.len());

    println!("\nLog Entries:");

    for entry in &logs {
        println!("[{}] {}", entry.level, entry.message);
    }
}
