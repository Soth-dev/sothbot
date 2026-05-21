use std::io::Cursor;

use crate::{delete, image, m, q, text};
use ab_glyph::{FontRef, PxScale};
use image::Rgb;
use imageproc::drawing::draw_text_mut;
use teloxide::{
    prelude::*,
    types::{InputFile, ParseMode, ReplyParameters},
};

pub async fn run(bot: Bot, msg: Message, text: String) -> ResponseResult<()> {
    if text.is_empty() {
        text!(bot, msg, "Use /write [text]").await?;
        return Ok(());
    }
    let msg1 = text!(bot, msg, q!(m!("Writing...")), ParseMode::Html).await?;

    let img_bytes = tokio::task::spawn_blocking(move || {
        let mut img = image::open("assets/note.jpg").unwrap().to_rgb8();

        let font_data = std::fs::read("assets/fonts/ass.ttf").unwrap();
        let font = FontRef::try_from_slice(&font_data).unwrap();

        let scale = PxScale::from(40.0);
        let mut y = 140;
        let x = 150;
        let lines = text_set(&text);

        let line_height = 42;

        for line in lines {
            for l in line.split("\n") {
                draw_text_mut(&mut img, Rgb([1, 22, 55]), x, y, scale, &font, l);
                y += line_height;
            }
        }

        let mut buffer = Cursor::new(Vec::new());
        img.write_to(&mut buffer, image::ImageFormat::Jpeg).unwrap();
        buffer.into_inner()
    })
    .await
    .unwrap();

    let input_file = InputFile::memory(img_bytes).file_name("note.jpg");
    image!(bot, msg, input_file).await?;

    delete!(bot, msg1).await?;
    Ok(())
}

fn text_set(text: &str) -> Vec<String> {
    let mut lines = Vec::new();
    if text.len() <= 55 {
        lines.push(text.to_string());
    } else {
        for line in text.lines() {
            if line.len() <= 55 {
                lines.push(line.to_string());
            } else {
                let chars: Vec<char> = line.chars().collect();
                for chunk in chars.chunks(55) {
                    lines.push(chunk.iter().collect());
                }
            }
        }
    }
    lines.truncate(24);
    lines
}
