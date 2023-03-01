# traq-ws-bot-rs
この README は開発用のメモです
traq-ws-bot の crate の使い方は [bot-core](./bot-core/) へ

## openapi crate の生成方法

1. `openapi-generator-cli generate -i https://raw.githubusercontent.com/traPtitech/traQ/master/docs/v3-api.yaml -g rust`
2. 一部の型がうまくリンクされないので、 `cargo check` 等ででるエラーを修正
