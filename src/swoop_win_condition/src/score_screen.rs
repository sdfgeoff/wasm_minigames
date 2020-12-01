use super::text_sprite::TextBox;

pub struct ScoreScreen {
    title: TextBox,
    scores: TextBox,
    instructions: TextBox,

}

impl ScoreScreen {
    pub fn new() -> Self {
        let mut title = TextBox::new((22, 1), 0.05, (0.0, -0.2));
        let scores = TextBox::new((22, 1), 0.05, (0.0, -0.2));
        let mut instructions = TextBox::new((22, 1), 0.05, (0.0, -0.2));

        title.clear();
        title.append_string("Round Completed", &[0.0, 0.7, 1.0]);


        instructions.clear();
        instructions.append_string("Press ", &[0.0, 0.7, 1.0]);
        instructions.append_string("[ENTER] ", &[0.0, 1.0, 0.7]);
        instructions.append_string(" to play again", &[0.0, 7.0, 1.0]);
        Self {
            title,
            scores,
            instructions,
        }
    }

    pub fn get_text_entities<'a>(&'a self) -> Vec<&'a TextBox> {
        vec![&self.title, &self.scores, &self.instructions]
    }

}