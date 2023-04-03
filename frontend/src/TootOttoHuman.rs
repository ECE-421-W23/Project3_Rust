use yew::prelude::*;
use common::TootOtto;

pub struct TootOttoHuman {
    // add any state necessary for the game
}

pub enum Msg {
    // add any messages needed for the game
}

impl Component for TootOttoHuman {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            // initialize any state necessary for the game
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // handle any messages needed for the game
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // add the HTML necessary for the game
        html! {
        <div id="main" style="margin-left:30%">
                <form>
                    <div class="w3-container" id="services" style="margin-top:75px">
                        <h5 class="w3-xxxlarge w3-text-red"><b>{"Welcome"}</b></h5>
                        <hr style="width:50px;border:5px solid red" class="w3-round"/>
                        <p>{"This application contains the following two board games, both in human Vs. human and human Vs. Computer versions."}
                        </p>

                        <ul>

                            <li>{"Connect 4"}</li>

                            <li>{"TOOT-OTTO"}</li>


                        </ul>
                        <p>{"Select the game of your choice from the side bar, and start playing. Enjoy!"}</p>
                    </div>
                </form>
            </div>
    }
    }
}