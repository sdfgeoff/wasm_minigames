use super::text_sprite::TextBox;
use super::ship::Ship;
use super::score::Score;

pub struct ScoreScreen {
    title: TextBox,
    scores: TextBox,
    instructions: TextBox,

}

impl ScoreScreen {
    pub fn new() -> Self {
        let mut title = TextBox::new((15, 1), 0.1, (0.0, 0.5));
        let mut scores = TextBox::new((13, 5), 0.05, (0.0, 0.0));
        let mut instructions = TextBox::new((27, 1), 0.05, (0.0, -0.5));

        title.clear();
        title.append_string("Round Completed", &[0.0, 0.7, 1.0]);

        scores.clear();

        instructions.clear();
        instructions.append_string("Press ", &[0.0, 0.7, 1.0]);
        instructions.append_string("[ENTER]", &[0.0, 1.0, 0.7]);
        instructions.append_string(" to play again", &[0.0, 0.7, 1.0]);
        Self {
            title,
            scores,
            instructions,
        }
    }

    pub fn get_text_entities<'a>(&'a self) -> Vec<&'a TextBox> {
        vec![&self.title, &self.scores, &self.instructions]
    }

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
