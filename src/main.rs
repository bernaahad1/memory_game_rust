use std::collections::HashMap;
use std::env;
use std::path;
use std::path::Path;

use bonuses::BonusState;
use ggez::audio::SoundSource;
use ggez::event;
use ggez::event::MouseButton;
use ggez::glam::Vec2;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::timer;
use std::time::Duration;
use std::time::Instant;

use ggez::conf::{Conf, WindowMode};
use ggez::mint::Point2;
use ggez::GameError;
use ggez::{Context, ContextBuilder, GameResult};
use rand::seq::SliceRandom;

const WINDOW_WIDTH: f32 = 1600.0;
const WINDOW_HEIGHT: f32 = 900.0;
const CARD_WIDTH: u32 = 125;
const CARD_HEIGHT: u32 = 200;

pub mod gameTimer;
use crate::gameTimer::GameTimer;

pub mod sounds;
use crate::sounds::Sounds;

pub mod card;
use crate::card::Card;

pub mod bonuses;
use crate::bonuses::Bonuses;

pub mod levels;
use crate::levels::Levels;

#[derive(Debug)]
enum GameState {
    Home,
    Match,
    NotMatched,
    Win,
    Lost,
    Default,
}

struct MainState {
    mouse_down: bool,
    mouse_click: Option<Vec2>,
    cards_map: HashMap<(u32, u32), Card>,
    timer: GameTimer,
    selected: Vec<((u32, u32), u32)>,
    time_on_last_click: Option<Duration>,
    game_state: GameState,
    match_strike: usize,
    bonuses: Bonuses,
    levels: Levels,
    sounds: Sounds,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let timer = GameTimer::new(ctx, Instant::now(), Duration::from_secs(0))?;

        let cards_map = HashMap::new();

        let bonuses = Bonuses::new(ctx, WINDOW_WIDTH)?;
        let levels = Levels::new(ctx, WINDOW_WIDTH, WINDOW_HEIGHT)?;
        let sounds = Sounds::new(ctx)?;

        Ok(MainState {
            mouse_down: false,
            mouse_click: None,
            cards_map,
            timer,
            selected: Vec::new(),
            time_on_last_click: None,
            game_state: GameState::Home,
            match_strike: 0,
            bonuses,
            levels,
            sounds,
        })
    }

    fn create_game(&mut self, ctx: &mut Context, board_size: u32, seconds: Duration) -> GameResult {
        let mut card_ids: Vec<u32> = (1..=(((board_size * 3) / 2) as u32))
            .chain(1..=((board_size * 3) / 2 as u32))
            .collect();

        let mut rnd = rand::thread_rng();
        card_ids.shuffle(&mut rnd);

        self.timer = GameTimer::new(ctx, Instant::now(), seconds)?;

        let start =
            (WINDOW_WIDTH.floor() as u32 - (10 * (board_size - 1)) - (CARD_WIDTH * board_size)) / 2;

        for j in 1..4 {
            let y = (50 * j) + CARD_HEIGHT * (j - 1);
            for i in 0..board_size {
                let x = start + (CARD_WIDTH + 10) * i;

                if let Some(match_id) = card_ids.pop() {
                    self.cards_map.insert(
                        (x, y),
                        Card::new(
                            ctx,
                            Path::new("/cards/back.png"),
                            &Path::new(&format!("/cards/card_{:?}.png", match_id)),
                            Point2 {
                                x: x as f32,
                                y: y as f32,
                            },
                            match_id,
                        )
                        .unwrap(),
                    );
                } else {
                    break;
                }
            }
        }

        Ok(())
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Slecting the game level
        if matches!(self.game_state, GameState::Home) {
            if self.levels.easy.is_clicked {
                self.create_game(ctx, 2, Duration::from_secs(45))?;
                self.game_state = GameState::Default;
            } else if self.levels.medium.is_clicked {
                self.create_game(ctx, 4, Duration::from_secs(60))?;
                self.game_state = GameState::Default;
            } else if self.levels.hard.is_clicked {
                self.create_game(ctx, 6, Duration::from_secs(90))?;
                self.game_state = GameState::Default;
            } else if let Some(click) = self.mouse_click {
                if self.levels.easy.is_clicked(click.x, click.y) {
                    self.sounds.start.play(ctx)?;
                    self.levels.update(ctx)?;
                } else if self.levels.medium.is_clicked(click.x, click.y) {
                    self.sounds.start.play(ctx)?;
                    self.levels.update(ctx)?;
                } else if self.levels.hard.is_clicked(click.x, click.y) {
                    self.sounds.start.play(ctx)?;
                    self.levels.update(ctx)?;
                }
            }

            self.levels.update(ctx)?;

            return Ok(());
        }

        self.game_state = GameState::Default;

        // Check if any bonus is selected
        if self.mouse_down {
            if let Some(click) = self.mouse_click {
                if self.bonuses.bonus_time.click_and_update(click.x, click.y) {
                    self.sounds.bonus.play(ctx)?;

                    self.timer.duration += Duration::new(15, 0);
                    self.mouse_down = false;
                    self.mouse_click = None;
                } else if self.bonuses.freeze_time.click_and_update(click.x, click.y) {
                    self.sounds.bonus.play(ctx)?;

                    self.timer.duration += Duration::new(15, 0);
                    self.mouse_down = false;
                    self.mouse_click = None;
                } else if self.selected.len() <= 1
                    && self.bonuses.free_match.click_and_update(click.x, click.y)
                {
                    self.sounds.bonus.play(ctx)?;

                    if self.selected.len() == 0 {
                        let mut match_id: i32 = -1;
                        for (key, value) in self.cards_map.iter_mut() {
                            if match_id == -1 {
                                match_id = value.match_id as i32;
                                value.click();
                                self.selected.push((*key, value.match_id));
                                continue;
                            }
                            if value.match_id as i32 == match_id {
                                self.selected.push((*key, value.match_id));
                                self.time_on_last_click = Some(ctx.time.time_since_start());

                                value.click();
                                break;
                            }
                        }
                    }
                    if self.selected.len() == 1 {
                        let match_id: u32 = self.selected[0].1;
                        for (key, value) in self.cards_map.iter_mut() {
                            if value.match_id == match_id && value.is_clicked == false {
                                self.selected.push((*key, value.match_id));
                                self.time_on_last_click = Some(ctx.time.time_since_start());

                                value.click();
                                self.mouse_down = false;
                                self.mouse_click = None;
                            }
                        }
                    }
                    self.mouse_down = false;
                    self.mouse_click = None;
                }
            }
        }

        self.bonuses.update(ctx)?;

        // Depending on the freeze time bonus update the timer
        match self.bonuses.freeze_time.state {
            BonusState::Used => {
                self.timer.update(ctx)?;
                self.sounds.bonus.play(ctx)?;
            }
            BonusState::NotUsed => self.timer.update(ctx)?,
            BonusState::Using => {}
            BonusState::NotActive => self.timer.update(ctx)?,
        }

        // Game is over if the given time passed
        if self.timer.remaining <= Duration::new(0, 5) && self.cards_map.len() != 0 {
            self.game_state = GameState::Lost;
            ctx.request_quit();

            return Ok(());
        }

        // Game if over of there are no cards left
        if self.cards_map.len() == 0 {
            ctx.request_quit();
            self.game_state = GameState::Win;
            return Ok(());
        }

        // Check if any card is clicked if there are 0 or 1 flipped already
        if self.mouse_down && self.selected.len() < 2 {
            if let Some(click) = self.mouse_click {
                for (key, value) in self.cards_map.iter_mut() {
                    if key.0 as f32 <= click[0] && (key.0 + CARD_WIDTH) as f32 >= click[0] {
                        if key.1 as f32 <= click[1] && (key.1 + CARD_HEIGHT) as f32 >= click[1] {
                            if value.is_clicked {
                                return Ok(());
                            }
                            value.click();
                            self.selected.push((*key, value.match_id));
                            self.mouse_down = false;
                            self.mouse_click = None;
                            self.time_on_last_click = Some(ctx.time.time_since_start());

                            return Ok(());
                        }
                    }
                }
            }
        }

        for (_, value) in self.cards_map.iter_mut() {
            value.update(ctx)?;
        }

        // Give some time for the card to flip before checking for match
        if let Some(time) = self.time_on_last_click {
            if time + Duration::from_secs(1) < ctx.time.time_since_start() {
                // Check for match
                if self.selected.len() == 2 && self.selected[0].1 == self.selected[1].1 {
                    self.cards_map.remove(&self.selected[0].0);

                    self.cards_map.remove(&self.selected[1].0);

                    self.game_state = GameState::Match;

                    if !matches!(self.bonuses.freeze_time.state, BonusState::Using) {
                        self.timer.give_additional_time(Duration::from_secs(5));
                    }

                    self.match_strike += 1;

                    self.selected = Vec::new();
                } else if self.selected.len() >= 2 && self.selected[0].1 != self.selected[1].1 {
                    let card_1 = self.cards_map.get_mut(&self.selected[0].0);
                    if let Some(card) = card_1 {
                        card.click();
                    }

                    let card_2 = self.cards_map.get_mut(&self.selected[1].0);
                    if let Some(card) = card_2 {
                        card.click();
                    }

                    self.game_state = GameState::NotMatched;

                    if !matches!(self.bonuses.freeze_time.state, BonusState::Using) {
                        self.timer.take_time(Duration::from_secs(2));
                    }

                    self.match_strike = 0;

                    self.selected = Vec::new();
                }
            }
        }

        // Check for match strike
        if self.match_strike == 2 {
            println!("strike");
            if matches!(self.bonuses.bonus_time.state, BonusState::NotActive) {
                self.bonuses.bonus_time.update_state()?;
            }
        }

        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32,
    ) -> Result<(), GameError> {
        self.mouse_down = true;
        self.mouse_click = Some(Vec2::new(x, y));

        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) -> Result<(), GameError> {
        self.mouse_down = false;
        self.mouse_click = None;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let canvas_color = graphics::Color::from_rgb(0, 25, 51);
        let mut canvas = graphics::Canvas::from_frame(ctx, canvas_color);

        // Check and draw the game state if needed
        match self.game_state {
            GameState::Home => {
                self.levels.draw(&mut canvas)?;
                canvas.finish(ctx)?;

                return Ok(());
            }
            GameState::Match => {
                self.sounds.collect.play(ctx)?;

                println!("Match")
            }
            GameState::NotMatched => {
                self.sounds.wrong.play(ctx)?;

                println!("Not a match");
            }
            GameState::Win => {
                let dest = Point2 {
                    x: ((WINDOW_WIDTH / 2.0) as f32),
                    y: ((WINDOW_HEIGHT / 2.0) as f32),
                };

                self.sounds.start.play_later()?;

                let mut text_finish = graphics::Text::new("FINISH");

                let mut text_win = graphics::Text::new("You win!");

                text_finish.set_scale(70.0);

                text_win.set_scale(70.0);

                let draw_params_finish = graphics::DrawParam::default()
                    .dest(Point2 {
                        x: dest.x,
                        y: dest.y - 40.0,
                    })
                    .offset(Point2 { x: 0.5, y: 0.5 })
                    .color(Color::WHITE);

                let draw_params_win = graphics::DrawParam::default()
                    .dest(Point2 {
                        x: dest.x,
                        y: dest.y + 40.0,
                    })
                    .offset(Point2 { x: 0.5, y: 0.5 })
                    .color(Color::WHITE);

                canvas.draw(&text_finish, draw_params_finish);

                canvas.draw(&text_win, draw_params_win);

                canvas.finish(ctx)?;

                return Ok(());
            }
            GameState::Lost => {
                let dest = Point2 {
                    x: ((WINDOW_WIDTH / 2.0) as f32),
                    y: ((WINDOW_HEIGHT / 2.0) as f32),
                };

                self.sounds.fail.play_later()?;

                let mut text_time_out = graphics::Text::new("TIME OUT");

                let mut text_you_lost = graphics::Text::new("You lost the game!");

                text_time_out.set_scale(70.0);

                text_you_lost.set_scale(70.0);

                let draw_params_time_out = graphics::DrawParam::default()
                    .dest(Point2 {
                        x: dest.x,
                        y: dest.y - 40.0,
                    })
                    .offset(Point2 { x: 0.5, y: 0.5 })
                    .color(Color::WHITE);

                let draw_params_lost = graphics::DrawParam::default()
                    .dest(Point2 {
                        x: dest.x,
                        y: dest.y + 40.0,
                    })
                    .offset(Point2 { x: 0.5, y: 0.5 })
                    .color(Color::WHITE);

                canvas.draw(&text_time_out, draw_params_time_out);

                canvas.draw(&text_you_lost, draw_params_lost);

                canvas.finish(ctx)?;

                return Ok(());
            }
            _ => {}
        }

        //Draw timer
        self.timer.draw(&mut canvas)?;

        // Draw bonus buttons
        self.bonuses.draw(&mut canvas)?;

        // Draw cards
        for (_key, value) in self.cards_map.iter_mut() {
            value.draw(&mut canvas)?;
        }

        canvas.finish(ctx)?;

        Ok(())
    }

    fn quit_event(&mut self, _ctx: &mut Context) -> Result<bool, GameError> {
        // Check if game quit is because Time Out or Win or sth else
        if self.timer.remaining <= Duration::new(0, 3) || self.cards_map.len() == 0 {
            timer::sleep(Duration::from_secs(5));
        }
        return Ok(false);
    }
}
pub fn main() -> GameResult {
    // Конфигурация:
    let conf = Conf::new().window_mode(WindowMode {
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        ..Default::default()
    });

    // Контекст и event loop
    let (mut ctx, event_loop) = ContextBuilder::new("memory_game", "Berna Ahad")
        .default_conf(conf.clone())
        .build()
        .unwrap();

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");

        ctx.fs.mount(&path, true);
    }

    let state = MainState::new(&mut ctx).unwrap();

    event::run(ctx, event_loop, state);
}
