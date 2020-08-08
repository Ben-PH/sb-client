use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};

fn init(_url: Url, orders: &mut impl Orders<Message>) -> Model {
    log!("I N I T I A L I Z E");
    orders
        .subscribe(Message::AppPathChange)
        .send_msg(Message::CheckProfile);

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

#[derive(Default, Debug)]
struct Model {
    first_name: String,
    last_name: String,
    email: String,
    password: String,
    page_id: Option<Page>,
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
    ToPage(Page),
    AppPathChange(subs::UrlChanged),
    Login,
    LoggedIn(User),
    ChangeEmail(String),
    ChangePassword(String),
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
        Message::ChangeEmail(new) => model.email = new,
        Message::ChangePassword(new) => model.password = new,
        Message::Login => {
            let request = Request::new("/api/auth/login")
                .method(Method::Post)
                .json(&Login {
                    email: model.email.to_string(),
                    password: model.password.to_string(),
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
        Message::LoggedOut => {
            *model = Model::default();
            orders.perform_cmd(async { Message::ToPage(Page::Login) });
        }
        Message::LoggedIn(log) => {
            model.password = "".to_string();
            model.first_name = log.first_name;
            model.last_name = log.last_name;
            model.email = log.email;
            orders.perform_cmd(async { Message::ToPage(Page::Home) });
        }
        Message::ToPage(pg) => model.page_id = Some(pg),
        Message::NotLoggedIn => {
            orders.perform_cmd(async { Message::ToPage(Page::Login) });
        }
        _ => log!("impl me", msg),
    }
}

// ------ ------
//     View
// ------ ------

fn home_view(model: &Model) -> Vec<Node<Message>> {
    nodes![
        div![format!("welcome home, {:#?}", model)],
        button!["logout", ev(Ev::Click, |_| Message::Logout)]
    ]
}

fn login_view(model: &Model) -> Vec<Node<Message>> {
    nodes![div![
        id!["root"],
        C!["sb-login"],
        div![
            C!["sb-login-container"],
            header![
                C!["banner"],
                a![
                    id!["logo"],
                    C!["flex-center"],
                    img![attrs! {
                        At::Src => "/img/logo-2.png",
                        At::Width => "40", At::Height => "40"
                    }],
                    h1!["Spacebook"]
                ]
            ],
            custom![
                Tag::from("main"),
                id!["main"],
                C!["sb-login-content"],
                attrs! {
                    At::from("role") => "main"
                },
                form![
                    id!["sb-login-form"],
                    fieldset![
                        legend!["Sign In to Continue:"],
                        div![
                            C!["flex-row"],
                            div![
                                C!["input-container"],
                                input![
                                    id!["sb-login-email"],
                                    C!["input"],
                                    attrs! {
                                        At::Type => "email",
                                        At::Required => true,
                                        At::Placeholder => "Email",
                                        At::Value => model.email
                                    },
                                    input_ev(Ev::Input, Message::ChangeEmail)
                                ]
                            ]
                        ],
                        div![
                            C!["flex-row"],
                            div![
                                C!["input-container"],
                                input![
                                    id!["sb-login-password"],
                                    C!["input"],
                                    attrs! {
                                        At::Type => "password",
                                        At::Required => true,
                                        At::Placeholder => "Password",
                                        At::Value => model.password
                                    },
                                    input_ev(Ev::Input, Message::ChangePassword)
                                ]
                            ]
                        ],
                        div![
                            C!["flex-row"],
                            div![
                                C!["login-unauth", "input-container"],
                                div![
                                    C!["button", "button-secondary"],
                                    "Sign In",
                                    ev(Ev::Click, |_| Message::Login)
                                ]
                            ]
                        ],
                        div![C!["flex-row"], format!("hello: {:?}", model)],
                    ]
                ]
            ]
        ]
    ]]
}

fn view(model: &Model) -> impl IntoNodes<Message> {
    match &model.page_id {
        Some(Page::Login) => login_view(model),
        Some(Page::Home) => home_view(model),
        _ => nodes![div!["unimplimented"]],
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
    pub fn not_found(self) -> Url {
        self.base_url().add_path_part("404")
    }
}
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
