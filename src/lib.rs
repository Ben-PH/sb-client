use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};

mod api_calls;

fn init(mut url: Url, orders: &mut impl Orders<Message>) -> Model {
    log!("I N I T I A L I Z E");

    orders.subscribe(Message::UrlChanged).perform_cmd(async {
        match Request::new("/api/auth").method(Method::Get).fetch().await {
            Ok(fetch) => match fetch.check_status() {
                Ok(good_resp) => {
                    log!("foobar", good_resp.raw_response());
                    Message::GoodLogin(good_resp.json().await.unwrap())
                }
                Err(e) => Message::ToLoginPage,
            },
            Err(e) => Message::NetworkError(e),
        }
    });
    let mut res = Model::default();
    let next = url.next_path_part();
    res.page = Route::init(next);
    res.base_url = url.to_base_url();
    res
}

#[derive(Default, Debug)]
struct Model {
    login: Login,
    user: Option<User>,
    sent: bool,
    good_log: bool,
    base_url: Url,
    page: Option<Route>,
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
}

impl Route {
    fn init(next_path_part: Option<&str>) -> Option<Self> {
        match next_path_part {
            None => Some(Self::Home),
            Some("login") => Some(Self::Login),
            Some(_) => None,
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
    LoginClicked,
}

// ------ ------
//    Update
// ------ ------

fn update(msg: Message, model: &mut Model, orders: &mut impl Orders<Message>) {
    log("updating");
    use Message::*;
    match msg {
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
                    Ok(fired) => {
                        log!(fired.raw_response());
                        LoginSent(fired)
                    }
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
                    log!(good_resp.raw_response());
                    orders.perform_cmd(async move { GoodLogin(good_resp.json().await.unwrap()) });
                }
                Err(e) => {
                    orders.perform_cmd(async { BadLogin(e) });
                }
            }
        }
        GoodLogin(usr) => {
            model.good_log = true;
            model.user = Some(usr);
        }
        BadLogin(er) => model.good_log = false,
        NetworkError(err) => {
            log!(err);
            model.sent = false;
        }
        ChangeEmail(new) => model.login.email = new,
        ChangePassword(new) => {
            model.login.password = new;
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
    login_view(model)
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
