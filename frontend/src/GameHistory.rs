#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use yew::{
    prelude::*
};
use reqwest;
use web_sys::console;

#[derive(Clone,Debug, Serialize, Deserialize)]
pub struct Game {
	pub gametype: String,
    pub player1: String,
    pub player2: String,
	pub winner: String,
	pub date: String,
}

pub struct GameHistory {
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

impl GameHistory {
	fn get_games(&self) -> Html {
        println!("{:?}",self.data);
        let games = self.data.iter().enumerate().map(|(i,game)| html! {
		    <tr>
                <td>{format!("{} ", i+1)}</td>
                <td>{format!("{} ", game.gametype)}</td>
			    <td>{format!("{} ", game.player1)}</td>
			    <td>{format!("{} ", game.player2)}</td>
			    <td>{format!("{} ", game.winner)}</td>
			    <td>{format!("{} ", game.date)}</td>
		    </tr>
	    }).collect::<Html>();
	    games
    }
}

impl Component for GameHistory {
    type Message = FetchStateMsg<Vec<Game>>;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            data: Vec::new(),
            state: FetchState::NotFetching,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
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
                let game = Game {
                    gametype: "Kap".to_string(),
                    player1: "Kap".to_string(),
                    player2: "Kap".to_string(),
                    winner: "Kap".to_string(),
                    date: "temp".to_string(),
                };/*
                _ctx.link().send_future(async move{
                    let client = reqwest::Client::new();
                    match client.post("http://127.0.0.1:8000/t/games").body(serde_json::to_string(&game).unwrap()).send().await{
                        Ok(makrup) => {
                            FetchStateMsg::SetDataFetchState(FetchState::Fetching)
                        }
                        Err(err) => {
                            FetchStateMsg::SetDataFetchState(FetchState::Failed)
                        }
                    }});*/
                _ctx.link().send_future(async move {
                    match reqwest::get("http://127.0.0.1:8000/games").await {
                        Ok(v) => match v.json().await {
                            Ok(v) => {
                                FetchStateMsg::SetDataFetchState(FetchState::Success(v))
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