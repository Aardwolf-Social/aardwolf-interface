use yew::prelude::*;
use serde::{Serialize, Deserialize};

// Lets make a struct for testing console logging
#[derive(Serialize, Deserialize)] 
struct LogMe {
    username: String,
    favorite_pizza: String,
}

#[function_component(Aardwolf)]
pub fn aardwolf() -> Html {

    // Demo for logging to the browsers console
    log::info!("Hello browser console! :D");
    log::error!("Ohnoes! There was an OOPSIE!");

    let test_debug = "THAT THING";
    log::debug!("Hey dev, heres the {} you wanted!!", test_debug);

    // Give values to our LogMe struct
    let log_me = LogMe {
        username:"DemoUser".to_owned(),
        favorite_pizza:"Hawaiian".to_owned(),
    };

    // Use j as a variable for our serde_json string?
    let j = serde_json::to_string_pretty(&log_me);

    // Finally pretty print our serde'd struct data x'D!
    log::info!("{:#?}", j);

    // Start of the html! Yew macro
    html!{
        <>
            <h1>{"Aardwolf Yew Genesis"}</h1>
            <p>{"This page is being rendered by Yew $version, and powered by Trunk!"}</p>
        </>
    }

} // End of pub fn aardwolf()
