use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};

mod api_calls;

fn init(mut url: Url, orders: &mut impl Orders<Message>) -> Model {
    log!("I N I T I A L I Z E");

    orders.subscribe(Message::UrlChanged).perform_cmd(async {
        match Request::new("/api/auth").method(Method::Get).fetch().await {
            Ok(fetch) => match fetch.check_status() {
                Ok(good_resp) => Message::GoodLogin(good_resp.json().await.unwrap()),
                Err(e) => Message::ToLoginPage,
            },
            Err(e) => Message::NetworkError(e),
        }
    });
    let mut res = Model::default();
    res.base_url = url.to_base_url();
    res.page = Route::init(&mut url);
    res
}

#[derive(Default, Debug)]
struct Model {
    login: Login,
    user: Option<User>,
    sent: bool,
    good_log: bool,
    base_url: Url,
    page: Route,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct User {
    first_name: String,
    last_name: String,
    email: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Login {
    email: String,
    password: String,
}

#[derive(Debug)]
enum Route {
    Home,
    Login,
    NotFound,
}

impl Route {
    fn init(url: &mut Url) -> Self {
        match url.next_path_part() {
            None => Self::Home,
            Some("login") => Self::Login,
            Some(_) => Self::NotFound,
        }
    }

    fn is_active(&self, path: String) -> bool {
        match &self {
            Route::Home => path.eq("home"),
            Route::Login => path.eq("login"),
            Route::NotFound => path.eq("404"),
        }
    }
}
struct_urls!();
/// Construct url injected in the web browser with path
impl<'a> Urls<'a> {
    pub fn build_url(self, path: &str) -> Url {
        if path.eq("Home") {
            self.base_url()
        } else {
            self.base_url().add_path_part(path)
        }
    }
}

impl Default for Route {
    fn default() -> Self {
        Self::Home
    }
}

#[derive(Debug)]
enum Message {
    UrlChanged(subs::UrlChanged),
    ToLoginPage,
    NetworkError(fetch::FetchError),
    LoginSent(fetch::Response),
    AuthCheck(fetch::Result<User>),
    ChangeEmail(String),
    ChangePassword(String),
    GoodLogin(User),
    BadLogin(fetch::FetchError),
    GoodLogout,
    BadLogout(fetch::FetchError),
    LoginClicked,
    LogoutSent(fetch::Response),
    LogoutClicked,
    GoToUrl(Url),
}

// ------ ------
//    Update
// ------ ------

fn update(msg: Message, model: &mut Model, orders: &mut impl Orders<Message>) {
    log("updating");
    use Message::*;
    match msg {
        LogoutClicked => {
            orders.skip();
            let resp = Request::new("api/auth").method(Method::Delete).fetch();
            orders.perform_cmd(async {
                match resp.await {
                    Ok(fired) => LogoutSent(fired),
                    Err(e) => NetworkError(e),
                }
            });
        }
        LoginClicked => {
            orders.skip();
            let resp = Request::new("api/auth/login")
                .method(Method::Post)
                .json(&model.login)
                .expect("bad serialization")
                .fetch();
            model.login.password = "".to_string();
            orders.perform_cmd(async {
                match resp.await {
                    Ok(fired) => LoginSent(fired),
                    Err(e) => NetworkError(e),
                }
            });
        }
        LoginSent(resp) => {
            // set the submitted state login is sent
            if model.sent {
                orders.skip();
            }
            model.sent = true;
            match resp.check_status() {
                Ok(good_resp) => {
                    orders.perform_cmd(async move { GoodLogin(good_resp.json().await.unwrap()) });
                }
                Err(e) => {
                    orders.perform_cmd(async { BadLogin(e) });
                }
            }
        }
        LogoutSent(resp) => {
            // set the submitted state login is sent
            match resp.check_status() {
                Ok(good_resp) => {
                    orders.perform_cmd(async move { GoodLogout });
                }
                Err(e) => {
                    orders.perform_cmd(async { BadLogout(e) });
                }
            }
        }
        GoodLogin(usr) => {
            model.good_log = true;
            model.user = Some(usr);
            orders.perform_cmd(async { GoToUrl(Url::new()) });
        }
        BadLogin(er) => model.good_log = false,
        GoodLogout => {
            model.good_log = true;
            model.user = None;
            orders.perform_cmd(async { ToLoginPage });
        }
        BadLogout(er) => model.good_log = true,
        NetworkError(err) => {
            model.sent = false;
        }
        ChangeEmail(new) => model.login.email = new,
        ChangePassword(new) => {
            model.login.password = new;
        }
        ToLoginPage => {
            let mut url = Url::new().add_path_part("login");
            orders.perform_cmd(async { GoToUrl(url) });
        }
        GoToUrl(mut url) => {
            model.page = Route::init(&mut url);
            url.go_and_push();
        }
        UrlChanged(subs::UrlChanged(mut url)) => {
            model.page = Route::init(&mut url);
        }
        _ => log!("impl me: ", msg),
    }
}

// ------ ------
//     View
// ------ ------

fn home_view(model: &Model) -> Vec<Node<Message>> {
    nodes![
        div![format!("welcome home, {:#?}", model)],
        // button!["logout", ev(Ev::Click, |_| Message::Logout)]
    ]
}

fn home_header_list(route: &Route) -> Vec<Node<Message>> {
    vec![
        nav![
            id!["sb-nav-top"],
            C!["sb-nav", "sb-nav-top"],
            ul![
                C!["sb-nav-container"],
                li![
                    C![
                        "sb-nav-item",
                        "flex-row",
                        IF![route.is_active("home".to_string()) => "sb-nav-item-active"]
                    ],
                    a![
                        attrs! {
                            At::Href => "/",
                            At::Name => "Dashboard"
                        },
                        "Dashboard"
                    ]
                ],
                li![
                    C![
                        "sb-nav-item",
                        "flex-row",
                        IF![route.is_active("bookings".to_string()) => "sb-nav-item-active"]
                    ],
                    a![
                        attrs! {
                            At::Href => "/bookings",
                            At::Name => "Bookings"
                        },
                        "Bookings"
                    ]
                ],
                li![
                    C![
                        "sb-nav-item",
                        "flex-row",
                        IF![route.is_active("spaces".to_string()) => "sb-nav-item-active"]
                    ],
                    a![
                        attrs! {
                            At::Href => "/spaces",
                            At::Name => "Spaces"
                        },
                        "Spaces"
                    ]
                ],
                li![
                    C![
                        "sb-nav-item",
                        "flex-row",
                        IF![route.is_active("people".to_string()) => "sb-nav-item-active"]
                    ],
                    a![
                        attrs! {
                            At::Href => "/people",
                            At::Name => "People"
                        },
                        "People"
                    ]
                ]
            ]
        ],
        div![
            id!["sb-profile"],
            C!["sb-nav", "sb-nav-top", "sb-profile"],
            div![C!["sb-nav-container"]],
            h2![C!["sb-nav-item"]],
            div![
                C!["sb-nav-item", "sb-profile-button", "button"],
                // navigate to profile
            ],
            div![
                C!["sb-nav-item", "sb-logout-button", "button"],
                // logout
            ]
        ],
    ]
}
fn header(list: Vec<Node<Message>>, route: &Route) -> Node<Message> {
    header![
        C!["banner"],
        a![
            id!["logo"],
            C!["flex-center"],
            // img![attrs! {
            //     At::Src => "/img/logo-2.png",
            //     At::Width => "40", At::Height => "40"
            // }],
            h1!["Spacebook"]
        ],
        home_header_list(route)
    ]
}

fn login_view(model: &Model) -> Vec<Node<Message>> {
    let submitted = match model.sent {
        true => "submitted",
        false => "",
    };
    let bad_login = match model.good_log {
        false => "invalid",
        true => "",
    };
    nodes![div![
        id!["root"],
        C!["sb-login"],
        div![
            C!["sb-login-container"],
            header(vec![], &model.page),
            custom![
                Tag::from("main"),
                id!["main"],
                C!["sb-login-content"],
                attrs! {
                    At::from("role") => "main"
                },
                form![
                    id!["sb-login-form"],
                    C![submitted, bad_login],
                    fieldset![
                        legend!["Sign In to Continue:"],
                        div![
                            C!["flex-row"],
                            div![
                                C!["input-container", bad_login],
                                input![
                                    id!["sb-login-email"],
                                    C!["input"],
                                    attrs! {
                                        At::Type => "email",
                                        At::Required => true,
                                        At::Placeholder => "Email",
                                        At::Value => model.login.email
                                    },
                                    input_ev(Ev::Input, Message::ChangeEmail)
                                ]
                            ]
                        ],
                        div![
                            C!["flex-row"],
                            div![
                                C!["input-container", bad_login],
                                input![
                                    id!["sb-login-password"],
                                    C!["input"],
                                    attrs! {
                                        At::Type => "password",
                                        At::Required => true,
                                        At::Placeholder => "Password",
                                        At::Value => model.login.password
                                    },
                                    input_ev(Ev::Input, Message::ChangePassword)
                                ]
                            ]
                        ],
                        div![
                            C!["flex-row"],
                            div![
                                C!["login-unauth", "input-container", bad_login],
                                IF![!model.good_log && model.sent => div![C!["input-validation"], "Incorrect Email Adress or Password"]],
                                div![
                                    C!["button", "button-secondary"],
                                    "Sign In",
                                    ev(Ev::Click, |_| Message::LoginClicked)
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
    match model.page {
        Route::Login => login_view(model),
        Route::Home => nodes![
            header(home_header_list(&model.page), &model.page),
            div![ol![
                li!["welcome home"],
                li![format!("{:#?}", model)],
                li![button![
                    "sign-out",
                    ev(Ev::Click, |_| Message::LogoutClicked)
                ]]
            ]]
        ],
        Route::NotFound => nodes![div![ol![
            li!["welcome to 404"],
            li![format!("{:#?}", model)]
        ]]],
    }
    // match &model.page_id {
    //     Some(Page::Login) => login_view(model),
    //     Some(Page::Home) => home_view(model),
    //     _ => nodes![div!["unimplimented"]],
    // }
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
