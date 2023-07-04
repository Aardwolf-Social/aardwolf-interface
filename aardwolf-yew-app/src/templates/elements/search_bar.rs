use yew::prelude::*;

#[function_component(HTMLThing)]
pub fn html_thing() -> Html {

    html!{
        <>
            <div class="search-bar field">
                <p class="control has-icons-left has-icons-right">
                    <input class="input" type="text" placeholder="Search" />
                    <span class="icon is-small is-left">
                        <i class="fa fa-search" aria-hidden="true"></i>
                    </span>
                </p>
            </div>
        </>
    }
}