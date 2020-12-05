use super::logo::Logo;
use super::text_sprite::TextBox;

pub struct MainMenu {
    pub logo: Logo,
    pub text: TextBox,
    logo_flash_time: f32,
}

impl MainMenu {
    pub fn new() -> Self {
        let text = TextBox::new((22, 1), 0.05, (0.0, -0.2));

        Self {
            logo: Logo::new(),
            text,
            logo_flash_time: 0.0,
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.logo_flash_time += dt as f32;

        let flash_color = [
            0.0,
            0.5 + 0.5 * f32::cos(std::f32::consts::PI * self.logo_flash_time + 0.0),
            0.0,
        ];
        if self.logo_flash_time > 1.0 {
            self.logo_flash_time = 0.0;
        }

        self.text.clear();
        self.text.append_string("Press", &[0.0, 0.7, 1.0]);
        self.text.append_string(" [ENTER] ", &flash_color);
        self.text.append_string("to start", &[0.0, 0.7, 1.0]);
    }
}
