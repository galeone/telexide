use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{
    message_contents::*, message_entity::*, utils::unix_date_formatting, CallbackQuery,
    ChatLocation, ChatMemberUpdated, ChatPhoto, ChatType, ChosenInlineResult, Game,
    InlineKeyboardMarkup, InlineQuery, Invoice, PassportData, PreCheckoutQuery, ShippingQuery,
    Sticker, SuccessfulPayment, User,
};

/// The raw message, for most usages the [`Message`] object is easier to use
///
/// [`Message`]: super::Message
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RawMessage {
    pub message_id: i64,
    pub from: Option<super::User>,
    pub sender_chat: Option<RawChat>,
    #[serde(with = "unix_date_formatting")]
    pub date: DateTime<Utc>,
    pub chat: RawChat,

    pub forward_from: Option<super::User>,
    pub forward_from_chat: Option<RawChat>,
    pub forward_from_message_id: Option<i64>,
    pub forward_signature: Option<String>,
    pub forward_sender_name: Option<String>,
    #[serde(default)]
    #[serde(with = "unix_date_formatting::optional")]
    pub forward_date: Option<DateTime<Utc>>,

    pub reply_to_message: Option<Box<RawMessage>>,
    pub via_bot: Option<User>,

    #[serde(default)]
    #[serde(with = "unix_date_formatting::optional")]
    pub edit_date: Option<DateTime<Utc>>,

    pub media_group_id: Option<String>,
    pub author_signature: Option<String>,

    pub text: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub audio: Option<Audio>,
    pub document: Option<Document>,
    pub animation: Option<Animation>,
    pub game: Option<Game>,
    pub photo: Option<Vec<PhotoSize>>,
    pub sticker: Option<Sticker>,
    pub video: Option<Video>,
    pub voice: Option<Voice>,
    pub video_note: Option<VideoNote>,
    pub caption: Option<String>,
    pub contact: Option<Contact>,
    pub location: Option<Location>,
    pub venue: Option<Venue>,
    pub poll: Option<Poll>,
    pub dice: Option<Dice>,
    pub new_chat_members: Option<Vec<User>>,
    pub left_chat_member: Option<User>,
    pub new_chat_title: Option<String>,
    pub new_chat_photo: Option<Vec<PhotoSize>>,

    #[serde(default)]
    pub delete_chat_photo: bool,
    #[serde(default)]
    pub group_chat_created: bool,
    #[serde(default)]
    pub supergroup_chat_created: bool,
    #[serde(default)]
    pub channel_chat_created: bool,

    pub message_auto_delete_timer_changed: Option<MessageAutoDeleteTimerChanged>,

    pub migrate_to_chat_id: Option<i64>,
    pub migrate_from_chat_id: Option<i64>,

    pub pinned_message: Option<Box<RawMessage>>,
    pub invoice: Option<Invoice>,
    pub successful_payment: Option<SuccessfulPayment>,

    pub connected_website: Option<String>,
    pub passport_data: Option<PassportData>,
    pub proximity_alert_triggered: Option<ProximityAlertTriggered>,
    pub reply_markup: Option<InlineKeyboardMarkup>,

    pub voice_chat_scheduled: Option<VoiceChatScheduled>,
    pub voice_chat_started: Option<VoiceChatStarted>,
    pub voice_chat_ended: Option<VoiceChatEnded>,
    pub voice_chat_participants_invited: Option<VoiceChatParticipantsInvited>,
}

/// The raw chat, for most usages the [`Chat`] object is easier to use
///
/// [`Chat`]: super::Chat
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RawChat {
    /// Unique identifier for this chat
    pub id: i64,
    #[serde(rename = "type")]
    pub chat_type: ChatType,
    /// Title, for supergroups, channels and group chats
    pub title: Option<String>,
    /// Username, for private chats, supergroups and channels if available
    pub username: Option<String>,
    /// First name of the other party in a private chat
    pub first_name: Option<String>,
    /// Last name of the other party in a private chat
    pub last_name: Option<String>,
    /// Chat photo. Returned only in getChat.
    pub photo: Option<ChatPhoto>,
    /// Bio of the other party in a private chat. Returned only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub bio: Option<String>,
    /// Description, for groups, supergroups and channel chats. Returned only in
    /// [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub description: Option<String>,
    /// Chat invite link, for groups, supergroups and channel chats.
    pub invite_link: Option<String>,
    /// Pinned message, for groups, supergroups and channels. Returned only in
    /// [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub pinned_message: Option<Box<RawMessage>>,
    /// Default chat member permissions, for groups and supergroups. Returned
    /// only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub permissions: Option<super::ChatPermissions>,
    /// For supergroups, the minimum allowed delay between consecutive messages
    /// sent by each unpriviledged user. Returned only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub slow_mode_delay: Option<usize>,
    /// For supergroups, name of group sticker set. Returned only in
    /// [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub sticker_set_name: Option<String>,
    /// True, if the bot can change the group sticker set. Returned only in
    /// [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub can_set_sticker_set: Option<bool>,
    /// Unique identifier for the linked chat, i.e. the discussion group
    /// identifier for a channel and vice versa; for supergroups and channel
    /// chats. This identifier may be greater than 32 bits and some
    /// programming languages may have difficulty/silent defects in interpreting
    /// it. But it is smaller than 52 bits, so a signed 64 bit integer or
    /// double-precision float type are safe for storing this identifier.
    /// Returned only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub linked_chat_id: Option<i64>,
    /// For supergroups, the location to which the supergroup is connected.
    /// Returned only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub location: Option<ChatLocation>,
}

/// The raw update, for most usages the [`Update`] object is easier to use
///
/// [`Update`]: super::Update
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RawUpdate {
    /// The update's unique identifier. Update identifiers start from a certain
    /// positive number and increase sequentially. If there are no new
    /// updates for at least a week, then identifier of the next update will
    /// be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// New incoming message of any kind — text, photo, sticker, etc.
    pub message: Option<RawMessage>,
    /// New version of a message that is known to the bot and was edited.
    pub edited_message: Option<RawMessage>,
    /// New incoming channel post of any kind — text, photo, sticker, etc.
    pub channel_post: Option<RawMessage>,
    /// New version of a channel post that is known to the bot and was edited.
    pub edited_channel_post: Option<RawMessage>,
    /// New incoming inline query.
    pub inline_query: Option<InlineQuery>,
    /// The result of an inline query that was chosen by a user and sent to
    /// their chat partner.
    pub chosen_inline_result: Option<ChosenInlineResult>,
    /// New incoming callback query.
    pub callback_query: Option<CallbackQuery>,
    /// New incoming shipping query. Only for invoices with flexible price.
    pub shipping_query: Option<ShippingQuery>,
    /// New incoming pre-checkout query. Contains full information about
    /// checkout.
    pub pre_checkout_query: Option<PreCheckoutQuery>,
    /// New poll state. Bots receive only updates about stopped polls and polls,
    /// which are sent by the bot.
    pub poll: Option<Poll>,
    /// A user changed their answer in a non-anonymous poll. Bots receive new
    /// votes only in polls that were sent by the bot itself.
    pub poll_answer: Option<PollAnswer>,
    /// The bot's chat member status was updated in a chat. For private chats,
    /// this update is received only when the bot is blocked or unblocked by
    /// the user.
    pub my_chat_member: Option<ChatMemberUpdated>,
    /// A chat member's status was updated in a chat. The bot must be an
    /// administrator in the chat and must explicitly specify “chat_member”
    /// in the list of allowed_updates to receive these updates.
    pub chat_member: Option<ChatMemberUpdated>,
}
