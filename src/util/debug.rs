use std::collections::LinkedList;

use chrono::{DateTime, Local, TimeZone};
use uuid::Uuid;

struct TEPILogger {
    id: Uuid,
    buffer: LinkedList<TEPILogMsg>,
    name: String,
    file_location: String,
    buffer_amount: u16,
    date_started: DateTime<Local>,
    console_log: bool
}

struct TEPILogMsg {
    timestamp: DateTime<Local>,
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
    pub fn new(name: String, fl: String, buffer: Option<u16>, console_log: bool) -> TEPILogger {
        // for now we will write to a txt file but will do something more compressed
        let mut b: u16 = 16;
        if let Some(b) = buffer {};
        let d = Local::now();
        TEPILogger {
            id: Uuid::new_v4(),
            buffer: LinkedList::new(),
            name: name,
            file_location: fl,
            buffer_amount: b,
            date_started: d,
            console_log: console_log
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
            timestamp: Local::now(),
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
        if self.buffer.len() < self.buffer_amount as usize {
            return
        }

        // payload is what we're writing to the file. map is a simplified foreach loop
        // they're inserted into a Vec and then combined into one string.
        let payload = self.buffer.iter()
            .map(|i| i.fmt_log_string())
            .collect::<Vec<String>>()
            .join("");

        // write payload to associated file
    }

    /// write_header
    /// creates a header for use
    /// Log: LogName
    /// Date started:
    /// Time started:
    /// Log Version: 1
    fn write_header(&self) -> String {
        let date = self.date_started.format("%Y-%m-%d").to_string();
        let time = self.date_started.format("%H:%M:%s");
        let version: u8 = 1;
        return format!("Log: {}\nDate Started: {}\nTime Started: {}\nLog Version: {}", self.name, date, time, version);

    }

    /// read_header
    /// reads in a header.
    /// we might need to make different header versions so we'll start accounting for this
    /// in the read and write by using an integer for version.
    /// until the initial log version is hammered down we will not increment versions.
    fn read_header() {

    }

    /// push
    /// push a new log message
    /// pushes the log message to the back of the buffer and then checks if the buffer is full.
    pub fn push(&mut self, msg: TEPILogMsg) {
        self.buffer.push_back(msg);
        if self.buffer.len() > self.buffer_amount as usize {
            self.update_file()
        }
    }

    /// pop
    /// pops the first log message from the buffer and returns it.
    pub fn pop(&mut self) -> Option<TEPILogMsg> {
        if self.buffer.len() > 0 {
            return self.buffer.pop_front()
        } else {
            return None
        }

    }
}

impl TEPILogMsg {
    /// fmt_log_string
    /// creates a string usable for either writing to a file or a live feed.
    /// example string without filters
    /// [HH:MM:SS][TYPE] message
    /// example string with filters
    /// [HH:MM:SS][TYPE][filter1][filter2] message
    pub fn fmt_log_string(&self) -> String {
        let s = Local::now().format("%H:%M:%s");
        let log_type = TEPILogger::log_type_str(self.log_type, self.log_type_custom.clone());
        let filters = self.fmt_log_filters();
        return format!("\n[{}][{}]{}: {}", s, log_type, filters, &self.message);
    }

    pub fn fmt_log_filters(&self) -> String {
        let s = self.filters.iter().map(|i| i.to_owned()).collect::<Vec<String>>().join("]["); // there was a shorter way to achieve the same thing that I lost
        return format!("[{}]", s);
    }

    pub fn read_msg(msg: String) -> TEPILogMsg {
        let mut m = msg.clone();
        let msg: &mut str = m.as_mut_str();

        let mut words: Vec<String> = Vec::new();
        let mut log_msg: String = String::new();
        let mut inside_bracket: bool = false;
        let mut curr_word: String = String::new();

        // handle the line character by character.
        for c in msg.chars() {
            match c {
                '[' => {
                    // can probably get rid of this conditional since logs aren't really supposed to be tampered with
                    if inside_bracket {
                        if !curr_word.is_empty() {
                            words.push(curr_word.clone());
                            curr_word.clear();
                        }
                    }
                    inside_bracket = true
                }
                ']' => {
                    if inside_bracket {
                        words.push(curr_word.clone());
                        curr_word.clear();
                        inside_bracket = false;
                    } else {
                        
                    }
                }
                _ => {
                    if inside_bracket {
                        curr_word.push(c);
                    } else {
                        log_msg.push(c);
                    }
                }
            };
        }
        // get our filters if they exist
        let mut filters: Vec<String> = Vec::new();
        while words.len() > 2 {
            filters.push(words.pop().expect("missing line info"));
        }
        // we're now at the log type and the timestamp
        let log_type_str = words.pop().expect("unable to find type info");
        let log_type: TEPILogType = TEPILogMsg::match_log_type(log_type_str.clone());
        let log_type_c: Option<String>;
        let filters: LinkedList<String> = filters.into_iter().collect();
        if log_type == TEPILogType::CUSTOM {
            log_type_c = Some(log_type_str);
        } else {
            log_type_c = None
        }
        let time = words.pop().expect("unable to find time info");
        return TEPILogMsg {
            timestamp: DateTime::parse_from_str(time.as_str(), "%H:%M:%S").expect("parse error").into(),
            filters: filters,
            log_type: log_type,
            log_type_custom: log_type_c,
            message: log_msg
        }
    }
    
    fn match_log_type(s: String) -> TEPILogType {
        match s.as_str() {
            "NORMAL" => {return TEPILogType::NORMAL;}
            "WARN" => {return TEPILogType::WARN;}
            "ERROR" => {return TEPILogType::ERROR;}
            _ => {return TEPILogType::CUSTOM;}
        }
    }
}

// UUID info
// let uuid_str = "uuid string from file"
// let parsed_uuid = Uuid::parse_str(uuid_str).expect("unable to parse uuid");
// assign uuid

// Local info from chrono
// get current date/time Local::now()
// customization with % followed by
// Y year
// m month
// d day
// H hour
// M minute
// S second

// formatting a datetime var.format("%Y-%m-%d %H:%M:%s")
// reading the same format: Local.datetime_from_str(var, "%Y-%m-%d %H:%M:%s")