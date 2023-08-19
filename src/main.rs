use dotenvy::dotenv;
use teloxide::prelude::*;
use teloxide::types::{MessageKind, ParseMode};

const OP_GROUP_ID_ENV: &str = "OP_GROUP_ID";
const WELCOME_MSG_BODY: &str = r#"
Sigues benvingut/da a la comunitat de One Piece Català. Esperem que t'hi trobis a gust i comparteixis bones estones amb tothom.

⚠️ És important per a nosaltres deixar clars tres punts, abans de començar:
- <b>No tenim els episodis en català</b> i no els podeu demanar pel grup, <b>busqueu a Google</b>.
- No repenjarem els episodis malgrat ja no estiguin disponibles; ho farem quan One Piece no estigui en emissió.
- ❗️ Llegiu-vos les <a href="https://t.me/onepiececatala/1/124586">normes</a>, si us plau ❗️.

Molt bona estada!
"#;

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
                let user = chat_member.new_chat_members
                    .first();

                if let Some(user) = user {
                    let name = &user.first_name;
                    let id = &user.id.0;
                    let text = format!("Hola, <a href=\"tg://user?id={id}\">{name}</a>,\n{WELCOME_MSG_BODY}");

                    bot.send_message(msg.chat.id, text)
                        .parse_mode(ParseMode::Html)
                        .await?;
                }
            }
        }

        Ok(())
    }).await;
}