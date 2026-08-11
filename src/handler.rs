use serenity::all::{Context, EventHandler, GuildId, Member, Ready, User};
use serenity::async_trait;

use crate::config::Config;

pub struct Handler {
    pub config: Config,
}

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, member: Member) {
        crate::events::member_join::handle(&ctx, &self.config, member).await;
    }

    async fn guild_member_removal(
        &self,
        ctx: Context,
        guild_id: GuildId,
        user: User,
        member_data_if_available: Option<Member>,
    ) {
        crate::events::member_leave::handle(
            &ctx,
            &self.config,
            guild_id,
            user,
            member_data_if_available,
        )
        .await;
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("✅ {} is connected!", ready.user.name);
    }
}
