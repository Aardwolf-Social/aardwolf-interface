use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>      
        <div class="columns is-mobile">
            <div class="left-column columns is-3">
            // Left Column Nav
            </div>
            <div class="right-column columns">
              <section class="section">
              // Right Column, main body
              </section>
            </div>
        </div>
        </main>
    }
}
