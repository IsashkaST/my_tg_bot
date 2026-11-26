use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, MaybeInaccessibleMessage},
};
use tokio::{io::AsyncWriteExt, net::TcpListener};

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        let port = std::env::var("PORT").unwrap_or_else(|_| "10000".to_string());
        if let Ok(listener) = TcpListener::bind(format!("0.0.0.0:{}", port)).await {
            while let Ok((mut socket, _)) = listener.accept().await {
                let _ = socket
                    .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nOK")
                    .await;
            }
        }
    });

    let bot = Bot::from_env();

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_callback_query().endpoint(callback_handler));

    Dispatcher::builder(bot, handler)
        .build()
        .dispatch()
        .await;
}

async fn message_handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    let keyboard = InlineKeyboardMarkup::new(vec![vec![
        InlineKeyboardButton::callback("Нажми меня", "btn_pressed"),
        InlineKeyboardButton::url("Открыть сайт", "https://telegram.org".parse().unwrap()),
    ]]);

    bot.send_message(msg.chat.id, "Привет с Rust! Выбери действие:")
        .reply_markup(keyboard)
        .await?;

    Ok(())
}

async fn callback_handler(bot: Bot, q: CallbackQuery) -> ResponseResult<()> {
    if q.data.as_deref() == Some("btn_pressed") {
        bot.answer_callback_query(q.id.clone()).await?;

        if let Some(MaybeInaccessibleMessage::Regular(msg)) = q.message {
            bot.send_message(msg.chat.id, "Кнопка на Rust сработала!")
                .await?;
        }
    }
    Ok(())
}
