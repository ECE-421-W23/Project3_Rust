use yew::prelude::*;

/*
#[derive(Routable, Debug, Clone, PartialEq)]
pub enum Route {
    //#[at("/HowToConnect4")]
    //HowToConnect4,
}

 */


pub struct Connect4Human {
    player1: Option<String>,
    player2: Option<String>,
}

pub enum Msg{
    Connect4,
    SetPlayer1Name(Option<String>),
    SetPlayer2Name(Option<String>),
}

impl Component for Connect4Human {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            player1: None,
            player2: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Connect4 => {
                // handle starting the game here
            }
            Msg::SetPlayer1Name(name) => {
                self.player1 += name.unwrap();
            }
            Msg::SetPlayer2Name(name) => {
                self.player2 += name.unwrap();
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div id="main"  style="margin-left:30%">
                <div class="w3-container" id="services" style="margin-top:75px">
                    <h5 class="w3-xxxlarge w3-text-red"><b>{"Enter Your Name"}</b></h5>
                    <hr style="width:50px;border:5px solid red" class="w3-round"/>
                </div>
                <div class="col-md-offset-3 col-md-8">
                    <input id="textbox1" type="text" placeholder="Player 1's Name" oninput={ctx.link().callback(|e: InputEvent| Msg::SetPlayer1Name(e.data()))}/>
                    <input id="textbox2" type="text" placeholder="Player 2's Name" oninput={ctx.link().callback(|e: InputEvent| Msg::SetPlayer2Name(e.data()))}/>
                    <input id="startbutton" class="button" type="submit" onclick={ctx.link().callback(|_| Msg::Connect4)} disabled = {self.player1.is_none() || self.player2.is_none()}/>
                </div>

                <div class="post">
                    <br/>
                    <h4>{"New Game:"} {self.player1.as_ref().unwrap_or(&"".to_string())} {"Vs"} {self.player2.as_ref().unwrap_or(&"".to_string())}</h4>
                    //<small>({"Disc Colors: "} {self.player1.as_ref().unwrap_or(&"".to_string())} - <b>{"Red"}</b>    {"and"}    {self.player2.as_ref().unwrap_or(&"".to_string())} - <b>{"Yellow"}</b>)</small>
                    <br/>
                </div>
            </div>
        }
    }
}

