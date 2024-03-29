use yew::prelude::*;

#[function_component(NotificationContent)]
pub fn notification_content() -> Html {

    html!{
        <>
            <div class="box">
                <article class="media">
                <div class="media-left">
                    <figure>
                    <img src="../web/images/default_avatar.png" alt="Profile Image" aria-hidden="true" />
                    </figure>
                </div>
                <div class="media-content">
                    <div class="content">
                        <strong>{"John Smith"}</strong> <small>{"@aardwolf.social"}</small>
                        <br />
                            {"Favorited your post"}
                        <br />
                        <small><time datetime="2016-1-1">{"11:09 PM - 1 Jan 2016"}</time></small>		  
                    </div>
                </div>
                </article>
            </div>
        </>
    }
}