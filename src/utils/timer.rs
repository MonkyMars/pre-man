use chrono::{Duration, Utc, DateTime};

pub struct Timer {
    pub start: DateTime<Utc>,
    pub stop: DateTime<Utc>,
    pub duration: Duration,
}

impl Timer {
    pub fn start() -> Self {
        let start = Self::get_current_time();
        Self {
            start,
            stop: start,
            duration: Duration::zero(),
        }
    }

    fn get_current_time() -> DateTime<Utc> {
        Utc::now()
    }

    pub fn stop(&mut self) {
        self.stop = Self::get_current_time();
        let duration = self.stop.signed_duration_since(self.start);
        self.duration = duration;
    }
}
