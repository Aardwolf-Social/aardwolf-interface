use yew::prelude::*;

// ybc crate imports -- https://crates.io/crates/ybc
use ybc::TileCtx::{Ancestor, Child, Parent};
// use ybc::TileSize; // Unused?
// use ybc::NavbarFixed; // Unused?

#[function_component(YbcHero)]
pub fn ybc_hero() -> Html {

    html!{
        <>
            <ybc::Hero    classes={classes!("is-light")}
    size={ybc::HeroSize::FullheightWithNavbar}
    body={html!{
        <ybc::Container classes={classes!("is-centered")}>
        <ybc::Tile ctx={Ancestor}>
            <ybc::Tile ctx={Parent} size={ybc::TileSize::Twelve}>
                <ybc::Tile ctx={Parent}>
                    <ybc::Tile ctx={Child} classes={classes!("notification", "is-success")}>
                        <ybc::Subtitle size={ybc::HeaderSize::Is3} classes={classes!("has-text-white")}>{"Trunk"}</ybc::Subtitle>
                        <p>{"Trunk is a WASM web application bundler for Rust."}</p>
                    </ybc::Tile>
                </ybc::Tile>
                <ybc::Tile ctx={Parent}>
                    <ybc::Tile ctx={Child} classes={classes!("notification", "is-success")}>
                        <ybc::Icon size={ybc::Size::Large} classes={classes!("is-pulled-right")}><img src="yew.svg"/></ybc::Icon>
                        <ybc::Subtitle size={ybc::HeaderSize::Is3} classes={classes!("has-text-white")}>
                            {"Yew"}
                        </ybc::Subtitle>
                        <p>{"Yew is a modern Rust framework for creating multi-threaded front-end web apps with WebAssembly."}</p>
                    </ybc::Tile>
                </ybc::Tile>
                <ybc::Tile ctx={Parent}>
                    <ybc::Tile ctx={Child} classes={classes!("notification", "is-success")}>
                        <ybc::Subtitle size={ybc::HeaderSize::Is3} classes={classes!("has-text-white")}>{"YBC"}</ybc::Subtitle>
                        <p>{"A Yew component library based on the Bulma CSS framework."}</p>
                    </ybc::Tile>
                </ybc::Tile>
                <ybc::Tile ctx={Parent}>
                    <ybc::Tile ctx={Child} classes={classes!("notification", "is-success")}>
                        <ybc::Subtitle size={ybc::HeaderSize::Is3} classes={classes!("has-text-white")}>{"Function Test"}</ybc::Subtitle>
                        <p>{"This is a list generated from a vec!"}</p>
                        <p>{"How do I get List to HTML in here? :think:"}</p>
                    </ybc::Tile>
                </ybc::Tile>                        
            </ybc::Tile>
        </ybc::Tile>
        </ybc::Container>
    }}>
    </ybc::Hero>
        </>
    }
}

