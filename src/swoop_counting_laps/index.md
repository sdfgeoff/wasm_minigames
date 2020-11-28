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
        let current_progress = map.calc_progress_relative_to_startline((ship.position.x, ship.position.y));
        
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

This uses a mythical `map.calc_progress_relative_to_startline` function that takes
a position in global coordinates and returns a single floating point
number. This number should jump jump from 1.0 to 0.0 when the player crosses the
start line, and can be anything else when the player is a long way away.

You may have expected it to be zero at the start line and go positive/negative when
moving each direction, but then you have to deal with a discontinuity and where the
startline is. By placing the discontinuity at the start line it reduces the number
of edge cases.

So how can we `calc_progress_relative_to_startline`. One way would be to convert
the player position into polar coordinates and use the angle. But this wouldn't
take the "tilt" of the startline into account so would only be accurate for the middle
of the track. But if we transform the ships position into the local coordinate
system of the start line:
```rust
let start_position = self.get_start_position();
let start_line_direction = self.get_track_direction(start_position.angle);

let start_position_cartesian = start_position.to_cartesian();

let distance_from_startline = (
    position.0 - start_position_cartesian.0,
    position.1 - start_position_cartesian.1
);

let s = f32::sin(-start_line_direction);
let c = f32::cos(-start_line_direction);

let position_local = (
    c*distance_from_startline.0 - s*distance_from_startline.1,
    s*distance_from_startline.0 + c*distance_from_startline.1
);
```

We can then check to see if the player is near the start line:

```rust
if f32::abs(position_local.0) > self.track_width {
    // Position is off to the side of the track
    0.5
} else {
    // Offset so that start line is at progress = 1.0
    let progress = position_local.1 + 1.0;

    if progress > 1.5 {
        // Position is a long way in front of the line
        0.5
    } else if progress < 0.5 {
        // Position is a long way behind the line
        0.5
    } else
```

And force the discontinuity to be at 0.0/1.0 boundary
```rust
// Position is near the line. We want the returned
// nunmber to be between 0.0 and 1.0 and the discontinuty
// to be at the start line. Currently `progress` goes
// from 0.5 to 1.5
if progress > 1.0 {
    progress - 1.0
} else {
    progress
}
```

ANd now we can count laps!

# Display a leader board
It would be cool for the player to be able to know how far behind the
leader they are (I have great faith in human's piloting ability...).
In my mind the leaderboard should be structured:
```
LAP 2/3
<SHIP>:  00.00
<SHIP>: +03.13
<SHIP>: +07.32
<SHIP>: +23.50

```
The `<SHIP>` is the ship glyph in the font, and the number is the number of
seconds behind the leader. If the leader is on a different lap, the leaderboard
can display:
```
LAP 2/3
<SHIP>:  00.00
<SHIP>: +03.13
<SHIP>: +--.--
<SHIP>: +--.--
```

We have all the scores in an array, so let's sort a vector of reference to them
in order of lap and then timing:
```

```

<canvas id="swoop_counting_laps"></canvas>


