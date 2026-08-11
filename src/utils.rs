use serenity::all::{Colour, CreateEmbed, CreateEmbedFooter, GuildId, Timestamp, User};

pub fn create_embed(
    title: String,
    description: String,
    color: Colour,
    user: User,
    guild_id: GuildId,
) -> CreateEmbed {
    CreateEmbed::new()
        .title(title)
        .description(description)
        .color(color)
        .thumbnail(user.face())
        .footer(CreateEmbedFooter::new(format!("Guild ID: {}", guild_id)))
        .timestamp(Timestamp::now())
}

pub fn create_description(user: &User, joined_timestamp: String) -> String {
    format!(
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
        joined_timestamp,
    )
}
