use wasm_bindgen::prelude::wasm_bindgen;

use web_sys::{window, HtmlCanvasElement, KeyboardEvent, MouseEvent};

use super::gameplay::GamePlay;
use super::keymap::{KeyMap, KeyState};
use super::main_menu::MainMenu;
use super::score_screen::ScoreScreen;

use super::renderer::Renderer;
use super::transform::Transform2d;

// Pull in the console.log function so we can debug things more easily
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

enum GameState {
    Menu,
    Playing,
    ScoreScreen,
}

pub struct App {
    renderer: Renderer,

    key_map: KeyMap,

    main_menu: MainMenu,
    gameplay: GamePlay,
    score_screen: ScoreScreen,

    prev_time: f64,

    game_state: GameState,
}

impl App {
    pub fn new(canvas: HtmlCanvasElement, _options: String) -> Self {
        let renderer = Renderer::new(canvas).expect("Failed to create renderer");

        let now = window().unwrap().performance().unwrap().now();
        let prev_time = now / 1000.0;

        let mut game = Self {
            renderer,
            main_menu: MainMenu::new(),
            key_map: KeyMap::new(),
            gameplay: GamePlay::new(),
            score_screen: ScoreScreen::new(),
            prev_time,
            game_state: GameState::ScoreScreen,
        };
        game.reset();
        game
    }

    fn reset(&mut self) {
        self.gameplay.reset();

        // TODO: this is a bit dodgy
        self.renderer
            .map_sprite
            .set_to_map(&self.renderer.gl, &self.gameplay.map);
    }

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

    pub fn animation_frame(&mut self) {
        let now = window().unwrap().performance().unwrap().now();
        let time = now / 1000.0;
        let dt = time - self.prev_time;
        self.prev_time = time;

        match self.game_state {
            GameState::Menu => {
                self.show_logo(dt);
            }
            GameState::Playing => {
                self.play_game(dt);
            }
            GameState::ScoreScreen => {
                self.show_scores(dt);
            }
        }

        self.key_map.update();
    }

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

    pub fn show_logo(&mut self, dt: f64) {
        if self.key_map.start_game == KeyState::JustReleased {
            self.game_state = GameState::Playing;
            return;
        }
        self.main_menu.update(dt);

        let world_to_camera = Transform2d::new(0.0, -0.7, 0.0, 3.0);

        let ship_entity_refs = self.main_menu.logo.ships.iter().collect();
        let trail_entity_refs = self.main_menu.logo.trails.iter().collect();

        self.renderer.render(
            &world_to_camera,
            ship_entity_refs,
            trail_entity_refs,
            vec![&self.main_menu.text],
        );
    }

    pub fn mouse_event(&mut self, _event: MouseEvent) {
        //log(&format!("Mouse Event {:?}", event));
    }
    pub fn keydown_event(&mut self, event: KeyboardEvent) {
        if !event.repeat() {
            self.key_map
                .set_state_from_str(&event.code(), KeyState::JustPressed);
        }
    }

    pub fn keyup_event(&mut self, event: KeyboardEvent) {
        self.key_map
            .set_state_from_str(&event.code(), KeyState::JustReleased);
    }
}
