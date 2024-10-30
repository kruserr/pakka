use serde::{Serialize, Deserialize};
use chrono::Utc;
use std::collections::{HashMap, HashSet};
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::sync::{Mutex, Arc};
use once_cell::sync::OnceCell;

#[derive(Serialize, Deserialize)]
pub enum EventType {
    Install,
    Uninstall,
}

#[derive(Serialize, Deserialize)]
pub struct Event {
    pub timestamp: String,
    pub event_type: EventType,
    pub package_name: String,
    pub package_manager: String,
}

impl Event {
    pub fn new(event_type: EventType, package_name: &str, package_manager: &str) -> Self {
        Event {
            timestamp: Utc::now().to_rfc3339(),
            event_type,
            package_name: package_name.to_string(),
            package_manager: package_manager.to_string(),
        }
    }
}

pub struct EventSourcingDatabase {
    log_file: String,
    file_lock: Mutex<()>,
}

impl EventSourcingDatabase {
    fn new(log_file: &str) -> Arc<Self> {
        Arc::new(Self {
            log_file: log_file.to_string(),
            file_lock: Mutex::new(()),
        })
    }

    pub fn instance() -> Arc<Self> {
        static INSTANCE: OnceCell<Arc<EventSourcingDatabase>> = OnceCell::new();
        INSTANCE.get_or_init(|| EventSourcingDatabase::new("event_log.jsonl")).clone()
    }

    pub fn log_event(&self, event: &Event) {
        let _lock = self.file_lock.lock().unwrap();
        let event_json = serde_json::to_string(event).expect("Failed to serialize event");
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.log_file)
            .expect("Failed to open log file");
        writeln!(file, "{}", event_json).expect("Failed to write to log file");
    }

    pub fn get_installed_packages(&self) -> io::Result<HashSet<String>> {
        let _lock = self.file_lock.lock().unwrap();
        let file = File::open(&self.log_file)?;
        let reader = BufReader::new(file);
        let mut packages = HashSet::new();

        for line in reader.lines() {
            let line = line?;
            let event: Event = serde_json::from_str(&line)?;
            match event.event_type {
                EventType::Install => {
                    packages.insert(event.package_name.clone());
                }
                EventType::Uninstall => {
                    packages.remove(&event.package_name);
                }
            }
        }
        Ok(packages)
    }

    pub fn export_installed_packages(&self, export_file: &str) -> Result<(), std::io::Error> {
        let log_file = "event_log.jsonl";
        let file = File::open(log_file)?;
        let reader = BufReader::new(file);

        let mut installed_packages: HashMap<String, String> = HashMap::new();

        for line in reader.lines() {
            let line = line?;
            let event: Event = serde_json::from_str(&line)?;
            match event.event_type {
                EventType::Install => {
                    installed_packages.insert(event.package_name.clone(), event.package_manager.clone());
                }
                EventType::Uninstall => {
                    installed_packages.remove(&event.package_name);
                }
            }
        }

        let export_file_path = export_file;
        let export_file = File::create(export_file_path)?;
        let mut writer = BufWriter::new(export_file);

        for (package_name, package_manager) in installed_packages.iter() {
            writeln!(writer, "{}\t{}", package_name, package_manager)?;
        }

        println!("Exported installed packages to {:?}", export_file_path);

        Ok(())
    }

    pub fn show_history(&self, date_filter: Option<String>) {
        let _lock = self.file_lock.lock().unwrap();
        let file = File::open(&self.log_file).expect("Failed to open event log file");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.expect("Failed to read line from event log");
            let event: Event = serde_json::from_str(&line).expect("Failed to parse event log line");

            if let Some(ref date) = date_filter {
                if !event.timestamp.starts_with(date) {
                    continue;
                }
            }

            println!(
                "[{}] {} {} via {}",
                event.timestamp,
                match event.event_type {
                    EventType::Install => "Installed",
                    EventType::Uninstall => "Uninstalled",
                },
                event.package_name,
                event.package_manager
            );
        }
    }

    pub fn show_diff(&self, from_date: &str, to_date: &str) {
        let _lock = self.file_lock.lock().unwrap();
        let file = File::open(&self.log_file).expect("Failed to open event log file");
        let reader = BufReader::new(file);

        let from_time = chrono::NaiveDate::parse_from_str(from_date, "%Y-%m-%d")
            .expect("Invalid from date format")
            .and_hms(0, 0, 0);
        let to_time = chrono::NaiveDate::parse_from_str(to_date, "%Y-%m-%d")
            .expect("Invalid to date format")
            .and_hms(23, 59, 59);

        let mut packages = HashSet::new();
        let mut packages_at_from = HashSet::new();
        let mut packages_at_to = HashSet::new();

        for line in reader.lines() {
            let line = line.expect("Failed to read line from event log");
            let event: Event = serde_json::from_str(&line).expect("Failed to parse event log line");

            let event_time = chrono::DateTime::parse_from_rfc3339(&event.timestamp)
                .expect("Invalid timestamp format")
                .naive_local();

            if event_time <= from_time {
                match event.event_type {
                    EventType::Install => {
                        packages.insert(event.package_name.clone());
                    }
                    EventType::Uninstall => {
                        packages.remove(&event.package_name);
                    }
                }
                packages_at_from = packages.clone();
            }

            if event_time <= to_time {
                match event.event_type {
                    EventType::Install => {
                        packages.insert(event.package_name.clone());
                    }
                    EventType::Uninstall => {
                        packages.remove(&event.package_name);
                    }
                }
                packages_at_to = packages.clone();
            } else {
                break;
            }
        }

        let installed_in_interval = packages_at_to.difference(&packages_at_from);
        let uninstalled_in_interval = packages_at_from.difference(&packages_at_to);

        println!("Packages installed between {} and {}:", from_date, to_date);
        for pkg in installed_in_interval {
            println!("+ {}", pkg);
        }

        println!("Packages uninstalled between {} and {}:", from_date, to_date);
        for pkg in uninstalled_in_interval {
            println!("- {}", pkg);
        }
    }
}