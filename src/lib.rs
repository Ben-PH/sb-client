use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};

mod pages;

fn init(mut url: Url, orders: &mut impl Orders<Message>) -> Model {
    log!("I N I T I A L I Z E");

    orders.subscribe(Message::UrlChanged).perform_cmd(async {
        match Request::new("/api/auth").method(Method::Get).fetch().await {
            Ok(fetch) => match fetch.check_status() {
                Ok(good_resp) => Message::GoodLogin(good_resp.json().await.unwrap()),
                Err(_) => Message::LoginMsg(pages::login::Message::Unauth),
            },
            Err(e) => Message::NetworkError(e),
        }
    });
    let mut res = Model::default();
    res.base_url = url.to_base_url();
    res.page = Route::init(&mut url);
    res.login = Some(pages::login::Model::default());
    res
}

#[derive(Default, Debug)]
struct Model {
    tab_model: Option<Tab>,
    login: Option<pages::login::Model>,
    user: Option<User>,
    sent: bool,
    good_log: bool,
    base_url: Url,
    page: Route,
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Tab {
    Dashboard(DBModel),
    Spaces(SpModel),
    Bookings(BksModel),
    People(PeopleModel),
}

#[derive(Debug, PartialEq, Default)]
struct DBModel;
#[derive(Debug, PartialEq, Default)]
struct SpModel;
#[derive(Debug, PartialEq, Default)]
struct BksModel;
#[derive(Debug, PartialEq, Default)]
struct PeopleModel;

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
    LoginMsg(pages::login::Message),
    DashboardTab,
    SpacesTab,
    BookingsTab,
    PeopleTab,
    UrlChanged(subs::UrlChanged),
    NetworkError(fetch::FetchError),
    // ChangeEmail(String),
    // ChangePassword(String),
    GoodLogout,
    BadLogout(fetch::FetchError),
    // LoginSent(fetch::Response),
    GoodLogin(User),
    BadLogin(fetch::FetchError),
    // LoginClicked,
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
        LogoutSent(resp) => {
            // set the submitted state login is sent
            match resp.check_status() {
                Ok(_) => {
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
        BadLogin(_) => model.good_log = false,
        GoodLogout => {
            model.good_log = true;
            model.user = None;
            orders.perform_cmd(async { LoginMsg(pages::login::Message::LoggedOut) });
        }
        BadLogout(_) => model.good_log = true,
        NetworkError(_) => {
            model.sent = false;
        }
        LoginMsg(msg) => {
            model.login = Some(pages::login::Model::default());
            pages::login::update(msg, model.login.as_mut().unwrap(), &mut orders.proxy(Message::LoginMsg))
        },
        DashboardTab => {
            model.tab_model = Some(Tab::Dashboard(DBModel));
        },
        PeopleTab => {
            model.tab_model = Some(Tab::People(PeopleModel));
        },
        SpacesTab => {
            model.tab_model = Some(Tab::Spaces(SpModel));
        },
        BookingsTab => {
            model.tab_model = Some(Tab::Bookings(BksModel));
        },
        GoToUrl(mut url) => {
            model.page = Route::init(&mut url);
            url.go_and_push();
        }
        UrlChanged(subs::UrlChanged(mut url)) => {
            model.page = Route::init(&mut url);
        }
    }
}

// ------ ------
//     View
// ------ ------

fn home_header_list<Ms: 'static>(tab: &Tab) -> Vec<Node<Ms>> {
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
                        IF![tab.eq(&Tab::Dashboard(DBModel::default())) => "sb-nav-item-active"]
                    ],
                    ev(Ev::Click, |_| Message::DashboardTab),
                    "Dashboard"
                ],
                li![
                    C![
                        "sb-nav-item",
                        "flex-row",
                        IF![tab.eq(&Tab::Bookings(BksModel::default())) => "sb-nav-item-active"]
                    ],
                    ev(Ev::Click, |_| Message::BookingsTab),
                    "Bookings"
                ],
                li![
                    C![
                        "sb-nav-item",
                        "flex-row",
                        IF![tab.eq(&Tab::Spaces(SpModel::default())) => "sb-nav-item-active"]
                    ],
                    ev(Ev::Click, |_| Message::SpacesTab),
                    "Spaces"
                ],
                li![
                    C![
                        "sb-nav-item",
                        "flex-row",
                        IF![tab.eq(&Tab::People(PeopleModel::default())) => "sb-nav-item-active"]
                    ],
                    ev(Ev::Click, |_| Message::PeopleTab),
                    "People"
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
fn header<Ms: 'static>(tab: &Option<Tab>) -> Node<Ms> {
    let list = match tab {
        Some(active) => home_header_list(&active),
        None => nodes![empty![]],
    };
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
        list
    ]
}


fn view<Ms: 'static>(model: &Model) -> Vec<Node<Ms>> {
    match &model.login {
        Some(login) => pages::login::login_view(&login),
        None => match model.page {
            Route::Home => nodes![
                header(&model.tab_model),
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
            _ => nodes![ol![
                li!["you should handle having Some(login) elsewhere"],
                li![format!("{:?}", model)]
            ]]
        },

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
