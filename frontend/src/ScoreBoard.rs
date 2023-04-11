#![allow(non_snake_case)]
#![allow(unused)]
use serde::{Deserialize, Serialize};
use yew::{
    prelude::*
};
use reqwest;
use common::Backend::Game;
use std::collections::HashMap;

pub struct ScoreBoard {
    // add any state necessary for the game
    state: FetchState<Vec<Game>>,
    data: Vec<Game>,
    data2: Vec<Game>,
    data3: Vec<Game>,
}

/// The possible states a fetch request can be in.
#[derive(Debug, Clone, PartialEq)]
pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success1(T),
    Success2(T),
    Success3(T),
    Failed,
}

pub enum FetchStateMsg<T> {
    SetDataFetchState(FetchState<T>),
    GetData,
}

impl ScoreBoard {
	fn get_compgames(&self) -> Html {
        let games = self.data.iter().enumerate().map(|(i,game)| html! {
            if game.player2 == "Computer" {
                <tr>
                    <td>{format!("{} ", i+1)}</td>
                    <td>{format!("{} ", game.gametype)}</td>
			        <td>{format!("{} ", game.winner)}</td>
			        <td>{format!("{} ", game.player1)}</td>
			        <td>{format!("{} ", game.date)}</td>
		        </tr>
            }
	    }).collect::<Html>();
	    games
    }

    fn get_playergames(&self) -> Html {
        let mut result = HashMap::new();
        
        for item in &self.data {
            *result.entry(&item.player1).or_insert(0 as i32) += 1;
            *result.entry(&item.player2).or_insert(0 as i32) += 1;
        }

        let mut vec: Vec<(&String, i32)> = result.into_iter().filter(|x| x.0 != "Computer").collect();
        vec.sort_by_key(|k| k.1);
        vec.reverse();
        let pgames = vec.iter().enumerate().map(|(i,(name,games))| {
            let wins = self.data.iter().filter(|x| x.winner==**name).count();
            html!{
                <tr>
                    <td>{format!("{} ", i+1)}</td>
                    <td>{format!("{} ", name)}</td>
			        <td>{format!("{} ", games)}</td>
                    <td>{format!("{} ", wins)}</td>
		        </tr>
            }}).collect::<Html>();
	    pgames
    }

    fn get_compstat(&self) -> Html {
        let totalgames = self.data.iter().count();
        let compgames = self.data.iter().filter(|x| x.player2=="Computer").count();
        let compwins = self.data.iter().filter(|x| x.winner=="Computer").count();

        let stat = 
        html!{
            <tr>
                <td>{format!("{} ", totalgames)}</td>
			    <td>{format!("{} ", compgames)}</td>
			    <td>{format!("{} ", compwins)}</td>
		    </tr>
        };
	    stat
    }
}

impl Component for ScoreBoard {
    type Message = FetchStateMsg<Vec<Game>>;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        
        Self {
            data: Vec::new(),
            data2: Vec::new(),
            data3: Vec::new(),
            state: FetchState::NotFetching,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        match _msg {
            FetchStateMsg::SetDataFetchState(state) => {
                match state.clone() {
                    FetchState::Success1(s1) => {
                        self.data = s1;
                    },
                    FetchState::Success2(s2) => {
                        self.data2 = s2;
                    },
                    FetchState::Success3(s3) => {
                        self.data3 = s3;
                    },
                    _=> (),
                }
                self.state = state;
                true
            }
            FetchStateMsg::GetData => {
                _ctx.link().send_future(async move {
                    match reqwest::get("http://127.0.0.1:8000/games").await {
                        Ok(v) => match v.json().await {
                            Ok(v) => {
                                FetchStateMsg::SetDataFetchState(FetchState::Success1(v))
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
            <h5 class="w3-xxxlarge w3-text-red"><b>{"Score Board"}</b></h5>
            <hr style="width:50px;border:5px solid red" class="w3-round"/>
                <div><h4>{"Games Won by Computer"}</h4></div>
                    <table>
			                <tr>
			                    <th>{"Total Games Played"}</th>
			                    <th>{"Games Against Computer"}</th>
			                    <th>{"Games Computer Won"}</th>
  			                </tr>
                            { self.get_compstat() }
	                </table>

	            <br/>

    	        <div><h4>{"Detailed Games of Computer"}</h4></div>
	            <div id="game-stream">
	                <table>
			            <tr>
				            <th>{"Sl. No."}</th>
				            <th>{"Game Type"}</th>
			                <th>{"Winner"}</th>
			                <th>{"Played Against"}</th>
			                <th>{"When Played"}</th>
  			            </tr>
                        { self.get_compgames() }
		            </table>

		        <br/>

    	        <div><h4>{"Details of Games Won by All Players"}</h4></div>
	            <div id="game-stream">
	                <table>
			            <tr>
                            <th>{"Ranking"}</th>
				            <th>{"Player Name"}</th>
			                <th>{"Total Games"}</th>
			                <th>{"No. of Wins"}</th>
  			            </tr>
                        { self.get_playergames() }
		            </table>
			        </div>
	        </div>
	
            </div>
            </div>
        }
    }

}