use seed::{prelude::*, *};

pub mod bookings;
pub mod dashboard;
pub mod people;
pub mod spaces;

#[derive(Debug)]
pub struct Model {
    dashboard: dashboard::Model,
    bookings: bookings::Model,
    spaces: spaces::Model,
    people: people::Model,
    current: Tab,
    user: shared::User,
}

impl Model {
    pub fn init(user: shared::User) -> Self {
        Self {
            dashboard: dashboard::Model::default(),
            bookings: bookings::Model::default(),
            spaces: spaces::Model,
            people: people::Model::default(),
            current: Tab::default(),
            user,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Tab {
    DashBoard,
    Bookings,
    Spaces,
    People,
}

impl Default for Tab {
    fn default() -> Self {
        Self::DashBoard
    }
}

#[derive(Debug)]
pub enum Message {
    GoodLogout,
    BadLogout(fetch::FetchError),
    LogoutClicked,
    LogoutSent(fetch::Response),
    SwitchTab(Tab),
    BookingsMsg(bookings::Message),
    PeopleMsg(people::Message),
}

pub fn update(msg: Message, model: &mut Model, orders: &mut impl Orders<Message>) {
    log!("tabs update");
    match msg {
        Message::SwitchTab(tab) => model.current = tab,
        Message::BookingsMsg(msg) => bookings::update(
            msg,
            &mut model.bookings,
            &mut orders.proxy(Message::BookingsMsg),
        ),
        Message::PeopleMsg(msg) => people::update(
            msg,
            &mut model.people,
            &mut orders.proxy(Message::PeopleMsg),
        ),
        _ => log!("impl me: ", msg),
    }
}

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
                        IF![tab.eq(&Tab::DashBoard) => "sb-nav-item-active"]
                    ],
                    ev(Ev::Click, |_| Message::SwitchTab(Tab::DashBoard)),
                    "Dashboard"
                ],
                li![
                    C![
                        "sb-nav-item",
                        "flex-row",
                        IF![tab.eq(&Tab::Bookings) => "sb-nav-item-active"]
                    ],
                    ev(Ev::Click, |_| Message::SwitchTab(Tab::Bookings)),
                    "Bookings"
                ],
                li![
                    C![
                        "sb-nav-item",
                        "flex-row",
                        IF![tab.eq(&Tab::Spaces) => "sb-nav-item-active"]
                    ],
                    ev(Ev::Click, |_| Message::SwitchTab(Tab::Spaces)),
                    "Spaces"
                ],
                li![
                    C![
                        "sb-nav-item",
                        "flex-row",
                        IF![tab.eq(&Tab::People) => "sb-nav-item-active"]
                    ],
                    ev(Ev::Click, |_| Message::SwitchTab(Tab::People)),
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
fn header<Ms: 'static>(tab: &Tab) -> Node<Ms> {
    let list = home_header_list(tab);
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
pub fn view(model: &Model) -> Vec<Node<Message>> {
    nodes![
        header(&model.current),
        match &model.current {
            Tab::Bookings => bookings::view(&model.bookings).map_msg(Message::BookingsMsg),
            Tab::People => people::view(&model.people).map_msg(Message::PeopleMsg),
            rest => div![format!("view inserted here for {:?}", rest)]
        }
        br![],br![],br![],
        div![format!("{:?}", model)]

    ]
}
