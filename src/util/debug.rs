use std::{collections::LinkedList, time::SystemTime};

struct TEPILogger {
    id: u32,
    buffer: LinkedList<TEPILogMsg>,
    name: String,
    file_location: String,
    buffer_amount: u16
}

struct TEPILogMsg {
    timestamp: SystemTime,
    log_type: TEPILogType,
    filters: LinkedList<String>,
    message: String,
    log_type_custom: Option<String>
}

#[derive(PartialEq, Clone, Copy)]
enum TEPILogType {
    NORMAL,
    WARN,
    ERROR,
    CUSTOM
}

impl TEPILogger {
    pub fn new(name: String, fl: String, buffer: Option<u16>) -> TEPILogger {
        // for now we will write to a txt file but will do something more compressed
        let mut b: u16 = 16;
        if let Some(b) = buffer {};
        TEPILogger {
            id: 1,
            buffer: LinkedList::new(),
            name: name,
            file_location: fl,
            buffer_amount: b
        }
    }

    pub fn log(&mut self, log_type: TEPILogType, message: String, filters: Option<Vec<String>>, flag: Option<String>) {
        let mut f: LinkedList<String> = LinkedList::new();
        if filters.is_some() {
            for s in filters.expect("no filters found") {
                f.push_back(s)
            }
        }
        self.buffer.push_back(TEPILogMsg {
            timestamp: SystemTime::now(),
            log_type: log_type,
            filters: f,
            message: message,
            log_type_custom: flag
        });

        self.update_file()
    }

    pub fn log_normal(&mut self, message: String, filters: Option<Vec<String>>) {
        self.log(TEPILogType::NORMAL, message, filters, None)
    }

    pub fn log_warn(&mut self, message: String, filters: Option<Vec<String>>) {
        self.log(TEPILogType::WARN, message, filters, None)
    }

    pub fn log_error(&mut self, message: String, filters: Option<Vec<String>>) {
        self.log(TEPILogType::ERROR, message, filters, None)
    }

    /// log_custom
    /// instead of normal, warn, error appearing after the timestamp the custom flag 
    pub fn log_custom(&mut self, message: String, filters: Option<Vec<String>>, flag: String) {
        self.log(TEPILogType::CUSTOM, message, filters, Some(flag))
    }

    // lined out for readability
    /// fmt_log_string
    /// creates the string for the log entry
    fn fmt_log_string(msg: TEPILogMsg) -> String {
        return String::new();
    }

    pub fn log_type_str(log_type: TEPILogType, flag: Option<String>) -> String {
        // change to case/switch if available
        if log_type == TEPILogType::NORMAL {
            return "NORMAL".to_string()
        } else if log_type == TEPILogType::WARN {
            return "WARN".to_string()
        } else if log_type == TEPILogType::ERROR {
            return "ERROR".to_string()
        } else {
            let s = flag.expect("missing string");
            return s
        }
    }

    fn update_file(&self) {

    }
}

impl TEPILogMsg {
    pub fn fmt_log_string(&self) -> String{
        let s = "timestamp";
        let log_type = TEPILogger::log_type_str(self.log_type, self.log_type_custom.clone());
        let text = format!("\n[{s}][{log_type}]: {}", &self.message);

        return s.to_string()
    }
}