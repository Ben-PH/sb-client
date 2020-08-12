use seed::{prelude::*, *};

mod pages;
mod tabs;

fn init(mut _url: Url, orders: &mut impl Orders<Message>) -> Model {
    log!("I N I T I A L I Z E");

    orders.perform_cmd(async {
        match Request::new("/api/auth").method(Method::Get).fetch().await {
            Ok(fetch) => match fetch.check_status() {
                Ok(good_resp) => Message::LoginMsg(pages::login::Message::GoodLogin(
                    good_resp.json().await.unwrap(),
                )),
                Err(_) => Message::LoginMsg(pages::login::Message::Unauth),
            },
            Err(e) => Message::NetworkError(e),
        }
    });
    let mut res = Model::default();
    res.login = Some(pages::login::Model::default());
    res
}

#[derive(Default, Debug)]
struct Model {
    tabs: Option<tabs::Model>,
    login: Option<pages::login::Model>,
}

impl Model {
    fn init(usr: shared::User) -> Self {
        Self {
            login: None,
            tabs: Some(tabs::Model::init(usr)),
        }
    }
}

#[derive(Debug)]
pub enum Message {
    GoodLogin(shared::User),
    LoginMsg(pages::login::Message),
    TabMessage(tabs::Message),
    NetworkError(fetch::FetchError),
}

// ------ ------
//    Update
// ------ ------

fn update(msg: Message, model: &mut Model, orders: &mut impl Orders<Message>) {
    log("updating");
    use Message::*;
    match msg {
        GoodLogin(usr) => {
            model.login = None;
            model.tabs = Some(tabs::Model::init(usr));
        }
        LoginMsg(msg) => {
            if let Some(GoodLogin(usr)) = pages::login::update(
                msg,
                model.login.as_mut().unwrap(),
                &mut orders.proxy(Message::LoginMsg),
            ) {
                orders.perform_cmd(async move { GoodLogin(usr) });
            }
        }
        TabMessage(msg) => tabs::update(
            msg,
            model.tabs.as_mut().unwrap(),
            &mut orders.proxy(Message::TabMessage),
        ),
        _ => log!("impl me: ", msg),
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Vec<Node<Message>> {
    match (&model.login, &model.tabs) {
        (Some(login), None) => pages::login::login_view(&login).map_msg(Message::LoginMsg),
        (None, Some(_tabs)) => nodes![ul![
            li!["impl non-login pages"],
            li![format!("{:?}", model)]
        ]],
        (li, tab) => nodes![ul![
            li!["one and ONLY one should be Some"],
            li![format!("{:?}", li)],
            li![format!("{:?}", tab)]
        ]],
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
