#![allow(non_snake_case)]

use yew::prelude::*;
use common::TootOtto::{Piece, Player, TootOtto};
use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::document;
use stdweb::web::event::{ClickEvent, ResizeEvent, MouseDownEvent};
use stdweb::web::html_element::{CanvasElement, SelectElement};
// use stdweb::web::{FillRule, window, CanvasRenderingContext2d};
use web_sys::{window, RequestInit};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, Request, Response, MouseEvent, HtmlInputElement};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;


pub struct TootOttoHuman {
    // add any state necessary for the game
    game: Rc<RefCell<TootOtto>>,
    is_game_started: bool,
    p1_name: String,
    p2_name: String,
    winner: String,
    is_game_over: bool,
    is_game_draw: bool,
    selected_letter: char,
    clicked_column: usize,
    columns: usize,
    rows: usize,
    current_player: Player,
    p1_name_event: Callback<InputEvent>,
    p2_name_event: Callback<InputEvent>,
    disc_change_event: Callback<MouseEvent>,
    start_event: Callback<MouseEvent>,
    end_event: Callback<MouseEvent>,
    canvas: NodeRef,
    context: Option<CanvasRenderingContext2d>,
}

pub enum Msg {
    // add any messages needed for the game
    P1NameInput(InputEvent),
    P2NameInput(InputEvent),
    StartGame,
    EndGame,
    SetDisc(String),
    ClickedColumn(Option<usize>),
    Record()
}

impl Component for TootOttoHuman {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let tootOtto = Rc::new(RefCell::new(TootOtto::new()));

        TootOttoHuman {
            // initialize any state necessary for the game
            game: tootOtto.clone(),
            is_game_started: false,
            p1_name: "".to_string(),
            p2_name: "".to_string(),
            winner: "".to_string(),
            is_game_over: false,
            is_game_draw: false,
            selected_letter: 'T',
            clicked_column: 0,
            columns: 7,
            rows: 6,
            current_player: Player::Toot,
            p1_name_event: _ctx.link().callback(|e: InputEvent| Msg::P1NameInput(e)),
            p2_name_event: _ctx.link().callback(|e: InputEvent| Msg::P2NameInput(e)),
            disc_change_event: _ctx.link().callback(|e: MouseEvent| {
                let value = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap().value();
                Msg::SetDisc(value)
            }),
            start_event: _ctx.link().callback(|_| Msg::StartGame),
            end_event: Default::default(),
            canvas: NodeRef::default(),
            context: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // handle any messages needed for the game
            Msg::P1NameInput(e) => {
                if e.data().is_some() {
                    let p1Name = e.data().unwrap().to_owned();
                    self.p1_name += &p1Name;
                }
            }
            Msg::P2NameInput(e) => {
                if e.data().is_some() {
                    let p2Name = e.data().unwrap().to_owned();
                    self.p2_name += &p2Name;
                }
            }
            Msg::StartGame => {
                if self.p1_name != "" && self.p2_name != "" {
                    self.is_game_started = true;
                    let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();
                    let context: CanvasRenderingContext2d = canvas.get_context("2d")
                        .unwrap()
                        .unwrap()
                        .dyn_into::<web_sys::CanvasRenderingContext2d>()
                        .unwrap();
                    let rect = canvas.get_bounding_client_rect();
                    let link = _ctx.link().clone();

                    let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
                        let x_click = event.client_x() - rect.left() as i32;
                        for col in 0..7 {
                            let x_col = 75 * col as i32 + 100;
                            if (x_click - x_col) * (x_click - x_col) <= 25 * 25 {
                                link.send_message(Msg::ClickedColumn(Some(col as usize)));
                                return;
                            }
                        }
                        link.send_message(Msg::ClickedColumn(None));
                    }) as Box<dyn FnMut(_)>);

                    canvas
                        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                        .unwrap();

                    closure.forget();

                    context.save();
                    context.set_fill_style(&JsValue::from("#00bfff"));
                    context.begin_path();
                    for y in 0..6 {
                        for x in 0..7{
                            let err = context.arc(
                                (75 * x + 100) as f64,
                                (75 * y + 50) as f64,
                                25.0,
                                0.0,
                                2.0 * PI,
                            );
                            context.rect((75 * x + 150) as f64, (75 * y) as f64, -100.0, 100.0);
                        }
                    }
                    context.fill();
                    context.restore();
                }
            }
            Msg::EndGame => {}
            Msg::SetDisc(disc) => {
                if disc == "T".to_string() {
                    self.selected_letter = 'T'
                } else {
                    self.selected_letter = 'O'
                }
            }
            Msg::ClickedColumn(column) => {
                if self.is_game_over == false {
                    match column {
                        None => {}
                        Some(col) => {
                            let game = self.game.clone();
                            let row = self.game.borrow_mut().top_row(col);
                            if row != 10 {
                                if self.selected_letter == 'T' {
                                    match self.current_player {
                                        Player::Toot => {
                                            game.borrow_mut().make_move_by_toot(col, Piece::T);
                                            self.current_player = Player::Otto;
                                        }
                                        Player::Otto => {
                                            game.borrow_mut().make_move_by_otto(col, Piece::T);
                                            self.current_player = Player::Toot;
                                        }
                                        Player::AI => {}
                                    }
                                } else {
                                    match self.current_player {
                                        Player::Toot => {
                                            game.borrow_mut().make_move_by_toot(col, Piece::O);
                                            self.current_player = Player::Otto;
                                        }
                                        Player::Otto => {
                                            game.borrow_mut().make_move_by_otto(col, Piece::O);
                                            self.current_player = Player::Toot;
                                        }
                                        Player::AI => {}
                                    }
                                }

                                let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();
                                let context: CanvasRenderingContext2d = canvas.get_context("2d").unwrap().unwrap().unchecked_into();
                                context.save();
                                context.set_fill_style(&JsValue::from("#99ffcc"));
                                context.begin_path();
                                let err = context.arc(
                                    (75 * col + 100) as f64,
                                    (75 * row + 50) as f64,
                                    25.0,
                                    0.0,
                                    2.0 * PI,
                                );
                                context.fill();
                                context.restore();
                                context.set_font("bold 25px serif");
                                let mut text = "O";
                                if self.selected_letter == 'T' {
                                    text = "T";
                                }
                                context.fill_text(text, (75 * col + 92) as f64, (75 * row + 58) as f64);

                                match game.borrow_mut().winner() {
                                    None => {}
                                    Some(x) => {
                                        if x == Player::Toot {
                                            self.winner = self.p1_name.clone();
                                            self.is_game_over = true;
                                        } else {
                                            self.winner = self.p2_name.clone();
                                            self.is_game_over = true;
                                        }
                                        let message = self.winner.to_string() + " wins - Click on game board to reset";
                                        context.save();
                                        context.set_fill_style(&JsValue::from("#111"));
                                        context.begin_path();
                                        context.fill_text(&message, (150) as f64, (20) as f64);
                                        context.restore();
                                    }
                                };
                            }
                        }
                    }
                } else {
                    self.game =  Rc::new(RefCell::new(TootOtto::new()));
                    self.winner = "".to_string();
                    self.is_game_over = false;
                    self.is_game_draw = false;
                    let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();
                    let context: CanvasRenderingContext2d = canvas.get_context("2d").unwrap().unwrap().unchecked_into();
                    context.clear_rect(0 as f64, 0 as f64, canvas.width() as f64, canvas.height() as f64);
                    let link = _ctx.link().clone();
                    context.save();
                    context.set_fill_style(&JsValue::from("#00bfff"));
                    context.begin_path();
                    for y in 0..6 {
                        for x in 0..7{
                            let err = context.arc(
                                (75 * x + 100) as f64,
                                (75 * y + 50) as f64,
                                25.0,
                                0.0,
                                2.0 * PI,
                            );
                            context.rect((75 * x + 150) as f64, (75 * y) as f64, -100.0, 100.0);
                        }
                    }
                    context.fill();
                    context.restore();
                }
            }
            Msg::Record() => {}
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {


        html! {
            <>
        // add the HTML necessary for the game
        {if !self.is_game_started {
            html! {
                <div style = "margin-top: 75px">
                <div class="w3-container" id="services" style="margin-left:30%">
                <h5 class="w3-xxxlarge w3-text-red"><b>{"Enter Player Names"}</b></h5>
                <hr style="width:50px;border:5px solid red" class="w3-round"/>
                    <div class="col-md-offset-3 col-md-8">

                        <input id="P1 Name" type="text" placeholder="Player 1's Name" oninput = {&self.p1_name_event}/>
                        <input id="P2 Name" type="text" placeholder="Player 2's Name" oninput = {&self.p2_name_event}/>
                        <button
                            id="Start Button"
                            onclick={&self.start_event}
                            disabled={false}
                            title="Start Game">
                            { "Start Game" }
                        </button>

                    </div>
                </div>
                </div>
            }
        } else {
            html! {
                <div style = "margin-top: 75px">
                    <div class="w3-container" id="services" style="margin-left:30%">
                        <div>
                        <h4>{format!("New Game:  {} Vs {}",self.p1_name, self.p2_name)}</h4>
                        <br/>
                        <small>{format!("(Winning Combination: {} - ", self.p1_name)} <b>{"TOOT"}</b> {format!("   and    {} - ", self.p2_name)} <b>{"OTTO)"}</b></small>
                        <br/>
                        {"Select a Disc Type:  "}
                        <input type="radio" name="choice" value="T" checked ={self.selected_letter == 'T'} onclick = {&self.disc_change_event}/> {"T"}
                        <input type="radio" name="choice" value="O" checked ={self.selected_letter == 'O'} onclick = {&self.disc_change_event}/> {"O"}
                        </div>
                        </div>
                </div>
                }
                }
            }
                <div style = "margin-top: 75px">
                <div class="w3-canvas" id="services" style="margin-left:30%">
                            <canvas
                            id="canvas"
                            height = "480" width = "640"
                            ref={self.canvas.clone()}>
                            </canvas>
                </div>
                </div>
            </>
        }
    }

}

macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}