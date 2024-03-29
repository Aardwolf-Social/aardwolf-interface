use yew::prelude::*;

#[function_component(error_http)]
pub fn error_http() -> Html {

    html!{
        <>
            <section class="section">
                <div class="container has-text-centered">
                    <h1 class="title">
                        <strong>{"!! ERHMAGERD !!"}</strong>
                    </h1>
                    <h2>{"error_code"}</h2>
                    <br />
                    <p class="subtitle">
                        {"Something has gone "}<strong class="tag is-danger">{"horribly wrong"}</strong>
                    </p>
                    <figure>
                        <img src="../images/aardwolf-error.jpg" width="256" height="256" alt="Error">
                    </figure>
                </div>
            </section>
        </>
    }
}