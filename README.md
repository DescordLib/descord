# descord
Descord is a discord api wrapper without async/await shit.
If you want multithreading, do it yourself.

## Example
```rust
use descord::*;
use descord::intents::GatewayIntent;

fn main() {
    let token = "Discord token here";

    let mut client = Client::new(
        token,
        GatewayIntent::MessageContent | GatewayIntent::GuildMessages,
    );

    client.login(Handler);
}

struct Handler;
impl EventHandler for Handler {
    fn ready(&self, payload: Payload) {
        let username = payload.data["user"]["username"].as_str().unwrap();
        let discriminator = payload.data["user"]["discriminator"].as_str().unwrap();

        println!("Logged in as: {username}#{discriminator}",);
    }

    fn message_create(&self, payload: Payload) {
        let author = payload.data["author"]["global_name"].as_str().unwrap();
        let content = payload.data["content"].as_str().unwrap();

        println!("Message received from `{author}`, message: '{content}'",)
    }

    // I'll add more later, I promise!
}
```
