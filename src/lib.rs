use seed::{prelude::*, *};


// ------ ------
//     Init
// ------ ------

fn init(_: Url, _: &mut impl Orders<Message>) -> Model {
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
    LoginButton,
    LogoutButton,
    ServerResponded(fetch::Result<String>),
}

fn update(msg: Message, model: &mut Model, orders: &mut impl Orders<Message>) {
    match msg {
        Message::LoginButton => {
            log!("clickity clackity");
            orders.perform_cmd(async { Message::ServerResponded(send_login().await) });
        },
        Message::LogoutButton => {
            log!("clickity clackity");
            orders.perform_cmd(async { Message::ServerResponded(send_logout().await) });
        },
        Message::ServerResponded(res) => log!(res),

        // Message::ExampleE(msg) => {
        //     example_e::update(msg, &mut model.example_e, &mut orders.proxy(Message::ExampleE));
        // }
    }
}

async fn send_logout() -> fetch::Result<String> {
    Request::new("/api/logout/yee-haw")
        .method(fetch::Method::Post)
        .fetch()
        .await?
        .text()
        .await
}

async fn send_login() -> fetch::Result<String> {
    Request::new("/api/login/yee-haw")
        .method(fetch::Method::Post)
        .fetch()
        .await?
        .text()
        .await
}
// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> impl IntoNodes<Message> {
    vec![button![
        "click me to post to /api/login/yee-haw",
        ev(Ev::Click, |event| {
            event.prevent_default();
            Message::LoginButton
        })
    ],
    button![
        "click me to post to /api/logout/yee-haw",
        ev(Ev::Click, |event| {
            event.prevent_default();
            Message::LogoutButton
        })
    ]]

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
