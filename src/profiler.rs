extern crate time;

use self::time::{Duration, PreciseTime};



/// A profiler that makes it easy to profile time and duration.
/// The profiler may be started when an expensive function is invoked,
/// and stopped when it completes. The profiler then makes it easy to report
/// the time that has past.
pub struct Profiler {
    /// Total duration.
    duration: Duration,

    /// A precise timer that is used to actuall profile time.
    timer: Option<PreciseTime>,
}

impl Profiler {
    /// Create a new profiler.
    pub fn new(start: bool) -> Profiler {
        // Instantiate the profiler
        let mut profiler = Profiler {
            duration: Duration::zero(),
            timer: None,
        };

        // Start
        if start {
            profiler.start();
        }

        profiler
    }

    /// Check whether the timer is running.
    pub fn running(&self) -> bool {
        self.timer.is_some()
    }

    /// Stop the profiler.
    pub fn start(&mut self) {
        // Set the timer if it isn't already running
        if self.timer.is_none() {
            self.timer = Some(PreciseTime::now());
        }
    }

    /// Stop the profiler.
    pub fn stop(&mut self) {
        // Add the time to the duration
        if let Some(timer) = self.timer.take() {
            self.duration = self.duration + timer.to(PreciseTime::now());
        }
    }

    /// Determine the total duration this profiler was running.
    /// If the profiler is still running, the duration until now is returned.
    pub fn duration(&mut self) -> Duration {
        // Restart the timer ifmut  it is currently running,
        // to update the duration
        if self.running() {
            self.stop();
            self.start();
        }

        // Return the duration
        self.duration
    }

    /// Determine the total duration this profiler was running.
    /// If the profiler is still running, the duration until now is returned.
    ///
    /// The returned duration is formatted in a human readable string.
    pub fn duration_format(&mut self) -> String {
        // Determine the duration
        let duration = self.duration();

        // Format the duration and return
        self.format(&duration)
    }

    /// Report the profiler duration with the given `description`.
    pub fn report(&mut self, description: &str) {
        println!("# {} took {}", description, self.duration_format())
    }

    /// Format the given duration in a human readable format.
    // TODO: make this method safe, improve accuracy at high numbers
    fn format(&self, d: &Duration) -> String {
        // Output weeks
        if d.num_weeks() > 0 {
            return format!("{:.2}d", d.num_days() as f64 / 7f64);
        }

        // Output days
        if d.num_days() > 0 {
            return format!("{:.2}d", d.num_hours() as f64 / 24f64);
        }

        // Output hours
        if d.num_hours() > 0 {
            return format!("{:.2}m", d.num_minutes() as f64 / 60f64);
        }

        // Output minutes
        if d.num_minutes() > 0 {
            return format!("{:.2}m", d.num_seconds() as f64 / 60f64);
        }

        // Output seconds
        if d.num_seconds() > 0 {
            return format!("{:.2}s", d.num_milliseconds() as f64 / 1000f64);
        }

        // Output milliseconds
        if d.num_milliseconds() > 0 {
            return format!("{:.2}ms", d.num_microseconds().unwrap() as f64 / 1000f64);
        }

        // Ooutput microseconds
        if d.num_microseconds().unwrap() > 0 {
            return format!("{:.2}μs", d.num_nanoseconds().unwrap() as f64 / 1000f64);
        }

        // Output nanoseconds
        return format!("{}ns", d.num_nanoseconds().unwrap());
    }
}
