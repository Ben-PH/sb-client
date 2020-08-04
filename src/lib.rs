
use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};

fn init(_: Url, orders: &mut impl Orders<Message>) -> Model {
    log!("I N I T I A L I Z E");
    // orders.subscribe(Message::UrlChanged);
    // orders.send_msg(Message::GoToUrl(Url::new().set_path(&["login"])));
    Model::default()
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Login {
    email: String,
    password: String,
}

// ------ ------
//     Model
// ------ ------

#[derive(Debug, Default)]
struct Model {
    login: Login,
    pub response_data: Option<String>,
}
#[derive(Debug)]
enum Message{
    EmailChanged(String),
    PasswordChanged(String),
    LoginButton,
    LogoutButton,
    CreateButton,
    Fetched(fetch::Result<String>),
}


// ------ ------
//    Update
// ------ ------

fn update(msg: Message, model: &mut Model, orders: &mut impl Orders<Message>) {
    match msg {
        Message::EmailChanged(email) => model.login.email = email,
        Message::PasswordChanged(password) => model.login.password = password,
        Message::LoginButton => {
            let request = Request::new("/api/auth/login")
                .method(Method::Post)
                .json(&model.login)
                .expect("Serialization failed");

            orders.perform_cmd(
                async {
                    let response = fetch(request)
                        .await
                        .unwrap()
                        .text()
                        .await;

                    Message::Fetched(response)
                }
            );
        },
        Message::Fetched(res) => match res {
            Ok(response_data) => model.response_data = Some(response_data),
            Err(res) => log!(res),
        }
        Message::LogoutButton => {
            let request = Request::new("/api/auth")
                .method(Method::Delete);

            orders.perform_cmd(
                async {
                    let response = fetch(request)
                        .await
                        .unwrap()
                        .text()
                        .await;

                    Message::Fetched(response)
                }
            );
        }
        // Message::CreateButton => {
        _ => log!("TODO: impl handling for ", msg)
    }
}

async fn send_logout(orders: &mut impl Orders<Message>) {
    let request = Request::new("/api/auth")
        .method(Method::Delete);

    orders.perform_cmd(
        async {
            let response = fetch(request)
                .await
                .unwrap()
                .text()
                .await;

            Message::Fetched(response)
        });
}

async fn send_login(ser: &impl Serialize) -> fetch::Result<String> {
    Request::new("/api/auth/login")
        .method(fetch::Method::Post)
        .json(ser)?
        .fetch()
        .await?
    .check_status()?
        .text()
        .await
}

async fn send_create(email: String, password: String) -> fetch::Result<String> {
    Request::new("/api/auth/create")
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

fn view(model: &Model) -> Vec<Node<Message>> {
    nodes![
        input![
            input_ev(Ev::Input, Message::EmailChanged),
        ],
        input![
            input_ev(Ev::Input, Message::PasswordChanged),
        ],
        button![ev(Ev::Click, |_| Message::LoginButton), "login"],
        button![ev(Ev::Click, |_| Message::LogoutButton), "logout"],
        button![ev(Ev::Click, |_| Message::CreateButton), "create"],
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
