use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};

use shared;
// ------ ------
//     Init
// ------ ------

pub fn init() -> Model {
    Model::default()
}

// ------ ------
//     Model
// ------ ------

#[derive(Debug, Default)]
pub struct Model {
    sent: bool,
    good_log: bool,
    form: shared::Login,
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Form {
    email: String,
    password: String,
}

// ------ ------
//     Update
// ------ ------

#[derive(Debug)]
pub enum Message {
    Unauth,
    LoggedOut,
    ChangeEmail(String),
    ChangePassword(String),
    LoginSent(fetch::Response),
    GoodLogin(shared::User),
    ParsedResp(shared::User),
    BadLogin(fetch::FetchError),
    LoginClicked,
    NetworkError(fetch::FetchError),
}

pub fn update(
    msg: Message,
    model: &mut Model,
    orders: &mut impl Orders<Message>,
) -> Option<crate::Message> {
    log!("login page update");
    use Message::*;
    match msg {
        Unauth => {
            log!("unauth");
            *model = Model::default();
        }
        ChangeEmail(new) => model.form.email = new,
        ChangePassword(new) => {
            model.form.password = new;
        }
        LoginClicked => {
            orders.skip();
            let resp = Request::new("api/auth/login")
                .method(Method::Post)
                .json(&model.form)
                .expect("bad serialization")
                .fetch();
            model.form.password = "".to_string();
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
                    orders.perform_cmd(async move {
                        GoodLogin(good_resp.json::<shared::User>().await.unwrap())
                    });
                }
                Err(e) => {
                    orders.perform_cmd(async { BadLogin(e) });
                }
            }
        }
        BadLogin(e) => {
            log!(e);
            model.good_log = false;
        }
        GoodLogin(usr) => return Some(crate::Message::GoodLogin(usr)),
        _ => log!("impl me: ", msg),
    }
    None
}
// ------ ------
//     View
// ------ ------
//
fn login_header<Ms>() -> Node<Ms> {
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
    ]
}

pub fn login_view<Ms: 'static>(model: &Model) -> Vec<Node<Ms>> {
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
            login_header(),
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
                                        At::Value => model.form.email
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
                                        At::Value => model.form.password
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
