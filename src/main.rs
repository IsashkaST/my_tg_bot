use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
};

#[tokio::main]
async fn main() {
    let bot = Bot::from_env();

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_callback_query().endpoint(callback_handler));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
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

        if let Some(msg) = q.message {
            bot.send_message(msg.chat().id, "Кнопка на Rust сработала!")
                .await?;
        }
    }
    Ok(())
}
