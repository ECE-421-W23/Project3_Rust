use yew::prelude::*;

/*
#[derive(Routable, Debug, Clone, PartialEq)]
pub enum Route {
    //#[at("/HowToConnect4")]
    //HowToConnect4,
}

 */


pub struct Connect4AI{
    player1: Option<String>,
    player2: Option<String>,
    difficulty: usize,
}

pub enum Msg{
    Connect4,
    SetPlayer1Name(Option<String>),
    SetDifficulty(usize),
}

impl Component for Connect4AI {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            player1: None,
            player2: Some("Computer".to_string()), //just done to add name on the scoreboard
            difficulty: 1,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Connect4 => {
                // handle starting the game here
            }
            Msg::SetPlayer1Name(name) => {
                self.player1 = name;
            }
            Msg::SetDifficulty(difficulty) => {
                self.difficulty = difficulty;
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
                    <input id="textbox1" type="text" placeholder="Your Name" oninput={ctx.link().callback(|e: InputEvent| Msg::SetPlayer1Name(e.data()))}/>

                    /*
                    <label for="difficulty_drop_down"> {"Difficulty:"} </label>
                        <select id="difficulty_drop_down" style="margin-left: 5px" onchange={ctx.link().callback(move |e: ChangeData| {
                                let difficulty = match e
                                    .select_element()
                                    .and_then(|el| el.value().parse::<usize>().ok())
                                {
                                    Some(d) => d,
                                    None => 1, // default to easy if parsing fails
                                };
                                Msg::SetDifficulty(difficulty)
                            })}>
                            <option value=1> {"Easy"}</option>
                            <option value=2> {"Medium"}</option>
                            <option value=3> {"Hard"}</option>
                        </select>

                     */

                    <input id="startbutton" class="button" type="submit" onclick={ctx.link().callback(|_| Msg::Connect4)} disabled = {self.player1.is_none()}/>
                </div>
            </div>
        }
    }
}

