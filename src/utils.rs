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
