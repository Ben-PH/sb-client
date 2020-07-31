use seed::{prelude::*, *};


// ------ ------
//     Init
// ------ ------

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
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

enum Msg {
    Nothing
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Nothing => {}
        // Msg::ExampleE(msg) => {
        //     example_e::update(msg, &mut model.example_e, &mut orders.proxy(Msg::ExampleE));
        // }
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> impl IntoNodes<Msg> {
    div!["placeholder"]
}

// ------ ------
//     Start
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
