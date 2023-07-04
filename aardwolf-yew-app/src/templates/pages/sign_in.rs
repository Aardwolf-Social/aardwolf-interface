use yew::prelude::*;

#[function_component(SignIn)]
pub fn sign_in() -> Html {

    html!{
        <>
            <head>
                <title>{"Aardwolf Social | Sign Up"}</title>
            </head>
            <section class="section">
                <div class="container">
        
                    <div class="columns is-centered">
        
                        // <!-- ******************************* -->
                        // <!-- Begin Left-Hand Column Contents -->
                        // <!-- ******************************* -->
                        <div class="column is-mobile">
                            <h1 class="title">
                                {"About Aardwolf"}
                            </h1>
                            <p class="subtitle">
                                {"This is who we are!"}
                            </p>
                            <p>{"Aardwolf is a platform for creating new social networks, connected across the web. While existing social media sites work to funnel the world into a single shared experience (and advertising marketplace), we recognize that we are all individuals with different identities and interests."}</p><br />
        
                            <p>{"Each server hosts it's own community of users who are posting, sharing pictures, links, etc. They are replying and \"liking\" each other's posts, and re-sharing the ones they like best."}</p><br />
        
                            <p>{"Users are not limited to only interacting with other users on their service: they can follow users on other sites that are powered by Aardwolf just as if they were on their own site (the official term is \"Federation\"). They can even connect with users on other platforms, if -- like the microblogging service"} <a href="http://joinmastodon.org">{"Mastodon"}</a> {"-- they implement the same open protocols Aardwolf is built on."}</p><br />
        
                            <p><a href="https://github.com/banjofox/aardwolf">{"Aardwolf"}</a> {"is open source, so developers who want to contribute or understand how it works can dig in and do so."}</p>
                        </div>// <!-- End Left-Hand Column -->
        
                        // <!-- ******************************** -->
                        // <!-- Begin Right-Hand Column Contents -->
                        // <!-- ******************************** -->			
                        <div class="column is-mobile">
                            <h1 class="title">
                                {"Welcome Back!"}
                            </h1>
                            <p class="subtitle">
                                {"Please login below."}
                            </p>
        
                            <span style="color: red;">{"Error Message"}</span>
                            <form class="box" method="POST" action="/auth">
                                // <!-- Reusable Email Input -->
                                <div class="field">
                                    <label class="label">{"Email Address"}</label>
                                    <div class="control has-icons-left">
                                        <input class="input" type="email" placeholder="e.g. HyenaHugger@SmallCo.net" />
                                        <span class="icon is-small is-left">
                                            <span class="fa fa-envelope"></span>
                                        </span>
                                    </div>
                                </div>
                                // <!-- End Reusable Email Input -->
                                // <!-- Reusable Password Input -->
                                <div class="field">
                                    <label class="label">{"Password"}</label>
                                    <div class="control has-icons-left">
                                        <input class="input" type="password" placeholder="*************" />
                                        <span class="icon is-small is-left">
                                            <span class="fa fa-lock"></span>
                                        </span>
                                    </div>
                                </div>
                                // <!-- End Reusable Password Input -->
                                <button>{"Sign Up"}</button>
                            </form>
                        </div>// <!-- End Left-Hand Column -->
                    </div>
                </div>
            </section>
        </>
    }
}