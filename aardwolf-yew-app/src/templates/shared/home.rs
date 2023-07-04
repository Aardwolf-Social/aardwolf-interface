use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {

    html!{
        <>
        <div class="columns is-mobile">// <!-- Column Content Container -->
            <div class="left-column columns is-3">// <!-- Left-column (Nav) -->
          
              // <!-- ****************** -->
              // <!-- Left-hand Nav Menu -->
              // <!-- ****************** -->
          
            </div>// <!-- /Left-column (Nav)-->
            <div class="right-column columns">// <!-- Right-column (Main content) -->
              <section class="section">// <!-- Right-column Section -->
          
              // <!-- *********************** -->
              // <!-- Right-hand Body Content -->
              // <!-- *********************** -->
            
              </section>// <!-- /Right-column Section -->
            </div>// <!-- /Right-column -->
        </div>// <!-- /Column Content Container -->
        </>
    }
}