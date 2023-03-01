# traq-ws-bot
[doc](https://docs.rs/traq-ws-bot/latest/traq_ws_bot/)

## 使用法
**Warning:** 現状の examples は動かないため修正が必要  
ref: [examples](../examples/)

Go で作成する方法は [traP 内 wiki に記載あり](https://wiki.trap.jp/user/toki/memo/5%E7%A7%92%E3%81%A7%E4%BD%9C%E3%82%8BtraQ-bot)

ここでは Rust の使用方法がわかる人向けに BOT の作成から動かすまでを説明します

1. [traQ BOT Console](https://bot-console.trap.jp/) の BOTs から BOT を作成する  
  注: この際動作モードを **WebSocket** にする必要があります
2. bot-console から BOT が受け取るイベントを設定する  
  詳しいイベントの種類は [BOT イベントリファレンス](https://bot-console.trap.jp/docs/bot/events)参照  
  echo-bot を動かすだけなら DIRECT_MESSAGE_CREATED にチェックをいれれば OK
3. 動かすだけなら、 [examples/echo-bot-openapi](../examples/echo-bot-openapi/) の内容を手元にコピーして、Cargo.toml の traq-ws-bot の依存関係を修正し、bot-console からアクセストークンを `.env` に `BOT_ACCESS_TOKEN=トークン` の形で記述すれば `cargo run` をすると動きます

### BOT の構築方法
BOT は `traq_ws_bot::bot::builder` を用いて作成されます。  
作成時には BOT アクセストークンが必要で、ここで設定した値は後述するハンドラー内から読み取ることはできず、イベントを取得するためのみに使われます。

BOT は唯一つの Resource を持つことができ、`.insert_resource(リソース)` の形で設定することができます。複数回呼ばれた場合は最後ののみが採用されます。  
リソースは `Send + Sync + 'static` である必要がありますが、Immutable で十分ならば `Arc` でのラップなどで一般的な構造体を持つことができます。  
`.insert_resource` を行ったあとは、それより前に設定されていたハンドラーは resource の使用の有無に関わらず削除されるため、`builder` の直後のチェーンで呼ぶことを強く推奨します。

BOT には無制限にハンドラーを追加することができます。リソースを使用しないハンドラーは `.on_event_name(ハンドラー)` のように、リソースを使用するハンドラーは `.on_event_name_with_resource(ハンドラー)` のように記述できます。  
ハンドラーは通常の関数か、環境をキャプチャしないクロージャを使用できます。  
`.on_event(Key, ハンドラー)` を用いることで、複数のイベント・すべてのイベント に対して一つのハンドラーを用いてまとめて設定することもできます。

一般的な Tips として、message の投稿のイベントなら `message.user.bot` のように、BOT に起因するイベントかどうかを取得できるため、無限ループの対策のために BOT からのメッセージであれば返信をしないといった処理を handler 内で行ったほうがいいです。ｌ

### API の叩き方
通常は reqwest などのてきとうな API クライアントを用いて [API Document](https://apis.trap.jp/) のリンクを叩くことで、叩けます。この場合は `traq_ws_bot::utils::create_client` を用いると少し楽にクライアントを生成できます。

Open API による自動生成のコードを用いることもでき、`traq_ws_bot` の `openapi` features を有効にした上で、`traq_ws_bot::openapi::apis` 以下の関数を用いることが可能です。  
この場合は `traq_ws_bot::utils::create_configuration` を用いると少し楽に configuration を構築することもできます。

### デプロイ方法
WIP

https://github.com/SSlime-s/BOT_SSlime/blob/main/showcase.yaml や https://github.com/SSlime-s/BOT_SSlime/tree/main/.github を参考に書く
