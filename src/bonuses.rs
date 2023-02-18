use std::time::Duration;

use ggez::graphics;
use ggez::graphics::{Color, DrawParam, Quad, TextLayout};
use ggez::{Context, GameResult};

#[derive(Debug)]
pub enum BonusState {
    Used,
    NotUsed,
    Using,
    NotActive,
}

#[derive(Debug)]
pub struct Bonus {
    text: graphics::Text,
    pub state: BonusState,
    width: f32,
    height: f32,
    start_x: f32,
    start_y: f32,
    pub started: Option<Duration>,
    pub duration: Duration,
}

impl Bonus {
    pub fn new(
        _ctx: &mut Context,
        text: String,
        state: BonusState,
        width: f32,
        height: f32,
        start_x: f32,
        start_y: f32,
        duration: Duration,
    ) -> GameResult<Bonus> {
        let mut bonus_text = graphics::Text::new(text);
        bonus_text
            .set_scale(30.)
            .set_layout(TextLayout {
                h_align: graphics::TextAlign::Middle,
                v_align: graphics::TextAlign::Middle,
            })
            .set_bounds([start_x + (width / 2.), start_y + (height / 2.)])
            .set_wrap(true);

        Ok(Bonus {
            text:bonus_text,
            state,
            width,
            height,
            start_x,
            start_y,
            duration,
            started: None,
        })
    }
    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Set the time that started using 
        if matches!(self.state, BonusState::Using) && self.started.is_none() {
            self.started = Some(ctx.time.time_since_start())

        } else if matches!(self.state, BonusState::Using) && !self.started.is_none() {
            if let Some(start) = self.started {
                // if the given duration has passed update the state
                if start + self.duration < ctx.time.time_since_start() {
                    self.update_state()?;
                }
            }
        }

        Ok(())
    }

    pub fn draw(&self, canvas: &mut graphics::Canvas) -> GameResult {
        let rect_color = match self.state {
            BonusState::Used => return Ok(()),
            BonusState::NotUsed => Color::GREEN,
            BonusState::Using => Color::YELLOW,
            BonusState::NotActive => return Ok(()),
        };

        canvas.draw(
            &self.text,
            DrawParam::default()
                .color(Color::WHITE)
                .dest([
                    self.start_x + (self.width / 2.),
                    self.start_y + (self.height / 2.),
                ])
                .z(5),
        );
        canvas.draw(
            &Quad,
            DrawParam::default()
                .color(rect_color)
                .scale([self.width, self.height])
                .dest([
                    self.start_x + (self.width / 2.),
                    self.start_y + (self.height / 2.),
                ])
                .offset([0.5, 0.5]),
        );

        Ok(())
    }

    pub fn update_state(&mut self) -> GameResult {
        self.state = match self.state {
            BonusState::Used => BonusState::Used,
            BonusState::NotUsed => BonusState::Using,
            BonusState::Using => BonusState::Used,
            BonusState::NotActive => BonusState::NotUsed,
        };
        Ok(())
    }

    pub fn click_and_update(&mut self, x: f32, y: f32) -> bool {
        if y < self.start_y || y > self.start_y + self.height {
            return false;
        }
        if x < self.start_x || x > self.start_x + self.width {
            return false;
        }

        return match self.state {
            BonusState::Used => false,
            BonusState::NotUsed => {
                self.update_state();
                return true;
            }
            BonusState::Using => false,
            BonusState::NotActive => false,
        };
    }
}

#[derive(Debug)]
pub struct Bonuses {
    pub bonus_time: Bonus,
    pub freeze_time: Bonus,
    pub free_match: Bonus,
}

impl Bonuses {
    pub fn new(ctx: &mut Context, screen_width: f32) -> GameResult<Bonuses> {
        let start_x = (screen_width - (600. + 100.)) / 2.;
        let bonus_time = Bonus::new(
            ctx,
            "+15 sec".to_owned(),
            BonusState::NotActive,
            200.0,
            70.,
            start_x,
            800.,
            Duration::new(2, 0),
        )?;
        let freeze_time = Bonus::new(
            ctx,
            "Freeze time".to_owned(),
            BonusState::NotUsed,
            200.0,
            70.,
            start_x + 250.,
            800.,
            Duration::new(15, 0),
        )?;
        let free_match = Bonus::new(
            ctx,
            "Match hint".to_owned(),
            BonusState::NotUsed,
            200.0,
            70.,
            start_x + 500.,
            800.,
            Duration::new(1, 0),
        )?;

        Ok(Bonuses {
            bonus_time,
            freeze_time,
            free_match,
        })
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.bonus_time.update(ctx)?;
        self.free_match.update(ctx)?;
        self.freeze_time.update(ctx)?;

        Ok(())
    }

    pub fn draw(&self, canvas: &mut graphics::Canvas) -> GameResult {
        self.bonus_time.draw(canvas)?;
        self.free_match.draw(canvas)?;
        self.freeze_time.draw(canvas)?;

        Ok(())
    }
}
