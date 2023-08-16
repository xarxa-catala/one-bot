use dotenvy::dotenv;
use teloxide::prelude::*;
use teloxide::types::{MessageKind, ParseMode};

const OP_GROUP_ID_ENV: &str = "OP_GROUP_ID";

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    let op_group_id: i64 = std::env::var(OP_GROUP_ID_ENV)
        .expect("OP group ID is not set in the environment variables")
        .parse()
        .expect("OP group ID format is not valid. It must be a 64 integer.");

    let bot = Bot::from_env();


    teloxide::repl(bot, move |bot: Bot, msg: Message| async move {
        if let MessageKind::NewChatMembers(chat_member) = msg.kind {
            if msg.chat.id.0 == op_group_id {
                let name = chat_member.new_chat_members
                    .first()
                    .unwrap()
                    .first_name
                    .clone();

                let text = format!(r#" Hola, {name},
- No tenim els episodis en català i no els podeu demanar pel grup, busqueu a Google.
- No repenjarem els episodis malgrat ja no estiguin disponibles; ho farem quan One Piece no estigui en emissió.
- Llegiu-vos les <a href="https://t.me/onepiececatala/1/124586">normes</a>.
"#);

                bot.send_message(msg.chat.id, text)
                    .parse_mode(ParseMode::Html)
                    .await?;
            }
        }

        Ok(())
    }).await;
}