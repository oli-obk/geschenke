#[macro_use]
extern crate yew;

use yew::prelude::*;

type Context = ();

enum Model {
    Login {
        email: String,
        password: String,
    },
    LoggedIn,
}

#[derive(Debug)]
enum LoginMsg {
    Email(String),
    Password(String),
    Login,
}

#[derive(Debug)]
enum Msg {
    Login(LoginMsg),
    Logout,
}

impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();
    
    fn create(_: &mut Env<Context, Self>) -> Self {
        Model::Login {
            email: String::new(),
            password: String::new(),
        }
    }
    // Some details omitted. Explore the examples to get more.
    fn update(&mut self, msg: Self::Msg, _: &mut Env<Context, Self>) -> ShouldRender {
        match *self {
            Model::Login { ref mut email, ref mut password } => match msg {
                Msg::Login(LoginMsg::Email(e)) => {
                    *email = e;
                    false
                }
                Msg::Login(LoginMsg::Password(pw)) => {
                    *password = pw;
                    false
                }
                Msg::Login(LoginMsg::Login) => {
                    unimplemented!();
                    true
                }
                _ => unreachable!("{:?}", msg),
            },
            Model::LoggedIn => match msg {
                Msg::Logout => {
                    *self = Model::Login {
                        email: String::new(),
                        password: String::new(),
                    };
                    true
                },
                _ => unreachable!("{:?}", msg),
            }
        }
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        match *self {
            Model::Login { ref email, ref password } => html! {
                <p>
                <input
                    name="email",
                    value = email,
                    oninput=|e: InputData| Msg::Login(LoginMsg::Email(e.value)),/>
                <input
                    name="password",
                    type="password",
                    value=password,
                    oninput=|e: InputData| Msg::Login(LoginMsg::Password(e.value)), />
                <button onclick=|_| Msg::Login(LoginMsg::Login),>{ "Login" }</button>
                </p>
            },
            Model::LoggedIn => html! {
                <button onclick=|_| Msg::Logout,>{ "Logout" }</button>
            }
        }
    }
}

fn main() {
    yew::initialize();
    let app: App<_, Model> = App::new(());
    app.mount_to_body();
    yew::run_loop();
}
