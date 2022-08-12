# Win Condition

We can now count laps, but the lap counter goes up indefinitely.
At some point teh game should end and you should see a more
detailed scoreboard - things like best lap time and average lap time.

Let's put a new state in the GameState enum:
```rust
enum GameState {
    Menu,
    Playing,
    ScoreScreen,
}
```

And a function to the gameplay struct to check if the game is
completed:

```rust
    /// Returns True when the game is complete.
    /// The game is considered complete when everyone has
    /// done enough laps
    pub fn game_complete(&self) -> bool {
        for score in self.scores.iter() {
            if score.laps.len() < NUM_LAPS_TO_WIN {
                return false
            }
        }
        return true
    }
```

And in the App struct's play_game function, we can now initiate
the state change:
```rust
    pub fn play_game(&mut self, dt: f64) {
        self.gameplay.update(dt, &self.key_map);
        let ship_entity_refs = self.gameplay.ship_entities.iter().collect();
        let trail_entity_refs = self.gameplay.trails.iter().collect();

        self.renderer.render(
            &self.gameplay.camera.get_camera_matrix(),
            ship_entity_refs,
            trail_entity_refs,
            self.gameplay.get_text_entities(),
        );

        // If the game is finished, show the score screen
        if self.gameplay.game_complete() {
            self.game_state = GameState::ScoreScreen;
        }
    }
```

And we can write another function that runs in the `ScoreScreen` state:

```rust
pub fn show_scores(&mut self, dt: f64) {
        self.gameplay.update(dt * 0.1, &self.key_map);
        let ship_entity_refs = self.gameplay.ship_entities.iter().collect();
        let trail_entity_refs = self.gameplay.trails.iter().collect();

        self.renderer.render(
            &self.gameplay.camera.get_camera_matrix(),
            ship_entity_refs,
            trail_entity_refs,
            self.score_screen.get_text_entities(),
        );

        // If the game is finished, show the score screen
        if self.key_map.start_game == KeyState::JustReleased {
            self.game_state = GameState::Menu;
            self.reset();
        }
    }
```
You may notice something strange there. I'm still calling `self.gameplay.update`
and I'm still rendering the ship entities and trails. That's because I think
it would be cool for the game to continue slow-motion in the background.

I have, of course, created a ScoreScreen struct that contains the text
entities. There's not much particularly complex in it except for the
code that populates the score screen text:
```rust
impl ScoreScreen {

    <<< snip >>>

    pub fn populate_scores(&mut self, ships: &Vec<Ship>, scores: &Vec<Score>) {
        self.scores.clear();

        let mut ship_and_score_refs: Vec<(&Ship, &Score)> =
            ships.iter().zip(scores.iter()).collect();
        ship_and_score_refs.sort_by(|a, b| a.1.cmp(b.1));

        self.scores.append_string("   Avg   Best", &[0.5, 0.5, 0.5]);

        for (ship, score) in ship_and_score_refs {
            let color = [ship.color.0, ship.color.1, ship.color.2];
            
            let best_lap = score.get_best_lap();
            let average_lap = score.get_average_lap();
            
            self.scores.append_string("~ ", &color);
            self.scores.append_string(&format_time(average_lap), &color);
            self.scores.append_string(" ", &color);
            self.scores.append_string(&format_time(best_lap), &color);
        }
    }

}

fn format_time(time: Option<f64>) -> String {
    if let Some(sec) = time {
        let seconds = sec as u32;
        let millis = (sec.fract() * 100.0).floor() as u32;
        format!("{:02}:{:02}", seconds, millis)
    } else {
        "--:--".to_string()
    }
}
```

Not to much complex there, just fetch the average and best lap times for
each player. Oops, better implement those:
```rust
impl Score {

    <<< snip >>>
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
}
```

Remember how the scores are stored as the times that the player
crosses the line? This means that we need the function `get_lap_timings()` 
to get the duration for each lap.

And that's about it really for the score screen.

<canvas id="swoop/swoop_win_condition"></canvas>


