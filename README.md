# SpaceTraders

This is a rust API wrapper for https://spacetraders.io

View documentation here https://docs.rs/spacetraders/

When running without optimizations (I.E. without --release) the library will not error if there are extra fields in the
JSON responses. In debug mode if the JSON contains extra fields an error will be emitted. This is done intentionally to
hopefully provide the most correct client possible. If you come across one of these errors please submit a PR to add the
additional fields! Thanks!

## Example

You can view my spacetraders bot here https://github.com/bloveless/spacemonger-api which has a full example of how to
use this crate.

A brief example is included below.

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let http_client = client::get_http_client();
    let client = Client::new(
        http_client,
        "your-username".to_string(),
        "your-token".to_string(),
    );

    let mut current_user_info = client.get_user_info().await?;
    println!("Current user info {:?}", current_user_info);

    // Continue calling other functions and build cool things!
}
```
