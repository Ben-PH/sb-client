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
    LoginButton(String),
    LogoutButton(String),
    ServerResponded(fetch::Result<String>),
}

fn update(msg: Message, model: &mut Model, orders: &mut impl Orders<Message>) {
    match msg {
        Message::LoginButton(name) => {
            log!("clickity clackity");
            orders.perform_cmd(async { Message::ServerResponded(send_login(name).await) });
        },
        Message::LogoutButton(name) => {
            log!("clickity clackity");
            orders.perform_cmd(async { Message::ServerResponded(send_logout(name).await) });
        },
        Message::ServerResponded(res) => log!(res),

        // Message::ExampleE(msg) => {
        //     example_e::update(msg, &mut model.example_e, &mut orders.proxy(Message::ExampleE));
        // }
    }
}

async fn send_logout(name: String) -> fetch::Result<String> {
    Request::new(format!("/api/logout/{}", &name))
        .method(fetch::Method::Post)
        .fetch()
        .await?
        .text()
        .await
}

async fn send_login(name: String) -> fetch::Result<String> {
    Request::new(format!("/api/login/{}", &name))
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
    let name = "yee-haw";
    vec![button![
        format!("click me to post to /api/login/{}", name),
        ev(Ev::Click, move |event| {
            event.prevent_default();
            Message::LoginButton(name.to_string())
        })
    ],
    button![
        format!("click me to post to /api/logout/{}", name),
        ev(Ev::Click, move |event| {
            event.prevent_default();
            Message::LogoutButton(name.to_string())
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
