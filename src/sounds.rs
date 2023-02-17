use ggez::audio;
use ggez::{Context, GameResult};

pub struct Sounds {
    pub collect: audio::Source,
    pub fail: audio::Source,
    pub start: audio::Source,
    pub wrong: audio::Source,
    pub bonus: audio::Source,
}

impl Sounds {
    pub fn new(ctx: &mut Context) -> GameResult<Sounds> {
        let collect = audio::Source::new(ctx, "/sounds/collect.ogg")?;
        let fail = audio::Source::new(ctx, "/sounds/fail.ogg")?;
        let start = audio::Source::new(ctx, "/sounds/start.ogg")?;
        let wrong = audio::Source::new(ctx, "/sounds/wrong.ogg")?;
        let bonus = audio::Source::new(ctx, "/sounds/bonus.ogg")?;

        Ok(Sounds {
            collect,
            fail,
            start,
            wrong,
            bonus,
        })
    }
}
