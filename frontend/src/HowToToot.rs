#![allow(non_snake_case)]
use yew::prelude::*;

#[function_component]
pub fn HowToToot() -> Html {
    html! {
        <div style = "margin-top: 75px">
        <div class="w3-container" id="services" style="margin-left:30%">
        <h5 class="w3-xxxlarge w3-text-red"><b>{"How to Play TOOT-OTTO"}</b></h5>
        <hr style="width:50px;border:5px solid red" class="w3-round"/>
        <p>{"TOOT-OTTO is a fun strategy game for older players who like tic-tac-toe and checkers. One player is TOOT and the other player is OTTO. Both players can place both T's and O's, based on their choice. The first player who spells his or her winning combination - horizontally, vertically or diagonally - wins!"}
        </p>
        <br/>
        <div><h5>{"To play TOOT-OTTO follow the following steps:"}</h5></div>
        <ul>

		    <li>{"A new game describes which player is TOOT and which is OTTO"}</li>

            <li>{"Select the disc type T or O that you want to place"}</li>

		    <li>{"Click on the desired column on the game board to place your disc"}</li>

		    <li>{"Try to spell TOOT or OTTO based on your winning combination, either horizontally or vertically or diagonally"}</li>

	    </ul>
        <br/> {"For More information on TOOT-OTTO click "}<a href="https://boardgamegeek.com/boardgame/19530/toot-and-otto">{"here"}</a>
        </div>
        </div>
    }
}