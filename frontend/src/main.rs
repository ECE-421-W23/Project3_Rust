use yew::html::Scope;
use yew::prelude::*;
use yew_router::prelude::*;

mod Home;
mod HowToConnect4;
mod Connect4Computer;
mod Connect4Human;
mod HowToToot;
mod TootOttoHuman;
mod TootOttoComputer;


#[derive(Routable, Debug, Clone, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/HowToConnect4")]
    HowToConnect4,
    #[at("/Connect4Computer")]
    Connect4Computer,
    #[at("/Connect4Human")]
    Connect4Human,
    #[at("/HowToToot")]
    HowToToot,
    #[at("/TootOttoHuman")]
    TootOttoHuman,
    #[at("/TootOttoComputer")]
    TootOttoComputer,
}


pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                { self.view_nav(ctx.link()) }
                <main>
                    <Switch<Route> render={switch}
                />
                </main>
            </BrowserRouter>
        }
    }
}

impl App {
    fn view_nav(&self, _link: &Scope<Self>) -> Html {
        html! {
            <nav class="w3-sidenav w3-red w3-collapse w3-top w3-large w3-padding" style="z-index:3;width:350px;font-weight:bold" id="mySidenav"><br/>
          <a href="javascript:void(0)" class="w3-padding-xlarge w3-hide-large w3-display-topleft w3-hover-white"
            style="width:100%">{"Close Menu"}</a>
          <div class="w3-container">
            <h3 class="w3-padding-64"><b>{"Play"}<br/>{"Connect4 / TOOT-OTTO"}</b></h3>
          </div>
            <Link<Route> to={Route::HowToConnect4}>{ "How to Play Connect 4"}</Link<Route>>
            <Link<Route> to={Route::Connect4Computer}>{"Play Connect4 With Computer"}</Link<Route>>
            <Link<Route> to={Route::Connect4Human}>{"Play Connect4 with Another Human"}</Link<Route>>
          <br/>
            <Link<Route> to={Route::HowToToot}>{ "How to Play Toot"}</Link<Route>>
            <Link<Route> to={Route::TootOttoComputer}>{ "Play Toot-Otto with Computer"}</Link<Route>>
            <Link<Route> to={Route::TootOttoHuman}>{ "Play Toot-Otto with Another Human"}</Link<Route>>
          <br/>
          <a href="#/ScoreBoard" class="w3-padding w3-hover-white">{"View Game History"}</a>
          <a href="#/Scores" class="w3-padding w3-hover-white">{"Score Board"}</a>
        </nav>
        }
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home::Home />},
        Route::HowToConnect4 => html! {<HowToConnect4::HowToConnect4 />},
        Route::Connect4Computer => html! {<Connect4Computer::Connect4AI />},
        Route::Connect4Human => html! {<Connect4Human::Connect4Human />},
        Route::HowToToot => html! {<HowToToot::HowToToot />},
        Route::TootOttoHuman => html! {<TootOttoHuman::TootOttoHuman />},
        Route::TootOttoComputer => html! {<TootOttoComputer::TootOttoComputer />},
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
    // yew::start_app::<App>();
}


// #[function_component(App)]
// fn app() -> Html {
//     html! {
//         <div>
//         <nav class="w3-sidenav w3-red w3-collapse w3-top w3-large w3-padding" style="z-index:3;width:350px;font-weight:bold" id="mySidenav"><br/>
//           <a href="javascript:void(0)" class="w3-padding-xlarge w3-hide-large w3-display-topleft w3-hover-white"
//             style="width:100%">{"Close Menu"}</a>
//           <div class="w3-container">
//             <h3 class="w3-padding-64"><b>{"Play"}<br/>{"Connect4 / TOOT-OTTO"}</b></h3>
//           </div>
//           <Link<Route> to={Route::Home}>{ "click here to go home" }</Link<Route>>
//           <a href="#/Connect4Computer" class="w3-padding w3-hover-white">{"Play Connect4 With Computer"}</a>
//           <a href="#/Connect4Human" class="w3-padding w3-hover-white">{"Play Connect4 with Another Human"}</a>
//           <br/>
//           <a href="#/HowToToot" class="w3-padding w3-hover-white">{"How to Play TOOT-OTTO"}</a>
//           <a href="#/TootOttoComputer" class="w3-padding w3-hover-white">{"Play Toot-Otto With Computer"}</a>
//           <a href="#/TootOttoHuman" class="w3-padding w3-hover-white">{"Play Toot-Otto With Another Human"}</a>
//           <br/>
//           <a href="#/ScoreBoard" class="w3-padding w3-hover-white">{"View Game History"}</a>
//           <a href="#/Scores" class="w3-padding w3-hover-white">{"Score Board"}</a>
//         </nav>
//         <BrowserRouter>
//             <main>
//                 <Switch<Route> render={switch}
//             />
//             </main>
//         </BrowserRouter>
//         </div>
//     }
// }