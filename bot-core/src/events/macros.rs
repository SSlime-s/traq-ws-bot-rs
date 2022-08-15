/// 汎用的な derive と 全 field に pub をつけるマクロ
/// field 名に as を用いることで、serialize/deserialize に変換する際のキーを指定できる
macro_rules! gp_struct {
    // ($name:ident { $( $field:ident $(as $rename:tt)? : $type:ty ),* $(,)? }) => {
    //     #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    //     pub struct $name {
    //         $(
    //             $(#[serde(rename = $rename)])?
    //             pub $field: $type,
    //         )*
    //     }
    // };

    // ($(struct $name:ident { $( $field:ident $(as $rename:tt)? : $type:ty ),* $(,)? })+) => {
    //     $(
    //         #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    //         pub struct $name {
    //             $(
    //                 $(#[serde(rename = $rename)])?
    //                 pub $field: $type,
    //             )*
    //         }
    //     )+
    // };

    ($(struct $name:ident {  $( $(#[$attr:meta])* $field:ident: $type:ty ),* $(,)? })+) => {
        $(
            #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            pub struct $name {
                $(
                    $(#[$attr])*
                    pub $field: $type,
                )*
            }
        )+
    };
}

pub(super) use gp_struct;
