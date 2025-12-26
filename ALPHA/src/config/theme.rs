// use proc_macro::TokenStream;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct Theme {
    pub label: String,
    pub value: String,
    pub private: bool,
    pub authorized_projects: Vec<String>,
}

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct ThemeConfigDefault {
    pub favicon: Option<String>,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
    pub background_color: Option<String>,
    pub text_color: Option<String>,
}

// /// Marks async main function as the Actix Web system entry-point.
// ///
// /// Note that Actix Web also works under `#[tokio::main]` since version 4.0. However, this macro is
// /// still necessary for actor support (since actors use a `System`). Read more in the
// /// [`actix_web::rt`](https://docs.rs/actix-web/4/actix_web/rt) module docs.
// ///
// /// # Examples
// /// ```
// /// #[actix_web::main]
// /// async fn main() {
// ///     async { println!("Hello world"); }.await
// /// }
// /// ```
// #[proc_macro_attribute]
// pub fn default_config(_item: TokenStream) -> TokenStream {
//     "pub favicon: Option<String>,
//     pub primary_color: Option<String>,
//     pub secondary_color: Option<String>,
//     pub background_color: Option<String>,
//     pub text_color: Option<String>,"
// }
