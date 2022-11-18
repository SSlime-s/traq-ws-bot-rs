//!
//!

pub mod bot;
pub mod events;
pub mod utils;

#[cfg(feature = "openapi")]
pub use openapi;

#[cfg(test)]
mod tests {
    use crate::events::{self, common::Base, payload::TagRemoved, Events};

    #[test]
    fn it_works() {
        let events = Events::TagRemoved(TagRemoved {
            base: Base {
                event_time: "2020-01-01T00:00:00.000Z".parse().unwrap(),
            },
            tag_id: "tag_id_tmp".to_string(),
            tag: "tag_name_tmp".to_string(),
        });
        assert_eq!(
            format!("{}", serde_json::to_string(&events).unwrap()),
            String::new()
        );
    }
}
