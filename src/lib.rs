use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};

fn init(url: Url, orders: &mut impl Orders<Message>) -> Model {
    log!("I N I T I A L I Z E");
    // orders.subscribe(Message::UrlChanged);
    Model
}

struct Model;
#[derive(Debug)]
enum Page {
    Home,
    Login,
    NotFound,
}


impl Default for Page {
    fn default() -> Self {
        Self::Login
    }
}
#[derive(Debug)]
enum Message {
    Nothing,
}

// ------ ------
//    Update
// ------ ------

fn update(msg: Message, model: &mut Model, orders: &mut impl Orders<Message>) {
    match msg {
        _ => {}
        // Message::LoginButton => {
        //     let request = Request::new("/api/auth/login")
        //         .method(Method::Post)
        //         .json(&model.login)
        //         .expect("Serialization failed");

        //     orders.perform_cmd(async {
        //         let resp = fetch(request).await.unwrap().check_status();
        //         match resp {
        //             Ok(_) => Message::GoodLogin,
        //             Err(e) => Message::Fetched(Err(e)),
        //         }
        //     });
        // }
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> impl IntoNodes<Message> {
    div![]
    // match model.page {
    //     Page::Home => nodes![div![
    //         button!["go to `/login`?", attrs! {At::Href => Urls::new("").login()}],
    //     ]],
    //     _ => nodes![p! {"TODO"}],
    // }
}

struct_urls!();
impl<'a> Urls<'a> {
    pub fn home(self) -> Url {self.base_url()}
    pub fn login(self) -> Url {self.base_url().add_path_part("login")}
}
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
