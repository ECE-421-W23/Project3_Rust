#![allow(non_snake_case)]

use yew::prelude::*;

#[function_component]
pub fn Home() -> Html {
    html! {
        <div id="main" style="margin-left:25%">
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