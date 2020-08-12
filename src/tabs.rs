use seed::{prelude::*, *};

pub mod dashboard;
// pub mod people;
// pub mod bookings;
// pub mod spaces;

#[derive(Debug)]
pub struct Model {
    dashboard: dashboard::Model,
    current: Tab,
    user: shared::User,
}

impl Model {
    pub fn init(user: shared::User) -> Self {
        Self {
            dashboard: dashboard::Model::default(),
            current: Tab::default(),
            user,
        }
    }
}

#[derive(Debug)]
pub enum Tab {
    DashBoard,
}

impl Default for Tab {
    fn default() -> Self {
        Self::DashBoard
    }
}

#[derive(Debug)]
pub enum Message {
    GoodLogin(shared::User),
    GoodLogout,
    BadLogout(fetch::FetchError),
    LogoutClicked,
    LogoutSent(fetch::Response),
}

pub fn update(msg: Message, _model: &mut Model, _orders: &mut impl Orders<Message>) {
    log!("tabs update");
    match msg {
        _ => log!("impl me: ", msg),
    }
}

// fn home_header_list<Ms: 'static>(tab: &Tab) -> Vec<Node<Ms>> {
//     vec![
//         nav![
//             id!["sb-nav-top"],
//             C!["sb-nav", "sb-nav-top"],
//             ul![
//                 C!["sb-nav-container"],
//                 li![
//                     C![
//                         "sb-nav-item",
//                         "flex-row",
//                         IF![tab.eq(&Tab::Dashboard(DBModel::default())) => "sb-nav-item-active"]
//                     ],
//                     ev(Ev::Click, |_| Message::DashboardTab),
//                     "Dashboard"
//                 ],
//                 li![
//                     C![
//                         "sb-nav-item",
//                         "flex-row",
//                         IF![tab.eq(&Tab::Bookings(BksModel::default())) => "sb-nav-item-active"]
//                     ],
//                     ev(Ev::Click, |_| Message::BookingsTab),
//                     "Bookings"
//                 ],
//                 li![
//                     C![
//                         "sb-nav-item",
//                         "flex-row",
//                         IF![tab.eq(&Tab::Spaces(SpModel::default())) => "sb-nav-item-active"]
//                     ],
//                     ev(Ev::Click, |_| Message::SpacesTab),
//                     "Spaces"
//                 ],
//                 li![
//                     C![
//                         "sb-nav-item",
//                         "flex-row",
//                         IF![tab.eq(&Tab::People(PeopleModel::default())) => "sb-nav-item-active"]
//                     ],
//                     ev(Ev::Click, |_| Message::PeopleTab),
//                     "People"
//                 ]
//             ]
//         ],
//         div![
//             id!["sb-profile"],
//             C!["sb-nav", "sb-nav-top", "sb-profile"],
//             div![C!["sb-nav-container"]],
//             h2![C!["sb-nav-item"]],
//             div![
//                 C!["sb-nav-item", "sb-profile-button", "button"],
//                 // navigate to profile
//             ],
//             div![
//                 C!["sb-nav-item", "sb-logout-button", "button"],
//                 // logout
//             ]
//         ],
//     ]
// }
// fn header<Ms: 'static>(tab: &Option<Tab>) -> Node<Ms> {
//     let list = match tab {
//         Some(active) => home_header_list(&active),
//         None => nodes![empty![]],
//     };
//     header![
//         C!["banner"],
//         a![
//             id!["logo"],
//             C!["flex-center"],
//             // img![attrs! {
//             //     At::Src => "/img/logo-2.png",
//             //     At::Width => "40", At::Height => "40"
//             // }],
//             h1!["Spacebook"]
//         ],
//         list
//     ]
// }
fn view(model: &Model) -> Node<Message> {
    ol![li!["dashboard view"], li![format!("{:?}", model)],]
}
