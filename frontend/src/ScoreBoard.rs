#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use yew::{
    prelude::*
};
use reqwasm::http::Request;
use web_sys::console;

#[derive(Clone,Debug, Serialize, Deserialize)]
pub struct Game {
    pub player1: String,
    pub player2: String,
	pub winner: String,
	pub date: String,
}

pub struct ScoreBoard {
    // add any state necessary for the game
    state: FetchState<Vec<Game>>,
    data: Vec<Game>,
}

/// The possible states a fetch request can be in.
#[derive(Debug, Clone, PartialEq)]
pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed,
}

pub enum FetchStateMsg<T> {
    SetDataFetchState(FetchState<T>),
    GetData,
}

impl ScoreBoard {
	fn get_games(&self) -> Html {
        println!("{:?}",self.data);
        let videos = self.data.iter().map(|video| html! {
		    <tr>
			    <td>{format!("{} ", video.player1)}</td>
			    <td>{format!("{} ", video.player2)}</td>
			    <td>{format!("{} ", video.winner)}</td>
			    <td>{format!("{} ", video.date)}</td>
		    </tr>
	    }).collect::<Html>();
	    videos
    }
}

impl Component for ScoreBoard {
    type Message = FetchStateMsg<Vec<Game>>;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            data: Vec::new(),
            state: FetchState::NotFetching,
        }
    }
    /*
    fn changed(&mut self, ctx: &Context<Self>, props: Self::Properties) -> bool {
        ctx.link().send_message(FetchStateMsg::GetData);
        true
    }*/

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        console::log_1(&"Hello using web-sys".into());
        match _msg {
            FetchStateMsg::SetDataFetchState(state) => {
                match state.clone() {
                    FetchState::Success(s2) => {
                        self.data = s2;
                    },
                    _=> (),
                }
                self.state = state;
                true
            }
            FetchStateMsg::GetData => {
                _ctx.link().send_future(async move {
                    match Request::get("http://127.0.0.1:8000/data").send().await {
                        Ok(makrup) => match makrup.json().await {
                            Ok(makrup) => {
                                FetchStateMsg::SetDataFetchState(FetchState::Success(makrup))
                            }
                            Err(err) => {
                                FetchStateMsg::SetDataFetchState(FetchState::Failed)
                            }
                        }
                        Err(err) => {
                            FetchStateMsg::SetDataFetchState(FetchState::Failed)
                        }
                    }
                });
                _ctx.link()
                    .send_message(FetchStateMsg::SetDataFetchState(FetchState::Fetching));
                false
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if matches!(&self.state, &FetchState::NotFetching) {
            _ctx.link().send_message(FetchStateMsg::GetData);
        }
        html! {
		    <div style = "margin-top: 75px">
		    <div class="w3-container" id="services" style="margin-left:30%">
		    <h5 class="w3-xxxlarge w3-text-red"><b>{"Game History"}</b></h5>
		    <hr style="width:50px;border:5px solid red" class="w3-round"/>
    
		    <div id="game-stream">
		    <table>
			    <tr>
				    <th>{"Game-ID"}</th>
				    <th>{"Game-Type"}</th>
				    <th>{"Player1"}</th>
				    <th>{"Player2"}</th>
				    <th>{"Winner"}</th>
				    <th>{"Date"}</th>
  			    </tr>
			    { self.get_games() }
		    </table>		
			    </div>
		    </div>
		    </div>
        }
    }

}