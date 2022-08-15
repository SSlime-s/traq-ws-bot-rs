use crate::events::common::Message;

pub fn is_mentioned_message(message: &Message, target_user_id: &str) -> bool {
    message
        .embedded
        .iter()
        .any(|emb| emb.id.as_str() == target_user_id)
}
