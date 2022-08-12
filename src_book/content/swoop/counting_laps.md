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
```rust
pub fn generate_leaderboard_text(&mut self) {
        self.leaderboard_text.clear();

        let mut ship_and_score_refs: Vec<(&Ship, &Score)> = self.ship_entities.iter().zip(self.scores.iter()).collect();
        ship_and_score_refs.sort_by(|a, b| { a.1.cmp(b.1)});
```

Hmm, what's this `cmp` function? Technically it should be an
implementation of the `Ord` trait, but to implement `Ord` you also
need to implementation `Eq` and `PartialOrd` and `PartialEq`. So
instead of `impl Ord for Score` I'm just putting the cmp function
in the `imp Score` block.
When sorting scores, first we need to sort by the lap counter:
```rust
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
            ....????
        }
```
If the laps are the same, we need to sort by the least time:
```rust
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
```
Some of these conditions should never be hit - if 
`a_last_lap.is_some()`, then `b_last_lap` will also be 
`is_some()` because we check that the number of laps is
the same. I can't think of a way to express this to the
compiler, so it'll just be a bit more verbose than it
needs to be.

Now that we can sort a list of scores we can find the winner and 
format the scoreboard as described above. The resulting function
is:
```rust
    pub fn generate_leaderboard_text(&mut self) {
        self.leaderboard_text.clear();

        let mut ship_and_score_refs: Vec<(&Ship, &Score)> =
            self.ship_entities.iter().zip(self.scores.iter()).collect();
        ship_and_score_refs.sort_by(|a, b| a.1.cmp(b.1));
        let winner_score = ship_and_score_refs.first().expect("No Ships").1;

        self.leaderboard_text.append_string(
            &format!("Lap {}/{}  ", winner_score.laps.len(), NUM_LAPS_TO_WIN),
            &[0.5, 0.5, 0.5],
        );
        for (ship, score) in ship_and_score_refs {
            let color = [ship.color.0, ship.color.1, ship.color.2];
            if score.laps.len() == winner_score.laps.len() {
                if let Some(winner_time) = winner_score.laps.last() {
                    // Same lap - display time
                    let time = score.laps.last().unwrap() - winner_time;
                    let seconds = time as u32;
                    let millis = (time.fract() * 100.0).floor() as u32;
                    self.leaderboard_text
                        .append_string(&format!("~ {:02}:{:02}  ", seconds, millis), &color);
                } else {
                    // No-one has any time yet
                    self.leaderboard_text
                        .append_string(&format!("~ --:--  ",), &color);
                }
            } else {
                // This player is at least a lap behind
                self.leaderboard_text
                    .append_string(&format!("~ --:--  ",), &color);
            }
        }
    }
```
The `unwrap` should never be encountered for the same reason as
mentioned above. We only get there when 
`winner_score.laps.last().is_some()` and when the length of the two laps arrays are equal. If someone knows how to tell the compiler this,
I'd love to know!

Anyway, the only thing to do now is to play it and see how far ahead
purple actually gets.....

<canvas id="swoop/swoop_counting_laps"></canvas>


