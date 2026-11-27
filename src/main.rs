use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, MaybeInaccessibleMessage},
};
use tokio::{io::AsyncWriteExt, net::TcpListener};

const INVITE_LINK: &str = "https://t.me/+wX9AhZCV3YhhZWY0";

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
    let user_name = msg
        .from
        .as_ref()
        .map(|u| u.first_name.as_str())
        .unwrap_or("друг");

    let text = format!(
        "Здравствуйте, {}! Рад видеть вас здесь.\n\n\
        Приглашаем вас присоединиться к нашему дружному сообществу игроков FR Legends — «FR Legends GC». \
        Нажмите на кнопку ниже, чтобы перейти в чат:",
        user_name
    );

    let keyboard = InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::url(
            "Вступить в FR Legends GC",
            INVITE_LINK.parse().unwrap(),
        )],
        vec![InlineKeyboardButton::callback(
            "О нашем сообществе",
            "about_community",
        )],
    ]);

    bot.send_message(msg.chat.id, text)
        .reply_markup(keyboard)
        .await?;

    Ok(())
}

async fn callback_handler(bot: Bot, q: CallbackQuery) -> ResponseResult<()> {
    if q.data.as_deref() == Some("about_community") {
        bot.answer_callback_query(q.id.clone()).await?;

        if let Some(MaybeInaccessibleMessage::Regular(msg)) = q.message {
            let info_text = "«FR Legends GC» — это уютное сообщество игроков FR Legends, \
                где можно обсудить дрифт-сборки, поделиться ливреями, найти напарников для заездов и просто пообщаться.\n\n\
                Будем очень рады видеть вас в наших рядах!";

            let keyboard = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::url(
                "Перейти в чат сообщества",
                INVITE_LINK.parse().unwrap(),
            )]]);

            bot.send_message(msg.chat.id, info_text)
                .reply_markup(keyboard)
                .await?;
        }
    }
    Ok(())
}
