#[macro_export]
macro_rules! text {
    ($bot:expr, $msg:expr, $text:expr) => {
        $bot.send_message($msg.chat.id, $text)
            .reply_parameters(teloxide::types::ReplyParameters::new($msg.id))
            .await
    };
    ($bot:expr, $msg:expr, $text:expr, $parser:expr) => {
        $bot.send_message($msg.chat.id, $text)
            .parse_mode($parser)
            .reply_parameters(teloxide::types::ReplyParameters::new($msg.id))
            .await
    };
}

#[macro_export]
macro_rules! text_to {
    ($bot:expr, $msg:expr, $reply:expr, $text:expr) => {
        $bot.send_message($msg.chat.id, $text)
            .reply_parameters(teloxide::types::ReplyParameters::new($reply.id))
            .await
    };
    ($bot:expr, $msg:expr, $reply:expr, $text:expr, $parser:expr) => {
        $bot.send_message($msg.chat.id, $text)
            .parse_mode($parser)
            .reply_parameters(teloxide::types::ReplyParameters::new($reply.id))
            .await
    };
}

#[macro_export]
macro_rules! edit {
    ($bot:expr, $chat:expr, $msg:expr, $text:expr) => {
        $bot.edit_message_text($chat.chat.id, $msg.id, $text).await
    };
    ($bot:expr, $chat:expr, $msg:expr, $text:expr, $parser:expr) => {
        $bot.edit_message_text($chat.chat.id, $msg.id, $text)
            .parse_mode($parser)
            .await
    };
}

#[macro_export]
macro_rules! delete {
    ($bot:expr, $msg:expr) => {
        $bot.delete_message($msg.chat.id, $msg.id).await
    };
    ($bot:expr, $chat:expr, $del:expr) => {
        $bot.delete_message($chat.chat.id, $del.id).await
    };
}

#[macro_export]
macro_rules! image {
    ($bot:expr, $chat:expr, $file:expr) => {
        $bot.send_photo($chat.chat.id, $file)
            .reply_parameters(ReplyParameters::new($chat.id))
            .await
    };
}
