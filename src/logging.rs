use anyhow::Result;
use colored::Colorize;
use fern::colors::{Color, ColoredLevelConfig};
use std::{sync, thread};

pub fn init() -> Result<()> {
    let (tx, rx) = sync::mpsc::channel();

    thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            print!("{}", msg);
        }
    });

    let colors = ColoredLevelConfig::new()
        .debug(Color::Magenta)
        .info(Color::Blue);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} {} {} {}",
                chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                colors.color(record.level()),
                record.target().dimmed(),
                message,
            ))
        })
        .level(log::LevelFilter::Trace)
        .level_for("", log::LevelFilter::Error)
        .level_for("hyper", log::LevelFilter::Error)
        .level_for("mio", log::LevelFilter::Error)
        .level_for("reqwest", log::LevelFilter::Error)
        .level_for("sqlx", log::LevelFilter::Error)
        .level_for("tracing", log::LevelFilter::Error)
        .level_for("want", log::LevelFilter::Error)
        .level_for("warp", log::LevelFilter::Error)
        .chain(tx)
        .apply()
        .map_err(Into::into)
}
