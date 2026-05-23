use crate::text;
use std::{collections::HashMap, sync::LazyLock};
use teloxide::{prelude::*, types::Message};

static FLIP_MAP: LazyLock<HashMap<char, char>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    let ranges = [
        ("abcdefghijklmnopqrstuvwxyz", "ɐqɔpǝɟƃɥᴉɾʞꞁɯuodbɹsʇnʌʍxʎz"),
        ("ABCDEFGHIJKLMNOPQRSTUVWXYZ", "ⱯᗺƆᗡƎᖵ⅁HIᒋ⋊ꞀWNOԀꝹᴚS⊥∩ɅMX⅄Z"),
        ("0123456789", "0ІᘔƐᔭ59Ɫ86"),
        (
            "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~",
            "¡„#$%⅋,)(*+'-˙/:؛>=<¿@]\\[ᵥ‾`}|{~",
        ),
    ];

    for (normal, flipped) in ranges {
        for (n, f) in normal.chars().zip(flipped.chars()) {
            m.insert(n, f);
            m.insert(f, n);
        }
    }
    m
});

static DIACRITICS_MAP: LazyLock<HashMap<char, char>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    let pairs = [
        ('\u{0308}', '\u{0324}'),
        ('\u{030A}', '\u{0325}'),
        ('\u{0301}', '\u{0317}'),
        ('\u{0300}', '\u{0316}'),
        ('\u{0307}', '\u{0323}'),
        ('\u{0303}', '\u{0330}'),
        ('\u{0304}', '\u{0331}'),
        ('\u{0302}', '\u{032C}'),
        ('\u{0306}', '\u{032F}'),
        ('\u{030C}', '\u{032D}'),
        ('\u{0311}', '\u{032E}'),
        ('\u{030D}', '\u{0329}'),
    ];
    for (n, f) in pairs {
        m.insert(n, f);
        m.insert(f, n);
    }
    m
});

fn transform_text(text: &str) -> String {
    let mut output = String::new();

    for c in text.replace('ß', "ss").chars().rev() {
        if let Some(&flipped) = FLIP_MAP.get(&c) {
            output.push(flipped);
        } else if let Some(&flipped_diacritic) = DIACRITICS_MAP.get(&c) {
            output.push(flipped_diacritic);
        } else {
            output.push(c);
        }
    }
    output
}

pub async fn flip(bot: Bot, msg: Message, text: String) -> ResponseResult<()> {
    let reply_text = match msg.reply_to_message() {
        Some(m) => m.text(),
        None => None,
    };
    if text.is_empty() && reply_text.is_none() {
        text!(bot, msg, "Use /flip [text]").await?;
        return Ok(());
    }
    let flipped = transform_text(if text.is_empty() {
        reply_text.unwrap_or(&text)
    } else {
        &text
    });
    text!(bot, msg, flipped).await?;

    Ok(())
}
