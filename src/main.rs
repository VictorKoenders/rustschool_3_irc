//! This crate has an example IRC bot, and a lot of comments to explain what's going on.
//!
//! TIP: run `cargo doc --open` in your terminal to open this help in a local web browser. This will also include all the documentation for all the crates used in this project.
//!
//! Useful resources:
//! - "the book": [https://doc.rust-lang.org/book/](https://doc.rust-lang.org/book/)
//! - This contains a _lot_ of useful information for anyone to get started with the language
//!
//! Rust by example: [https://doc.rust-lang.org/rust-by-example/](https://doc.rust-lang.org/rust-by-example/)
//! - Contains a lot of examples to explain the language and how to solve problems.
//!
//! Documentation of "crates": [https://docs.rs/](https://docs.rs/)
//!
//! For more info on crates, see chapter 14: [https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html](https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html)
//!
//! Crates used in this bot:
//! - irc: [https://docs.rs/irc/0.13.6/irc/](https://docs.rs/irc/0.13.6/irc/)
//! - reqwest: [https://docs.rs/reqwest/0.9.8/reqwest/](https://docs.rs/reqwest/0.9.8/reqwest/)
//! - chrono: [https://docs.rs/chrono/0.4.6/chrono/](https://docs.rs/chrono/0.4.6/chrono/)
//! - serde_json: [https://docs.rs/serde_json/1.0.36/serde_json/](https://docs.rs/serde_json/1.0.36/serde_json/)
//! - rand: [https://docs.rs/rand/0.6.4/rand/](https://docs.rs/rand/0.6.4/rand/)
//!
//! For more crates, visit [https://crates.io/](https://crates.io/)

const NICK: &str = "TestBot";
const SERVER: &str = "irc.smurfnet.ch";
const CHANNEL: &str = "#rustschool";

use irc::client::prelude::*;

type Result<T> = std::result::Result<T, failure::Error>;

/// The main entry point of this bot.
///
/// This sets up a bot by calling [get_irc_client](get_irc_client).
///
/// Then, for each message this bot receives, it'll call [handle_message](handle_message).
pub fn main() {
    // Create our bot, see the bottom of this file on how the bot gets created
    let client = get_irc_client();

    // This will start our bot. Each message that it receives, will call function `handle_message`
    // These messages are of type `Message`:
    // https://docs.rs/irc/0.13.6/irc/proto/message/struct.Message.html
    client
        .for_each_incoming(|irc_msg| handle_message(&client, irc_msg))
        .unwrap();
}

/// Handle a single message that our IRC bot receives
///
/// `client` is of type IrcClient: [https://docs.rs/irc/0.13.6/irc/client/struct.IrcClient.html](https://docs.rs/irc/0.13.6/irc/client/struct.IrcClient.html)
///
/// On that page, make sure to scroll down to the code "impl Client for IrcClient" for more functions.
///
/// Advanced: For more info on traits, read the book chapter 10.2: [https://doc.rust-lang.org/book/ch10-02-traits.html](https://doc.rust-lang.org/book/ch10-02-traits.html)
///
/// `irc_msg` is of type `Message`: [https://docs.rs/irc/0.13.6/irc/proto/message/struct.Message.html](https://docs.rs/irc/0.13.6/irc/proto/message/struct.Message.html)
pub fn handle_message(client: &IrcClient, irc_msg: Message) {
    // irc_msg has a field called `command`
    // This field is an enum, which can be one of several possible values.
    // You can find all values here: https://docs.rs/irc/0.13.6/irc/proto/command/enum.Command.html
    // More info on enums see chapter 6 in the book: https://doc.rust-lang.org/book/ch06-00-enums.html
    if let Command::PRIVMSG(channel, message) = irc_msg.command {
        // `message` is a String: https://doc.rust-lang.org/std/string/struct.String.html
        // ( Which also implements everything of &str: https://doc.rust-lang.org/std/primitive.str.html )
        // It has methods like `contains` and `starts_with` and `trim`

        // `channel` is also a String. This can be one of the following values:
        // - If someone types in a channel this bot is in (like #rustschool), it'll start with a # followed by the channel name
        // - If someone sends you a message directly, it'll be the full name of that user
        // Regardless of which one it is, you can just do `client.send_privmsg` to reply wherever the message comes from
        if message.contains(client.current_nickname()) {
            // TIP: Because this can fail, for example if you lose internet, this returns a result.
            // For now we're ignoring this and allowing our application to crash if this fails.
            // For more info, see chapter 9.2 in the book: https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
            client.send_privmsg(&channel, "beep boop").unwrap();
        }

        // Uncomment this to allow your bot to reply to the command `!time` with the current time
        /*
        if message.starts_with("!time") {
            let response = get_current_time();
            client.send_privmsg(&channel, response).unwrap();
        }
        */

        // Uncomment this to allow your bot to retreive a cat fact from https://alexwohlbruck.github.io/cat-facts/
        // whenever someone says `!catfact`
        /*
        if message.starts_with("!catfact") {
            let response = get_cat_fact().expect("Could not get cat fact");
            client.send_privmsg(&channel, response).unwrap();
        }
        */
    }
}

/// Get the current time
///
/// This is done by using a crate called `chrono`
///
/// For more info, see [https://docs.rs/chrono/0.4.6/chrono/]([https://docs.rs/chrono/0.4.6/chrono/)
pub fn get_current_time() -> String {
    let time = chrono::Local::now();
    format!("The current time is {}", time)
}

/// Get a cat fact!
///
/// This queries the API endpoint [https://cat-fact.herokuapp.com/facts](https://cat-fact.herokuapp.com/facts), which results a JSON array of facts
///
/// We use reqwest to query this endpoint: [https://docs.rs/reqwest/0.9.8/reqwest/](https://docs.rs/reqwest/0.9.8/reqwest/)
///
/// And then we use serde_json to deserialize the JSON to code: [https://docs.rs/serde_json/1.0.36/serde_json/](https://docs.rs/serde_json/1.0.36/serde_json/)
pub fn get_cat_fact() -> Result<String> {
    use failure::{bail, format_err};
    use rand::Rng;
    use serde_json::Value;

    // `?` is a language construct that roughly means "if this fails, return the error"
    // See chapter 9.2 for more info: https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator
    // The actual implementation is: https://doc.rust-lang.org/src/core/macros.rs.html#299-307
    // the expanded code would be something like:
    // `match reqwest::get(..) { Ok(response) => response, Err(e) => return Err(From::from(e)) }`
    // So doing `reqwest::get(..)?` is a bit shorter, but still the exact same
    let response = reqwest::get("https://cat-fact.herokuapp.com/facts")?.text()?;
    let json: Value = serde_json::from_str(&response)?;

    // The json is now in a wrapper called `Value`: https://docs.rs/serde_json/1.0.36/serde_json/enum.Value.html
    // You can go to https://cat-fact.herokuapp.com/facts and see the json for yourself

    // First off, the json is an object with a single field: "all"
    // We can easily get this field by using the `.get` function: https://docs.rs/serde_json/1.0.36/serde_json/enum.Value.html#method.get

    // Because we do `Some(Value::Array(s))`, this `if` only passes if the json value is an actual array
    // This makes sure we don't accidentally have to deal with a string or number, or null
    if let Some(Value::Array(facts)) = json.get("all") {
        // Pick a random index
        // We use the `rand` crate for this. For more information, see https://docs.rs/rand/0.6.4/rand/
        let number_of_facts = facts.len();
        let fact_index = rand::thread_rng().gen_range(0, number_of_facts);

        let fact = &facts[fact_index];
        // We now have a cat fact, which is an object like:
        // {"_id":"5894af975cdc7400113ef7f9","text":"The technical term for a cat’s hairball is a bezoar.","user":{"_id":"587288f6e6f85e64ae1c7ef7","name":{"first":"Alex","last":"Wohlbruck"}},"upvotes":[{"user":"5872812829f7f43cacb0c4de"}]}
        // We only care about the "text" field, so we do the same thing as above
        if let Some(Value::String(s)) = fact.get("text") {
            // We now have the text of our cat fact!
            // Fun fact and self-praise: when I was writing this code, this worked on the first try.

            // now, becauase the return type of this function is `Result<String>` instead of `String`, we need to wrap our string in a `Result`
            // Result has two values: `Err` or `Ok`.
            // Because we succeeded in getting our cat fact, we'll return `Ok(String)` and let the caller know we succeeded.

            // Rust has an implicit return: https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#functions-with-return-values
            // Because all the paths in this function return a Result, we can just simply say `x` instead of `return x;`
            Ok(format!("Did you know that {}", s))
        } else {
            // We could not get the text value of the cat. This means we have to return an error with a message.
            // https://docs.rs/failure/0.1.5/failure/macro.format_err.html
            Err(format_err!("We loaded the cat facts succesfully, but we couldn't get the text value of the fact"))
        }
    } else {
        // We could not get the json data. This means we have to return an error with a message.
        // This is an alternative, but equivalent, of the `Err(format_err!(...))` above.
        // https://docs.rs/failure/0.1.5/failure/macro.bail.html
        bail!("We could not load the json value of the cat facts")
    }
}

/// Get the IRC client to start this bot
///
/// This simply creates a config based on the global variables at the top of this file, and connects to the network
///
/// See the irc docs for more info: [https://docs.rs/irc/0.13.6/irc/](https://docs.rs/irc/0.13.6/irc/)
pub fn get_irc_client() -> IrcClient {
    use irc::client::data::config::Config;

    let client = IrcClient::from_config(Config {
        server: Some(SERVER.to_string()),
        nickname: Some(NICK.to_string()),
        channels: Some(vec![CHANNEL.to_string()]),
        // All config options can be found here: https://docs.rs/irc/0.13.6/irc/client/data/config/struct.Config.html
        ..Default::default()
    })
    .unwrap();
    client.identify().unwrap();
    client
}
