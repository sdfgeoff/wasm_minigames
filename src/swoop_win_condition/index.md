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


<canvas id="swoop_win_condition"></canvas>


