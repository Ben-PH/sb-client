use seed::{prelude::*, *};
#[derive(Debug)]
pub struct Model {
    main: Main,
    cal: Cal,
    settings: Settings,
    cur: Menu,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            main: Main,
            cal: Cal,
            settings: Settings,
            cur: Menu::Main,
        }
    }
}

#[derive(Debug)]
struct Main;
#[derive(Debug)]
struct Cal;
#[derive(Debug)]
struct Settings;

#[derive(Debug, PartialEq)]
pub enum Menu {
    Main,
    Calendar,
    Settings,
}

#[derive(Debug)]
pub enum Message {
    ChangeMenu(Menu),
}

pub fn update(msg: Message, model: &mut Model, _: &mut impl Orders<Message>) {
    match msg {
        Message::ChangeMenu(tab) => model.cur = tab,
    }
}

pub fn view(model: &Model) -> Node<Message> {
    custom![
        Tag::from("main"),
        C!["sb-view", "sb-view-bookings"],
        attrs! {
            At::from("role") => "main"
        },
        div![
            C!["sb-view-container", "flex-row"],
            nav![
                C!["sb-nav", "sb-nav-side"],
                ul![
                    C!["sb-nav-container"],
                    li![
                        C![
                            "sb-nav-item",
                            "flex-row",
                            IF![model.cur.eq(&Menu::Main) => "sb-nav-item-active"]
                        ],
                        a![
                            attrs! {
                                At::from("name") => "Bookings"
                            },
                            ev(Ev::Click, |_| Message::ChangeMenu(Menu::Main)),
                            "Bookings"
                        ]
                    ],
                    li![
                        C![
                            "sb-nav-item",
                            "flex-row",
                            IF![model.cur.eq(&Menu::Calendar) => "sb-nav-item-active"]
                        ],
                        a![
                            attrs! {
                                At::from("name") => "Calnedar"
                            },
                            ev(Ev::Click, |_| Message::ChangeMenu(Menu::Calendar)),
                            "Calendar"
                        ]
                    ],
                    li![
                        C![
                            "sb-nav-item",
                            "flex-row",
                            IF![model.cur.eq(&Menu::Settings) => "sb-nav-item-active"]
                        ],
                        a![
                            attrs! {
                                At::from("name") => "Settings"
                            },
                            ev(Ev::Click, |_| Message::ChangeMenu(Menu::Settings)),
                            "Settings"
                        ]
                    ]
                ]
            ],
            div![C!["sb-view-content", "flex-column"]]
        ]
    ]
}
