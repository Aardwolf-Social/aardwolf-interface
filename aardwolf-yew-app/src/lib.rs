use yew::prelude::*;

#[function_component(Aardwolf)]
pub fn aardwolf() -> Html {
    html!{
        <>
            <h1>{"Aardwolf Yew Genesis"}</h1>
            <p>{"This page is being rendered by Yew $version, and powered by trunk!"}</p>
        </>
    }
}
