use std::collections::HashMap;
use std::env;
use std::path;
use std::path::Path;

use ggez::audio;
use ggez::audio::SoundSource;
use ggez::event;
use ggez::event::MouseButton;
use ggez::glam::Vec2;
use ggez::graphics;
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

pub mod card;
use crate::card::Card;

#[derive(Debug)]
enum GameMatchState {
    Match,
    Default,
    NotMatched,
}

struct MainState {
    mouse_down: bool,
    mouse_click: Option<Vec2>,
    board_size: u32,
    cards_map: HashMap<(u32, u32), Card>,
    timer: GameTimer,
    selected: Vec<((u32, u32), u32)>,
    time_on_last_click: Option<Duration>,
    game_state: GameMatchState,
}

impl MainState {
    fn new(ctx: &mut Context, board_size: u32) -> GameResult<MainState> {
        let mut card_ids: Vec<u32> = (1..=((board_size * 3) / 2 as u32))
            .chain(1..=((board_size * 3) / 2 as u32))
            .collect();

        let mut rng = rand::thread_rng();
        card_ids.shuffle(&mut rng);

        let timer = GameTimer::new(ctx, Instant::now(), Duration::from_secs(120))?;
        let mut cards_map = HashMap::new();

        let start =
            (WINDOW_WIDTH.floor() as u32 - (10 * (board_size - 1)) - (CARD_WIDTH * board_size)) / 2;

        for j in 1..4 {
            let y = (50 * j) + CARD_HEIGHT * (j - 1);
            for i in 0..board_size {
                let x = start + (CARD_WIDTH + 10) * i;

                if let Some(match_id) = card_ids.pop() {
                    cards_map.insert(
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

        let s = MainState {
            mouse_down: false,
            mouse_click: None,
            board_size,
            cards_map,
            timer,
            selected: Vec::new(),
            time_on_last_click: None,
            game_state: GameMatchState::Default,
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.timer.update(ctx)?;
        if self.cards_map.len() == 0 {
            ctx.request_quit();
            return Ok(());
        }
        self.game_state = GameMatchState::Default;

        for (_, value) in self.cards_map.iter_mut() {
            value.update(ctx)?;
        }

        if self.mouse_down && self.selected.len() < 2 {
            if let Some(click) = self.mouse_click {
                for (key, value) in self.cards_map.iter_mut() {
                    if key.0 as f32 <= click[0] && (key.0 + CARD_WIDTH) as f32 >= click[0] {
                        if key.1 as f32 <= click[1] && (key.1 + CARD_HEIGHT) as f32 >= click[1] {
                            if value.is_clicked {
                                self.mouse_down = false;
                                self.mouse_click = None;
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

        self.mouse_down = false;
        self.mouse_click = None;

        if let Some(time) = self.time_on_last_click {
            if time + Duration::from_secs(1) < ctx.time.time_since_start() {
                if self.selected.len() == 2 && self.selected[0].1 == self.selected[1].1 {
                    // let card_1 =
                    self.cards_map.remove(&self.selected[0].0);
                    // if let Some(first) = card_1 {
                    //     first.is_matched = true;
                    // }

                    // let card_2 =
                    self.cards_map.remove(&self.selected[1].0);

                    // if let Some(second) = card_2 {
                    //     second.is_matched = true;
                    // }

                    self.game_state = GameMatchState::Match;
                    self.timer.give_additional_time(Duration::from_secs(5));

                    self.selected = Vec::new();
                } else if self.selected.len() >= 2 && self.selected[0].1 != self.selected[1].1 {
                    let card_1 = self.cards_map.get_mut(&self.selected[0].0);
                    if let Some(card) = card_1 {
                        card.click();
                        // ctx.time.time_since_start()
                    }

                    let card_2 = self.cards_map.get_mut(&self.selected[1].0);
                    if let Some(card) = card_2 {
                        println!("{:?}", card);
                        card.click();
                    }

                    self.game_state = GameMatchState::NotMatched;
                    self.timer.take_time(Duration::from_secs(2));

                    self.selected = Vec::new();
                }
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

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let dark_blue = graphics::Color::from_rgb(26, 51, 77);
        let mut canvas = graphics::Canvas::from_frame(ctx, dark_blue);
        self.timer.draw(&mut canvas)?;

        for (_key, value) in self.cards_map.iter_mut() {
            value.draw(&mut canvas)?;
        }

        match self.game_state {
            GameMatchState::Match => {
                let mut matched = audio::Source::new(ctx, "/sounds/collect.ogg")?;
                matched.play(ctx)?;
                println!("Match")
            }
            GameMatchState::NotMatched => println!("Not a match"),
            _ => {}
        }
        

        canvas.finish(ctx)?;

        Ok(())
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
    let (mut ctx, event_loop) = ContextBuilder::new("memry_game", "awesome_person")
        .default_conf(conf.clone())
        .build()
        .unwrap();

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.fs.mount(&path, true);
    }

    let state = MainState::new(&mut ctx, 4).unwrap();

    event::run(ctx, event_loop, state);
}
