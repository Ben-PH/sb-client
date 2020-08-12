use seed::{prelude::*, *};
#[derive(Debug)]
pub struct Model {
    main: Main,
    settings: Settings,
    cur: Menu,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            main: Main,
            settings: Settings,
            cur: Menu::Main,
        }
    }
}

#[derive(Debug)]
struct Main;
#[derive(Debug)]
struct Settings;

#[derive(Debug, PartialEq)]
pub enum Menu {
    Main,
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
        C!["sb-view", "sb-view-people"],
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
                                At::from("name") => "People"
                            },
                            ev(Ev::Click, |_| Message::ChangeMenu(Menu::Main)),
                            "People"
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
            div![
                C!["sb-view-content", "flex-column"],
                div![
                    C!["sb-view-content", "flex-column"],
                    div![
                        C!["sb-list"],
                        h2![C!["title"], "People"],
                        div![
                            C!["sb-table"],
                            div![
                                C!["flex-row", "sb-table-controls"],
                                div![
                                    C!["button-container"],
                                    button![
                                        C!["button", "button-green", "hollow", "button-add"],
                                        "Add"
                                    ]
                                ],
                                div![
                                    C!["button-container"],
                                    button![
                                        C!["button", "button-red", "hollow", "button-remove"],
                                        "Remove"
                                    ]
                                ],
                                div![
                                    C!["button-container"],
                                    button![
                                        C!["button", "button-primary", "hollow", "button-edit"],
                                        "Edit"
                                    ]
                                ]
                            ],
                            div![
                                C!["sb-table-container"],
                                div![
                                    C!["sb-table-row", "sb-table-row-header"],
                                    div![
                                        C!["sb-table-col"],
                                        div![
                                            C!["input-container"],
                                            input![
                                                id!["row-cb-null"],
                                                C!["input"],
                                                attrs! {At::from("type") => "checkbox"}
                                            ]
                                        ]
                                    ],
                                    div![
                                        C!["sb-table-col"],
                                        div![C!["input-container"], span!["Name"]]
                                    ],
                                    div![
                                        C!["sb-table-col"],
                                        div![C!["input-container"], span!["Email"]]
                                    ],
                                    div![
                                        C!["sb-table-col"],
                                        div![C!["input-container"], span!["Role"]]
                                    ]
                                ],
                                // TODO
                                div![
                                    C!["sb-table-body"],
                                    div![
                                        C!["sb-table-row"],
                                        div![
                                            C!["sb-table-col"],
                                            input![
                                                id!["row-cb-null"],
                                                C!["input"],
                                                attrs! {At::from("type") => "checkbox"}
                                            ]
                                        ],
                                        div![C!["sb-table-col"], span!["Hard-coded"]],
                                        div![C!["sb-table-col"], span!["fetech"]],
                                        div![C!["sb-table-col"], span!["data"]]
                                    ]
                                ]
                            ]
                        ],
                        div![
                            C!["sb-modal-container", "flex-center"],
                            div![
                                C!["sb-modal-content"],
                                form![
                                    id!["user"],
                                    C!["sb-form"],
                                    div![C!["sb-form-title"], h2![C!["title"], "Add Person"]],
                                    div![
                                        C!["sb-form-content"],
                                        fieldset![
                                            C!["sb-form-controls", "flex-row"],
                                            div![
                                                C!["flex-row"],
                                                div![
                                                    C!["input-container"],
                                                    label![
                                                        C!["alt-text", "input-label"],
                                                        "First Name"
                                                    ],
                                                    input![
                                                        id!["first_name"],
                                                        C!["input"],
                                                        attrs! {At::from("type") => "text", At::Required => true}
                                                    ]
                                                ],
                                                div![
                                                    C!["input-container"],
                                                    label![
                                                        C!["alt-text", "input-label"],
                                                        "Last Name"
                                                    ],
                                                    input![
                                                        id!["last_name"],
                                                        C!["input"],
                                                        attrs! {At::from("type") => "text", At::Required => true}
                                                    ]
                                                ]
                                            ]
                                        ],
                                        fieldset![
                                            C!["sb-form-controls", "flex-row"],
                                            div![
                                                C!["flex-row"],
                                                div![
                                                    C!["input-container"],
                                                    label![C!["alt-text", "input-label"], "Role:"],
                                                    select![
                                                        C!["input"],
                                                        id!["role"],
                                                        attrs! {At::from("type") => "select", At::Required => true, At::from("placeholder") => "Select User Role..."}
                                                    ]
                                                ]
                                            ]
                                        ],
                                        fieldset![
                                            C!["sb-form-controls", "flex-row"],
                                            div![C!["flex-row"], div![C!["input-container"]]]
                                        ],
                                    ],
                                    fieldset![C!["sb-form-controls", "flex-row"],]
                                ]
                            ],
                            div![C!["sb-slideout-resize"]]
                        ]
                    ]
                ]
            ]
        ]
    ]
}
