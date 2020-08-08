use either::Either;
use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};

fn init(_url: Url, orders: &mut impl Orders<Message>) -> Model {
    log!("I N I T I A L I Z E");
    orders
        .subscribe(Message::AppPathChange)
        .after_next_render(|_| Message::CheckProfile);

    // .perform_cmd(async {
    //     let req = Request::new("/api/auth")
    //         .method(Method::Get)
    //         .fetch()
    //         .await;
    //     match req {
    //         Ok(resp) => match resp.check_status() {
    //             Ok(r) => Message::LoggedIn,
    //             _ => Message::NotLoggedIn,
    //         }
    //         _ => Message::NotLoggedIn,
    //     }
    // });
    //     .notify(subs::UrlChanged(Urls::new(url).login()));
    Model::default()
}

#[derive(Debug)]
struct Model {
    user_ctx: Either<Login, User>,
    page_id: Option<Page>,
}
impl Default for Model {
    fn default() -> Self {
        Self {
            user_ctx: Either::Left(Login::default()),
            page_id: Some(Page::Home),
        }
    }
}

impl Model {
    fn get_login(&self) -> Option<&Login> {
        self.user_ctx.as_ref().left()
    }
    fn get_user(&self) -> Option<&User> {
        self.user_ctx.as_ref().right()
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct Login {
    email: String,
    password: String,
}
#[derive(Default, Debug, Serialize, Deserialize)]
struct User {
    first_name: String,
    last_name: String,
    email: String,
}
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
    AppPathChange(subs::UrlChanged),
    Login,
    LoggedIn(User),
    Logout,
    LoggedOut,
    NotLoggedIn,
    CheckProfile,
    Nothing,
}

// ------ ------
//    Update
// ------ ------

fn update(msg: Message, model: &mut Model, orders: &mut impl Orders<Message>) {
    log("updating");
    match msg {
        Message::Login => {
            let request = Request::new("/api/auth/login")
                .method(Method::Post)
                .json(&Login {
                    email: "f@bar.com".to_string(),
                    password: "hunter2".to_string(),
                })
                .expect("Serialization failed");

            orders.perform_cmd(async {
                let resp = fetch(request).await.unwrap().check_status();
                match resp {
                    Ok(o) => match o.json::<User>().await {
                        Ok(usr) => Message::LoggedIn(usr),
                        Err(e) => {
                            log!("bad deserialization", e);
                            Message::NotLoggedIn
                        }
                    },
                    Err(e) => {
                        log!("bad response", e);
                        Message::NotLoggedIn
                    }
                }
            });
        }
        Message::CheckProfile => {
            let request = Request::new("/api/auth").method(Method::Get);

            orders.perform_cmd(async {
                let resp = fetch(request).await.unwrap().check_status();
                match resp {
                    Ok(o) => match o.json::<User>().await {
                        Ok(usr) => Message::LoggedIn(usr),
                        Err(e) => {
                            log!("bad deserialization", e);
                            Message::NotLoggedIn
                        }
                    },
                    Err(e) => {
                        log!("bad response", e);
                        Message::NotLoggedIn
                    }
                }
            });
        }
        Message::Logout => {
            let request = Request::new("/api/auth").method(Method::Delete);

            orders.perform_cmd(async {
                let resp = fetch(request).await.unwrap().check_status();
                match resp {
                    Ok(_) => Message::LoggedOut,
                    _ => Message::Nothing,
                }
            });
        }
        Message::LoggedOut => model.user_ctx = Either::Left(Login::default()),
        Message::LoggedIn(usr) => if let Either::Left(_) = &model.user_ctx {
            model.user_ctx = Either::Right(usr);
        },
        _ => log!("impl me", msg),
    }
}

// ------ ------
//     View
// ------ ------

fn guest_view(_model: &Model) -> Node<Message> {
    button!["login", ev(Ev::Click, |_| Message::Login)]
}

fn logged_view(_model: &Model) -> Node<Message> {
    div!["logged in users page goes here"]
}

fn view(model: &Model) -> impl IntoNodes<Message> {
    nodes![
        match model.page_id {
            Some(Page::Login) => {
                if model.get_login().is_some() {
                    guest_view(model)
                } else {
                    logged_view(model)
                }
            }
            _ => p! {"implement me"},
        },
        match &model.user_ctx {
            Either::Left(_) => button!["login", ev(Ev::Click, |_| Message::Login)],
            Either::Right(logged) => {
                div![
                    div![format!("hello, {:?}", logged)],
                    button!["logout", ev(Ev::Click, |_| Message::Logout)]
                ]
            }
        }
    ]

    // match model.page {
    //     Page::Home => nodes![div![
    //         button!["go to `/login`?", attrs! {At::Href => Urls::new("").login()}],
    //     ]],
    //     _ => nodes![p! {"TODO"}],
    // }
}

struct_urls!();
impl<'a> Urls<'a> {
    pub fn home(self) -> Url {
        self.base_url()
    }
    pub fn login(self) -> Url {
        self.base_url().add_path_part("login")
    }
    pub fn not_found(self) -> Url {
        self.base_url().add_path_part("404")
    }
}
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
