[![Crate](https://img.shields.io/crates/v/telexide?style=flat-square)](https://crates.io/crates/telexide)
[![Docs](https://docs.rs/telexide/badge.svg)](https://docs.rs/telexide)
[![Build Status](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Fcallieve%2Ftelexide%2Fbadge&style=flat-square)](https://actions-badge.atrox.dev/callieve/telexide/goto)
[![Rust Version](https://img.shields.io/badge/rust-1.46.0+-93450a.svg?style=flat-square)](https://blog.rust-lang.org/2020/01/30/Rust-1.46.0.html)

# telexide

Telexide is an easy to use library for making a telegram bot, built on tokio and hyper.

View the [examples] on how to make and structure a bot.

Use the [`ClientBuilder`] to easily create a [`Client`] object to your
preferences and register commands with the [`create_framework`] macro and/or
register your own update handlers, before running [`Client::start`] to start
your bot. All of this is designed to be highly customisable. For further
information about the client, please see the [client's module-level
documentation][client].

API calls are easy to make using the [`APIClient`] and the api data models,
or create and use your own api client by implementing the [`API`] trait. For
further information about the api client, please see the [api's module-level
documentation][api].

A default command framework is provided using the [`Framework`] object,
providing easy handling of incoming [telegram bot commands][tg_commands]
sent by users of your bot. For further information about the framework,
please see the [framework's module-level documentation][framework].

## Example Bot

A basic ping-pong bot can be written like:

```rust
use std::env;
use telexide_fork::{api::types::SendMessage, prelude::*};

#[command(description = "just a ping-pong command")]
async fn ping(context: Context, message: Message) -> CommandResult {
    context
        .api
        .send_message(SendMessage::new(message.chat.get_id(), "pong"))
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() -> telexide_fork::Result<()> {
    let token = env::var("BOT_TOKEN").expect("no token environment variable set");

    ClientBuilder::with_framework(
        create_framework!("ping-pong", ping),
        token
    ).start().await
}
```

For more examples, please see the examples dir.

## Features

- [x] easy to use and customisable client
- [x] long-polling based update handling
  - [x] set your own timeout
  - [x] set your own limit for updates gotten at once
- [x] easy to use, macro-based command framework
- [x] easy to use and heavily customisable api client
  - [x] use your own hyper client
  - [x] use your own api struct so you control the get and post methods
  - [x] includes all telegram api endpoints
- [x] webhook based update handling

#### Planned:

- [ ] choosing what you want from the lib using feature flags
- [ ] subscribe to non-message events using command (or similar) framework
  - [ ] run command on receiving an inline query or answer to one
  - [ ] run command on receiving a poll that matches your requirements
- [ ] wait_for style Context method to wait for an update matching your criteria
- [ ] more builder methods for creating api data
- [ ] methods on models for easier calling of API endpoints (like `ChatMember::kick`)

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
telexide = "0.1.6"
```

## Supported Rust Versions

The minimum supported version is 1.46. The current Telexide version is not guaranteed to build on Rust versions earlier than the minimum supported version.

[examples]: https://github.com/callieve/telexide/blob/master/examples
[client]: https://docs.rs/telexide/*/telexide/client/index.html
[`clientbuilder`]: https://docs.rs/telexide/*/telexide/client/struct.ClientBuilder.html
[`client`]: https://docs.rs/telexide/*/telexide/client/struct.Client.html
[`client::start`]: https://docs.rs/telexide/*/telexide/client/struct.Client.html#method.start
[`apiclient`]: https://docs.rs/telexide/*/telexide/api/struct.APIClient.html
[`api`]: https://docs.rs/telexide/*/telexide/api/trait.API.html
[api]: https://docs.rs/telexide/*/telexide/api/index.html
[`create_framework`]: https://docs.rs/telexide/*/telexide/macro.create_framework.html
[tg_commands]: https://core.telegram.org/bots#commands
[`framework`]: https://docs.rs/telexide/*/telexide/framework/struct.Framework.html
[framework]: https://docs.rs/telexide/*/telexide/framework/index.html
