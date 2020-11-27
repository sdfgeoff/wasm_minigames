# Counting Laps

The main purpose of this game is to fly the ship around the map and
be faster than the AI's. The game needs to keep track of the laps so
that it an tell who wins.

So how can we count laps? Well, we can check if the player crosses
the start line. How do we know when this happens? Well, if we have the
transform of the start line we can check which side of the start line
a ship is currently on. Then we can track when it changes side.

So let's create a struct to represent the score:
```rust
struct Score {
    laps: Vec<f64>,
    previous_progress: f32
}
```

Why is laps a vector of f64's? Surely it should be an integer of some
sort? Well, we may as well store the time in which the lap is completed.
The lap count is easily derived using `score.laps.len()` and as a bonus
the time difference between players is easily calculable.

And the previous_progress is to store a value telling how far around
the track the player is. Assuming we have a way to tell how far around
they are, we can do:

```rust
    /// Checks if the player crosses the start/finish line and updates
    /// the score to match
    pub fn update(&self, map: &Map, ship: &Ship, time: f64) {
        let current_progress = map.calc_position_on_track((ship.position.x, ship.position.y));
        
        // Progress has jumped from previously being near 1.0 (nearly completed)
        // to being near 0.0 (just started), so they probably did a lap
        if self.previous_progress > 0.8 && current_progress < 0.2{
            self.laps.push(time);
        }
        
        // Progress has jumped from previously being near 0.0 (just started)
        // to being close to 1.0 (nearly completed) so the player went back
        // across the line.
        if self.previous_progress < 0.2 && current_progress > 0.8{
            self.laps.pop();
        }
        
        self.previous_progress = current_progress
    }
```

This uses a mythical `map.calc_position_on_track` function that takes
a position in global coordinates and returns a single floating point
number that is zero at the start line. A good first approximation to
this function is the angular position on the track of the ship relative
to the start line.

```rust


```



<canvas id="swoop_counting_laps"></canvas>


