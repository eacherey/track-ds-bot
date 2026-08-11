use dotenvy::{dotenv, var};
use serenity::all::ChannelId;

pub struct Config {
    pub log_channel_message_edits: ChannelId,
    pub log_channel_member_events: ChannelId,
    pub log_channel_role_changes: ChannelId,
}

impl Config {
    pub fn from_env() -> Self {
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
