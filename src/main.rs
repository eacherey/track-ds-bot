use dotenvy::{dotenv, var};
use serenity::all::{
    ChannelId, Context, CreateMessage, EventHandler, GatewayIntents, GuildId,
     Member, Ready, Timestamp, User,
};
use serenity::async_trait;
use serenity::builder::{CreateEmbed, CreateEmbedFooter};
use serenity::model::Colour;
use serenity::prelude::*;

struct Handler {
    log_channel_message_edits: ChannelId,
    log_channel_member_events: ChannelId,
    log_channel_role_changes: ChannelId,
}

impl Handler {
    fn new() -> Self {
        Self {
            log_channel_message_edits: var("LOG_CHANNEL_MESSAGE_EDITS")
                .expect("LOG_CHANNEL_MESSAGE_EDITS not set")
                .parse()
                .expect("Invalid LOG_CHANNEL_MESSAGE_EDITS"),
            log_channel_member_events: var("LOG_CHANNEL_MEMBER_EVENTS")
                .expect("LOG_CHANNEL_MEMBER_EVENTS not set")
                .parse()
                .expect("Invalid LOG_CHANNEL_MEMBER_EVENTS"),
            log_channel_role_changes: var("LOG_CHANNEL_ROLE_CHANGES")
                .expect("LOG_CHANNEL_ROLE_CHANGES not set")
                .parse()
                .expect("Invalid LOG_CHANNEL_ROLE_CHANGES"),
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, member: Member) {
        let channel_id = self.log_channel_member_events;

        let description = format!(
            "**Пользователь:** {} (`{}`)\n\
            **ID:** `{}`\n\
            **Бот:** {} | **Системный:** {}\n\
            **Дата регистрации:** <t:{}:F> (<t:{}:R>)\n\
            **Дата входа:** <t:{}:F> (<t:{}:R>)",
            member.user.global_name.as_deref().unwrap_or("Не указано"),
            member.user.tag(),
            member.user.id,
            if member.user.bot { "Да" } else { "Нет" },
            if member.user.system { "Да" } else { "Нет" },
            member.user.created_at().unix_timestamp(),
            member.user.created_at().unix_timestamp(),
            member.joined_at.map(|t| t.unix_timestamp()).unwrap_or(0),
            member.joined_at.map(|t| t.unix_timestamp()).unwrap_or(0),
        );

        let embed = CreateEmbed::new()
            .title("🛡️ Информация о новом участнике")
            .description(description)
            .color(Colour::from_rgb(233, 30, 99))
            .thumbnail(member.user.face())
            .footer(CreateEmbedFooter::new(format!(
                "Guild ID: {}",
                member.guild_id
            )))
            .timestamp(Timestamp::now());

        if let Err(why) = channel_id
            .send_message(&ctx, CreateMessage::new().embed(embed))
            .await
        {
            eprintln!("❌ Error send log: {why:?}");
        }
    }

    async fn guild_member_removal(
        &self,
        ctx: Context,
        guild_id: GuildId,
        user: User,
        member_data_if_available: Option<Member>,
    ) {
        let channel_id = self.log_channel_member_events;

        let joined_timestamp = member_data_if_available
            .and_then(|m| m.joined_at)
            .map(|t| t.unix_timestamp())
            .unwrap_or(0);

        let description = format!(
            "**Пользователь:** {} (`{}`)\n\
            **ID:** `{}`\n\
            **Бот:** {} | **Системный:** {}\n\
            **Дата регистрации:** <t:{}:F> (<t:{}:R>)\n\
            **Дата входа:** {}",
            user.global_name.as_deref().unwrap_or("Не указано"),
            user.tag(),
            user.id,
            if user.bot { "Да" } else { "Нет" },
            if user.system { "Да" } else { "Нет" },
            user.created_at().unix_timestamp(),
            user.created_at().unix_timestamp(),
            if joined_timestamp > 0 {
                format!("<t:{}:F> (<t:{}:R>)", joined_timestamp, joined_timestamp)
            } else {
                "Данные недоступны".to_string()
            }
        );

        let embed = CreateEmbed::new()
            .title("🚪 Информация о вышедшем участнике")
            .description(description)
            .color(Colour::from_rgb(150, 75, 0))
            .thumbnail(user.face())
            .footer(CreateEmbedFooter::new(format!("Guild ID: {}", guild_id)))
            .timestamp(Timestamp::now());

        if let Err(why) = channel_id
            .send_message(&ctx, CreateMessage::new().embed(embed))
            .await
        {
            eprintln!("❌ Error send log: {why:?}");
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("✅ {} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::MESSAGE_CONTENT;

    let handler = Handler::new();

    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        eprintln!("❌ Client failed to start: {why:?}");
    } else {
        println!("✅ Client started successfully");
    }
}
