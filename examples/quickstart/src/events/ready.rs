#[descord::event]
pub async fn ready(ready: ReadyData) {
    println!(
        "Logged in as: {}#{}",
        ready.user.username, ready.user.discriminator
    );
}

// alternatively, you can also do this if

// #[descord::event(ready)]
// pub async fn on_ready(ready: ReadyData) { ... }
