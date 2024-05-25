use std::sync::Weak;

use strum_macros::Display;

use flowy_derive::{Flowy_Event, ProtoBuf_Enum};
use lib_dispatch::prelude::*;

use crate::event_handler::*;
use crate::manager::ChatManager;

pub fn init(chat_manager: Weak<ChatManager>) -> AFPlugin {
  AFPlugin::new()
    .name("Flowy-Chat")
    .state(chat_manager)
    .event(ChatEvent::SendMessage, send_chat_message_handler)
    .event(ChatEvent::LoadMessage, load_message_handler)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Display, Hash, ProtoBuf_Enum, Flowy_Event)]
#[event_err = "FlowyError"]
pub enum ChatEvent {
  /// Create a new workspace
  #[event(input = "LoadChatMessagePB", output = "ChatMessageListPB")]
  LoadMessage = 0,

  #[event(input = "SendChatPayloadPB", output = "RepeatedChatMessagePB")]
  SendMessage = 1,
}
