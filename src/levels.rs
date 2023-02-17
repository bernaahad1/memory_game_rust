use ggez::graphics;
use ggez::graphics::Color;
use ggez::graphics::DrawParam;
use ggez::graphics::Quad;
use ggez::graphics::TextLayout;

use ggez::{Context, GameResult};

pub struct Level {
    text: graphics::Text,
    width: f32,
    height: f32,
    start_x: f32,
    start_y: f32,
    pub is_clicked: bool,
}

impl Level {
    pub fn new(
        _ctx: &mut Context,
        text: String,
        width: f32,
        height: f32,
        start_x: f32,
        start_y: f32,
    ) -> GameResult<Level> {
        let mut res_text = graphics::Text::new(text);
        res_text
            .set_scale(30.)
            .set_layout(TextLayout {
                h_align: graphics::TextAlign::Middle,
                v_align: graphics::TextAlign::Middle,
            })
            .set_bounds([start_x + (width / 2.), start_y + (height / 2.)])
            .set_wrap(true);

        Ok(Level {
            text: res_text,
            width,
            height,
            start_x,
            start_y,
            is_clicked: false,
        })
    }
    pub fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    pub fn draw(&self, canvas: &mut graphics::Canvas) -> GameResult {
        let mut rect_color = Color::BLUE;

        if self.is_clicked {
            rect_color = Color::YELLOW;
        }

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
        Ok(())
    }

    pub fn is_clicked(&mut self, x: f32, y: f32) -> bool {
        if y < self.start_y || y > self.start_y + self.height {
            return false;
        }
        if x < self.start_x || x > self.start_x + self.width {
            return false;
        }

        self.is_clicked = true;
        return true;
    }
}

pub struct Levels {
    pub easy: Level,
    pub medium: Level,
    pub hard: Level,
}

impl Levels {
    pub fn new(ctx: &mut Context, screen_width: f32, screen_height: f32) -> GameResult<Levels> {
        let start_x = (screen_width - (600. + 100.)) / 2.;
        let start_y = (screen_height - 70.) / 2.;

        let easy = Level::new(ctx, "Easy".to_owned(), 200.0, 70., start_x, start_y)?;
        let medium = Level::new(
            ctx,
            "Medium".to_owned(),
            200.0,
            70.,
            start_x + 250.,
            start_y,
        )?;
        let hard = Level::new(ctx, "Hard".to_owned(), 200.0, 70., start_x + 500., start_y)?;

        Ok(Levels { easy, medium, hard })
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.easy.update(ctx)?;
        self.medium.update(ctx)?;
        self.hard.update(ctx)?;

        Ok(())
    }

    pub fn draw(&self, canvas: &mut graphics::Canvas) -> GameResult {
        self.easy.draw(canvas)?;
        self.medium.draw(canvas)?;
        self.hard.draw(canvas)?;

        Ok(())
    }
}
