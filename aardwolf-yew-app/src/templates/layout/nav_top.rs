use yew::prelude::*;

#[function_component(NavTop)]
pub fn nav_top() -> Html {

    html!{
        <>
        <nav class="navbar" role="navigation" aria-label="main navigation">
            <div class="navbar-brand">
              <a class="navbar-item">
                    <strong>{"INSTANCE TITLE"}</strong>
              </a>  
              <a class="navbar-item" href="https://aardwolf.social">
                     <img src="/web/images/aardwolf-logo.png" height="100" alt="Aardwolf" />
              </a>
              <a role="button" class="navbar-burger burger" aria-label="menu" aria-expanded="false" data-target="navbarBasicExample">
                <span aria-hidden="true"></span>
                <span aria-hidden="true"></span>
                <span aria-hidden="true"></span>
              </a>
              <a class="navbar-item">
                    <strong>{"USERNAME"}</strong>
              </a>  	
            </div>//<!-- /navbar-brand -->
          
            <div id="navbarBasicExample" class="navbar-menu">
              <div class="navbar-end">
                  //<!-- Nav Right-hand Menu -->	
                  <a class="navbar-item">
                    {"Home"}
                  </a>
                  <a class="navbar-item" href="#">
                    {"My Notifications"}
                  </a>
                  <notification-box v-if="showNotificationBox"></notification-box>
                //<!-- Nav Right-hand Dropdown -->
                <div class="navbar-item has-dropdown is-hoverable">
                  <a class="navbar-link">
                    {"More"}
                  </a>
                    <div class="navbar-dropdown">
                      <a class="navbar-item">
                        {"About"}
                      </a>
                      <a class="navbar-item" href="../templates/asides/aside_settings.html">
                        {"Settuings"}
                      </a>
                      <a class="navbar-item">
                        {"Contact"}
                      </a>
                      <hr class="navbar-divider" />
                      <a class="navbar-item">
                        {"Report an issue"}
                      </a>
                    </div>
                </div>
                //<!-- /Nav Dropdown -->
                //<!-- Nav Login/Logout Buttons -->
                <div class="navbar-item">
                  <div class="buttons">
                    <a class="button is-light">
                      {"Logout"}
                    </a>
                  </div>
                </div>
                //<!-- /Nav Login/Logout Buttons -->	  
              </div>//<!-- /navbar-end -->
            </div>//<!-- /navbarBasicExample -->
          </nav>
        </>
    }
}