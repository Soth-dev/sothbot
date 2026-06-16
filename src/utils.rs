#[macro_export]
macro_rules! text {
    ($bot:expr, $msg:expr, $text:expr) => {
        $bot.send_message($msg.chat.id, $text)
            .reply_parameters(teloxide::types::ReplyParameters::new($msg.id))
    };
    ($bot:expr, $msg:expr, $text:expr, $parser:expr) => {
        $bot.send_message($msg.chat.id, $text)
            .parse_mode($parser)
            .reply_parameters(teloxide::types::ReplyParameters::new($msg.id))
    };
}

#[macro_export]
macro_rules! text_to {
    ($bot:expr, $msg:expr, $reply:expr, $text:expr) => {
        $bot.send_message($msg.chat.id, $text)
            .reply_parameters(teloxide::types::ReplyParameters::new($reply.id))
    };
    ($bot:expr, $msg:expr, $reply:expr, $text:expr, $parser:expr) => {
        $bot.send_message($msg.chat.id, $text)
            .parse_mode($parser)
            .reply_parameters(teloxide::types::ReplyParameters::new($reply.id))
    };
}

#[macro_export]
macro_rules! edit {
    ($bot:expr, $chat:expr, $msg:expr, $text:expr) => {
        $bot.edit_message_text($chat.chat.id, $msg.id, $text)
    };
    ($bot:expr, $chat:expr, $msg:expr, $text:expr, $parser:expr) => {
        $bot.edit_message_text($chat.chat.id, $msg.id, $text)
            .parse_mode($parser)
    };
}

#[macro_export]
macro_rules! delete {
    ($bot:expr, $msg:expr) => {
        $bot.delete_message($msg.chat.id, $msg.id)
    };
    ($bot:expr, $chat:expr, $del:expr) => {
        $bot.delete_message($chat.chat.id, $del.id)
    };
}

#[macro_export]
macro_rules! image {
    ($bot:expr, $chat:expr, $file:expr) => {
        $bot.send_photo($chat.chat.id, $file)
            .reply_parameters(ReplyParameters::new($chat.id))
    };
}

#[macro_export]
macro_rules! f {
    ($t:expr, $u:expr) => {
        format!("<{}>{}</{}>", $u, $t, $u)
    };
}

#[macro_export]
macro_rules! q {
    ($t:expr) => {
        $crate::f!($t, "blockquote")
    };
}

#[macro_export]
macro_rules! m {
    ($t:expr) => {
        $crate::f!($t, "code")
    };
}

#[macro_export]
macro_rules! b {
    ($t:expr) => {
        $crate::f!($t, "b")
    };
}

#[macro_export]
macro_rules! i {
    ($t:expr) => {
        $crate::f!($t, "i")
    };
}

#[macro_export]
macro_rules! code {
    ($t:expr, $lang:expr) => {
        $crate::f!(
            format!("<code class=\"language-{}\">{}</code>", $lang, $t),
            "pre"
        )
    };
}

#[macro_export]
macro_rules! spoiler {
    ($t:expr) => {
        $crate::f!($t, "tg-spoiler")
    };
}

#[macro_export]
macro_rules! link {
    ($t:expr, $url:expr) => {
        format!("<a href=\"{}\">{}</a>", $url, $t)
    };
}

#[macro_export]
macro_rules! emoji {
    ($t:expr, $id:expr) => {
        format!("<tg-emoji emoji-id=\"{}\">{}</tg-emoji>", $id, $t)
    };
}

#[macro_export]
macro_rules! esp_html {
    ($t:expr) => {
        $t.replace("&", "&amp;")
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("\"", "&quot;")
    };
}

#[macro_export]
macro_rules! esp_md_v2 {
    ($t:expr) => {
        $t.replace("\\", "\\\\")
            .replace("_", "\\_")
            .replace("*", "\\*")
            .replace("[", "\\[")
            .replace("]", "\\]")
            .replace("(", "\\(")
            .replace(")", "\\)")
            .replace("`", "\\`")
            .replace("+", "\\+")
            .replace("-", "\\-")
            .replace("=", "\\=")
            .replace("#", "\\#")
            .replace("|", "\\|")
            .replace("{", "\\{")
            .replace("}", "\\}")
            .replace(">", "\\>")
    };
}
