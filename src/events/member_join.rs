use serenity::all::{Colour, Context, CreateMessage, Member};

use crate::config::Config;
use crate::utils::{create_member_description, create_member_embed};

pub async fn handle(ctx: &Context, config: &Config, member: Member) {
    let channel_id = config.log_channel_member_events;

    let joined_timestamp = format!(
        "<t:{}:F> (<t:{}:R>)",
        member.joined_at.map(|t| t.unix_timestamp()).unwrap_or(0),
        member.joined_at.map(|t| t.unix_timestamp()).unwrap_or(0)
    );

    let description = create_member_description(&member.user, joined_timestamp);

    let embed = create_member_embed(
        "🛡️ Информация о новом участнике".to_string(),
        description,
        Colour::from_rgb(233, 30, 99),
        member.user,
        member.guild_id,
    );

    if let Err(why) = channel_id
        .send_message(&ctx, CreateMessage::new().embed(embed))
        .await
    {
        eprintln!("❌ Error send log: {why:?}");
    }
}
