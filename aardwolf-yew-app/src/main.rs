use yew::{prelude::*, Renderer};

fn main() {
    yew::Renderer::<Aardwolf>::new().render();
}

#[function_component(Aardwolf)]
fn aardwolf() -> Html {
    html!{
        <h1>{"Aardwolf Yew Genesis"}</h1>
    }
}