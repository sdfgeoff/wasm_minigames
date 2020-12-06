use super::map::Map;
use super::ship::Ship;
use std::cmp::Ordering;

pub struct Score {
    pub laps: Vec<f64>,
    previous_progress: f32,
}

impl Score {
    pub fn new() -> Self {
        Self {
            laps: vec![],
            previous_progress: 0.0,
        }
    }

    pub fn reset(&mut self, map: &Map, ship: &Ship) {
        self.laps.clear();

        self.previous_progress =
            map.calc_progress_relative_to_startline((ship.position.x, ship.position.y));
    }

    /// Checks if the player crosses the start/finish line and updates
    /// the score to match
    pub fn update(&mut self, map: &Map, ship: &Ship, time: f64) {
        let current_progress =
            map.calc_progress_relative_to_startline((ship.position.x, ship.position.y));

        // Progress has jumped from previously being near 1.0 (nearly completed)
        // to being near 0.0 (just started), so they probably did a lap
        if self.previous_progress > 0.8 && current_progress < 0.2 {
            self.laps.push(time);
        }

        // Progress has jumped from previously being near 0.0 (just started)
        // to being close to 1.0 (nearly completed) so the player went back
        // across the line.
        if self.previous_progress < 0.2 && current_progress > 0.8 {
            self.laps.pop();
        }

        self.previous_progress = current_progress
    }

    /// Returns a vector of the times for each lap
    pub fn get_lap_timings(&self) -> Vec<f64> {
        let mut lap_times = vec![];
        let mut lap_start_time = 0.0;
        for lap_end_time in &self.laps {
            lap_times.push(lap_end_time - lap_start_time);
            lap_start_time = *lap_end_time;
        }
        // First "lap" is the time it takes to get across
        // the start line
        lap_times.drain(0..1);
        lap_times
    }

    /// Returns the average lap time
    pub fn get_average_lap(&self) -> Option<f64> {
        let lap_timings = self.get_lap_timings();

        if lap_timings.len() > 0 {
            let mut total_time = 0.0;
            for lap_time in &lap_timings {
                total_time += lap_time
            }
            Some(total_time / (lap_timings.len() as f64))
        } else {
            None
        }
    }

    pub fn get_best_lap(&self) -> Option<f64> {
        let mut lap_timings = self.get_lap_timings();

        // Lap timings should never be NAN
        lap_timings.sort_by(|a, b| a.partial_cmp(b).unwrap());
        lap_timings.first().cloned()
    }

    // Compare two scores to see which is better
    pub fn cmp(&self, other: &Self) -> Ordering {
        let a_laps = self.laps.len();
        let b_laps = other.laps.len();
        let a_last_lap = self.laps.last();
        let b_last_lap = other.laps.last();

        if a_laps > b_laps {
            Ordering::Less
        } else if a_laps < b_laps {
            Ordering::Greater
        } else {
            if let Some(a_last_lap) = a_last_lap {
                if let Some(b_last_lap) = b_last_lap {
                    // Both scores show at least one lap, so compare times
                    if a_last_lap > b_last_lap {
                        // A has the longer time, so is doing worse
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                } else {
                    // b has not done any laps
                    Ordering::Less
                }
            } else {
                if b_last_lap.is_some() {
                    // b has done some laps, a has not
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
        }
    }
}
