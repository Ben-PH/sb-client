use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Login {
    email: String,
    password: String,
}

// ------ ------
//     Init
// ------ ------

fn init(_: Url, orders: &mut impl Orders<Message>) -> Model {
    log!("I N I T I A L I Z E");
    orders.subscribe(Message::UrlChanged);
    orders.send_msg(Message::GoToUrl(Url::new().set_path(&["login"])));
    Model::default()
}

// ------ ------
//     Model
// ------ ------

#[derive(Default)]
struct Model;

// ------ ------
//    Update
// ------ ------

enum Message {
    UrlChanged(subs::UrlChanged),
    GoToUrl(Url),
    LoginButton(String, String),
    CreateButton(String, String),
    LogoutButton(String),
    ServerResponded(fetch::Result<String>),
}

fn update(msg: Message, _model: &mut Model, orders: &mut impl Orders<Message>) {
    match msg {
        Message::UrlChanged(change) => log!("changed to", change),
        Message::GoToUrl(url) => {
            log!("going to", url);
            orders.notify(subs::UrlRequested::new(url));
        }
        Message::CreateButton(name, password) => {
            log!("clickity clackity");
            orders
                .perform_cmd(async { Message::ServerResponded(send_create(name, password).await) });
        }
        Message::LoginButton(name, password) => {
            log!("clickity clackity");
            orders
                .perform_cmd(async { Message::ServerResponded(send_login(name, password).await) });
        }
        Message::LogoutButton(_name) => {
            log!("clickity clackity");
            orders.perform_cmd(async {
                Message::ServerResponded(send_logout("f@bar.com".to_string()).await)
            });
        }
        Message::ServerResponded(res) => log!(res, cookies()),
        // Message::ExampleE(msg) => {
        //     example_e::update(msg, &mut model.example_e, &mut orders.proxy(Message::ExampleE));
        // }
    }
}

async fn send_logout(name: String) -> fetch::Result<String> {
    Request::new(format!("/api/auth/{}", &name))
        .method(fetch::Method::Delete)
        .fetch()
        .await?
        .text()
        .await
}

async fn send_login(email: String, password: String) -> fetch::Result<String> {
    Request::new("/api/auth/login".to_string())
        .method(fetch::Method::Post)
        .json(&Login { email, password })?
        .fetch()
        .await?
        .text()
        .await
}

async fn send_create(email: String, password: String) -> fetch::Result<String> {
    Request::new("/api/auth/create".to_string())
        .method(fetch::Method::Post)
        .json(&Login { email, password })?
        .fetch()
        .await?
        .text()
        .await
}
// ------ ------
//     View
// ------ ------

fn view(_model: &Model) -> impl IntoNodes<Message> {
    let name = "f@bar.com";
    vec![
        button![
            "click me to post to /api/auth/login".to_string(),
            ev(Ev::Click, move |event| {
                event.prevent_default();
                Message::LoginButton(name.to_string(), "hunter2".to_string())
            })
        ],
        button![
            format!("click me to DELETE to /api/auth/{}", name),
            ev(Ev::Click, move |event| {
                event.prevent_default();
                Message::LogoutButton(name.to_string())
            })
        ],
        button![
            "navigate to /login",
            ev(Ev::Click, |event| {
                event.prevent_default();
                Url::new().set_path(&["login"]).go_and_load()
            })
        ],
        button![
            "Go to '/foo' and trigger `UrlChanged` (simulate `<a>` link click)",
            ev(Ev::Click, |_| Message::GoToUrl(
                Url::new().set_path(&["foo"])
            ))
        ],
        button![
            "create mr fooey",
            ev(Ev::Click, move |event| {
                event.prevent_default();
                Message::CreateButton("f@bar.com".to_string(), "hunter2".to_string())
            })
        ],
    ]
}

// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
