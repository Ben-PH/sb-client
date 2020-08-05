use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};

fn init(url: Url, orders: &mut impl Orders<Message>) -> Model {
    log!("I N I T I A L I Z E");
    orders.subscribe(Message::UrlChanged);
    Model {
        login: Login::default(),
        response_data: None,
        base_url: url.to_base_url(),
        page: Page::init(url),
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Login {
    email: String,
    password: String,
}

// ------ ------
//     Model
// ------ ------

#[derive(Debug, Default)]
struct Model {
    login: Login,
    pub response_data: Option<String>,
    page: Page,
    base_url: Url,
}

#[derive(Debug)]
enum Page {
    Home,
    Login,
    NotFound,
}

impl Page {
    fn init(mut url: Url) -> Self {
        match url.next_path_part() {
            None => Self::Home,
            Some("login") => Self::Login,
            _ => Self::NotFound,
        }
    }
}

impl Default for Page {
    fn default() -> Self {
        Self::Login
    }
}
#[derive(Debug)]
enum Message {
    EmailChanged(String),
    PasswordChanged(String),
    LoginButton,
    GoodLogin,
    LogoutButton,
    GoodLogout,
    CreateButton,
    Fetched(fetch::Result<String>),
    UrlChanged(subs::UrlChanged),
}

// ------ ------
//    Update
// ------ ------

fn update(msg: Message, model: &mut Model, orders: &mut impl Orders<Message>) {
    match msg {
        Message::EmailChanged(email) => model.login.email = email,
        Message::PasswordChanged(password) => model.login.password = password,
        Message::LoginButton => {
            let request = Request::new("/api/auth/login")
                .method(Method::Post)
                .json(&model.login)
                .expect("Serialization failed");

            orders.perform_cmd(async {
                let resp = fetch(request).await.unwrap().check_status();
                match resp {
                    Ok(_) => Message::GoodLogin,
                    Err(e) => Message::Fetched(Err(e)),
                }
            });
        }
        Message::Fetched(res) => match res {
            Ok(res) => model.response_data = Some(res),
            Err(e) => {
                model.response_data = None;
                log!(e)
            }
        },
        Message::LogoutButton => {
            let request = Request::new("/api/auth").method(Method::Delete);

            orders.perform_cmd(async {
                let resp = fetch(request).await.unwrap().check_status();
                match resp {
                    Ok(_) => Message::GoodLogout,
                    Err(e) => Message::Fetched(Err(e)),
                }
            });
        }
        Message::UrlChanged(subs::UrlChanged(url)) => {
            model.page = Page::init(url);
        }
        // Message::CreateButton => {
        Message::GoodLogin => {
            model.login = Login::default();
            orders.notify(subs::UrlRequested::new(Urls::new(&model.base_url).home()));
        }
        Message::GoodLogout => {
            model.login = Login::default();
            orders.notify(subs::UrlRequested::new(Urls::new(&model.base_url).login()));
        }
        _ => log!("TODO: impl handling for ", msg),
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> impl IntoNodes<Message> {
    match model.page {
        Page::Login => nodes![
            input![input_ev(Ev::Input, Message::EmailChanged),],
            input![input_ev(Ev::Input, Message::PasswordChanged),],
            button![ev(Ev::Click, |_| Message::LoginButton), "login"],
            button![ev(Ev::Click, |_| Message::CreateButton), "create"],
            dialog!["this is a dialog"]
        ],
        Page::Home => nodes![div![
            div!["Welcome home!"],
            button![
                "go to `/login`?",
                attrs! {
                    At::Href => Urls::new(&model.base_url).login()
                }
            ],
            button![ev(Ev::Click, |_| Message::LogoutButton), "logout"],
        ]],
        _ => nodes![p! {format!("implement this page: {:#?}", model.page)}],
    }
}

struct_urls!();
impl<'a> Urls<'a> {
    pub fn home(self) -> Url {
        self.base_url()
    }
    pub fn login(self) -> Url {
        self.base_url().add_path_part("login")
    }
}
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
