use serenity::all::{Colour, Context, CreateMessage, GuildId, Member, User};

use crate::config::Config;
use crate::utils::{create_member_description, create_member_embed};

pub async fn handle(
    ctx: &Context,
    config: &Config,
    guild_id: GuildId,
    user: User,
    member_data_if_available: Option<Member>,
) {
    let channel_id = config.log_channel_member_events;

    let joined_timestamp = member_data_if_available
        .and_then(|m| m.joined_at)
        .map(|t| t.unix_timestamp())
        .unwrap_or(0);

    let joined_timestamp = if joined_timestamp > 0 {
        format!("<t:{}:F> (<t:{}:R>)", joined_timestamp, joined_timestamp)
    } else {
        "Данные недоступны".to_string()
    };

    let description = create_member_description(&user, joined_timestamp);

    let embed = create_member_embed(
        "🚪 Информация о вышедшем участнике".to_string(),
        description,
        Colour::from_rgb(150, 75, 0),
        user,
        guild_id,
    );

    if let Err(why) = channel_id
        .send_message(&ctx, CreateMessage::new().embed(embed))
        .await
    {
        eprintln!("❌ Error send log: {why:?}");
    }
}
