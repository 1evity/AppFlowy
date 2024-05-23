use client_api::entity::{MessageCursor, RepeatedChatMessage};
use flowy_chat_pub::cloud::{ChatCloudService, QAChatMessage};
use flowy_error::FlowyError;
use lib_infra::future::FutureResult;

pub(crate) struct DefaultChatCloudServiceImpl;

impl ChatCloudService for DefaultChatCloudServiceImpl {
  fn create_chat(
    &self,
    _uid: &i64,
    _workspace_id: &str,
    _chat_id: &str,
  ) -> FutureResult<(), FlowyError> {
    FutureResult::new(async move {
      Err(FlowyError::not_support().with_context("Chat is not supported in local server."))
    })
  }

  fn send_message(
    &self,
    workspace_id: &str,
    chat_id: &str,
    message: &str,
    require_answer: bool,
  ) -> FutureResult<QAChatMessage, FlowyError> {
    FutureResult::new(async move {
      Err(FlowyError::not_support().with_context("Chat is not supported in local server."))
    })
  }

  fn get_chat_messages(
    &self,
    _workspace_id: &str,
    _chat_id: &str,
    _offset: MessageCursor,
    _limit: u64,
  ) -> FutureResult<RepeatedChatMessage, FlowyError> {
    FutureResult::new(async move {
      Err(FlowyError::not_support().with_context("Chat is not supported in local server."))
    })
  }
}
