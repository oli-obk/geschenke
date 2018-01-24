#[macro_use]
extern crate yew;

use yew::prelude::*;

type Context = ();

struct Model {
    n: u32,
}

enum Msg {
    DoIt,
}

impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();
    
    fn create(_: &mut Env<Context, Self>) -> Self {
        Model{
            n: 0,
        }
    }
    // Some details omitted. Explore the examples to get more.
    fn update(&mut self, msg: Self::Msg, _: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::DoIt => {
                self.n += 1;
                true
            }
        }
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        html! {
            // Render your model here
            <button onclick=|_| Msg::DoIt,>{ self.n.to_string() }</button>
        }
    }
}

fn main() {
    yew::initialize();
    let app: App<_, Model> = App::new(());
    app.mount_to_body();
    yew::run_loop();
}
