/*
 * Store detections for ui
 */

use crate::parser::{Detection, Severity};
use std::collections::HashMap;

pub struct App {
    pub raw_logs: Vec<String>,
    pub detections: Vec<Detection>,
    pub counts: HashMap<String, usize>,
    pub selected: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            detections: Vec::new(),
            counts: HashMap::new(),
            selected: 0,
            raw_logs: Vec::new(),
        }
    }

    //Raw Logs 
    pub fn add_raw(&mut self, line: String) {
        self.raw_logs.insert(0, line);

        if self.raw_logs.len() > 50 {
            self.raw_logs.pop();
        }
    }

    pub fn add_detection(&mut self, detection: Detection){

        let key = detection.name.clone();

        // increment count
        let count = self.counts.entry(key.clone()).or_insert(0);
        *count += 1;

        // remove old instance
        self.detections.retain(|d| d.name != key);

        // insert newest at top
        self.detections.insert(0, detection.clone());

        self.detections.push(detection);
    }

    pub fn next(&mut self) {
        if self.selected < self.detections.len() -1 {
            self.selected += 1;
        }
    }

    pub fn previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn severity_counts(&self) -> (usize, usize, usize) {
        let mut critical = 0;
        let mut warning = 0;
        let mut info = 0;

        for d in &self.detections {
            match d.severity {
                Severity::Critical => critical += 1,
                Severity::Warning => warning += 1,
                Severity::Info => info += 1,
            }
        } 
        (critical, warning, info)
    }
}
