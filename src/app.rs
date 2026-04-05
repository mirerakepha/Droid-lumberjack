/*
 * Store detections for ui
 */

use crate::parser::{Detection, Severity};
use std::collections::HashMap;

pub struct App {
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
        }
    }

    pub fn add_detection(&mut self, detection: Detections){

        let key = detection.name.clone();

        // increment count
        let count = self.counts.entry(key.clone()).or_insert(0);
        *count += 1;

        // remove old instance
        self.detections.retain(|d| d.name != key);

        // insert newest at top
        self.detections.insert(0, detection);

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

        for d in &self.detection {
            match d.severity {
                Severiry::Critical => critical += 1,
                Severiry::Warning => warning += 1,
                Severiry::Info => info += 1,
            }
        } 
        (critical, warning, info)
    }
}
