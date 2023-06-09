#![allow(non_snake_case)]
#![allow(unused)]

use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;

use common::TootOtto::{Piece, Player, TootOtto, Difficulty};
use common::Backend::Game;
use stdweb::traits::*;
use wasm_bindgen::{JsCast, JsValue, prelude::Closure};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlInputElement, MouseEvent};
use yew::prelude::*;
use reqwest;

pub struct TootOttoComputer {
    // add any state necessary for the game
    game: Rc<RefCell<TootOtto>>,
    is_game_started: bool,
    p1_name: String,
    winner: String,
    is_game_over: bool,
    is_game_draw: bool,
    selected_letter: char,
    current_player: Player,
    difficulty: usize,
    p1_name_event: Callback<InputEvent>,
    disc_change_event: Callback<MouseEvent>,
    start_event: Callback<MouseEvent>,
    end_event: Callback<String>,
    canvas: NodeRef,
    context: Option<CanvasRenderingContext2d>,
}

pub enum Msg {
    // add any messages needed for the game
    P1NameInput(InputEvent),
    StartGame,
    EndGame,
    SetDisc(String),
    ClickedColumn(Option<usize>),
    Record(),
}

impl TootOttoComputer {
    fn render_board(&mut self) {
        let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();
        let context: CanvasRenderingContext2d = canvas.get_context("2d").unwrap().unwrap().unchecked_into();
        context.save();
        let board = self.game.borrow_mut().get_grid();
        for (row, row_elems) in board.iter().enumerate() {
            for (col, elem) in row_elems.iter().enumerate() {
                if let Some(piece) = elem {
                    match piece {
                        Piece::T => {
                            context.set_fill_style(&JsValue::from("#99ffcc"));
                        }
                        Piece::O => {
                            context.set_fill_style(&JsValue::from("#ffff99"));
                        }
                    }
                    context.begin_path();
                    context.arc(
                        (75 * col + 100) as f64,
                        (75 * row + 50) as f64,
                        25.0,
                        0.0,
                        2.0 * PI,
                    ).expect("Failed to fill text");
                    context.fill();
                    context.set_font("bold 25px serif");
                    context.set_fill_style(&JsValue::from("#111"));
                    let text = match piece {
                        Piece::T => "T",
                        Piece::O => "O",
                    };
                    context.fill_text(text, (75 * col + 92) as f64, (75 * row + 58) as f64).expect("Failed to fill text");
                }
            }
        }

        context.restore();
    }

    fn render_background(&mut self) {
        let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();
        let context: CanvasRenderingContext2d = canvas.get_context("2d").unwrap().unwrap().unchecked_into();
        context.save();
        context.set_fill_style(&JsValue::from("#00bfff"));
        context.begin_path();
        for y in 0..6 {
            for x in 0..7 {
                let _err = context.arc(
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

    fn check_winner(&mut self) {
        match self.game.borrow_mut().winner() {
            None => {}
            Some(x) => {
                if x == Player::Toot {
                    self.winner = self.p1_name.clone();
                    self.is_game_over = true;
                } else {
                    self.winner = "Computer".parse().unwrap();
                    self.is_game_over = true;
                }
                let message = self.winner.to_string() + " wins - Click on game board to reset";
                let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();
                let context: CanvasRenderingContext2d = canvas.get_context("2d").unwrap().unwrap().unchecked_into();
                context.save();
                context.set_font("bold 25px serif");
                context.set_fill_style(&JsValue::from("#111"));
                context.begin_path();
                context.fill_text(&message, (50) as f64, (20) as f64).expect("Failed to fill text");
                context.restore();
                &self.end_event.emit("end".to_string());
            }
        };
        if self.game.borrow_mut().is_draw() == true {
            self.is_game_over = true;
            let message = "It's a draw - Click on game board to reset";
            let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();
            let context: CanvasRenderingContext2d = canvas.get_context("2d").unwrap().unwrap().unchecked_into();
            context.save();
            context.set_font("bold 25px serif");
            context.set_fill_style(&JsValue::from("#111"));
            context.begin_path();
            context.fill_text(message, (50) as f64, (20) as f64).expect("Failed to fill text");
            context.restore();
            &self.end_event.emit("end".to_string());
        }
    }

    fn make_move(&mut self, col: usize) {
        if self.selected_letter == 'T' {
            match self.current_player {
                Player::Toot => {
                    self.game.borrow_mut().make_move_by_toot(col, Piece::T);
                }
                Player::Otto => {
                    self.game.borrow_mut().make_move_by_otto(col, Piece::T);
                }
                Player::AI => {}
            }
        } else {
            match self.current_player {
                Player::Toot => {
                    self.game.borrow_mut().make_move_by_toot(col, Piece::O);
                }
                Player::Otto => {
                    self.game.borrow_mut().make_move_by_otto(col, Piece::O);
                }
                Player::AI => {}
            }
        }
        self.game.borrow_mut().make_move_by_ai();
    }

    fn new_game(&mut self) {
        self.game = Rc::new(RefCell::new(TootOtto::new()));
        match self.difficulty{
            1 => {
                self.game.borrow_mut().set_difficulty(Difficulty::Easy);
            },
            2 => {
                self.game.borrow_mut().set_difficulty(Difficulty::Medium);
            },
            3 => {
                self.game.borrow_mut().set_difficulty(Difficulty::Hard);
            },
            _ => {},
        }
        self.winner = "".to_string();
        self.is_game_over = false;
        self.is_game_draw = false;
        let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();
        let context: CanvasRenderingContext2d = canvas.get_context("2d").unwrap().unwrap().unchecked_into();
        context.clear_rect(0 as f64, 0 as f64, canvas.width() as f64, canvas.height() as f64);
    }
}

impl Component for TootOttoComputer {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let tootOtto = Rc::new(RefCell::new(TootOtto::new()));

        TootOttoComputer {
            // initialize any state necessary for the game
            game: tootOtto.clone(),
            is_game_started: false,
            p1_name: "".to_string(),
            winner: "".to_string(),
            is_game_over: false,
            is_game_draw: false,
            selected_letter: 'T',
            current_player: Player::Toot,
            difficulty: 0,
            p1_name_event: _ctx.link().callback(|e: InputEvent| Msg::P1NameInput(e)),
            disc_change_event: _ctx.link().callback(|e: MouseEvent| {
                let value = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap().value();
                Msg::SetDisc(value)
            }),
            start_event: _ctx.link().callback(|_| Msg::StartGame),
            end_event: _ctx.link().callback(|_| Msg::EndGame),
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
            Msg::StartGame => {
                if self.p1_name != "" {
                    self.is_game_started = true;
                    // get the selected difficulty
                    let document = web_sys::window().unwrap().document().unwrap();
                    let difficulty_drop_down = document.query_selector("#difficulty_drop_down")
                        .unwrap()
                        .unwrap()
                        .dyn_into::<web_sys::HtmlSelectElement>()
                        .unwrap();
                    self.difficulty = difficulty_drop_down.value().parse::<usize>().unwrap();
                    match self.difficulty{
                        1 => {
                            self.game.borrow_mut().set_difficulty(Difficulty::Easy);
                        },
                        2 => {
                            self.game.borrow_mut().set_difficulty(Difficulty::Medium);
                        },
                        3 => {
                            self.game.borrow_mut().set_difficulty(Difficulty::Hard);
                        },
                        _ => {},
                    }
                    let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();
                    // let context: CanvasRenderingContext2d = canvas.get_context("2d")
                    //     .unwrap()
                    //     .unwrap()
                    //     .dyn_into::<web_sys::CanvasRenderingContext2d>()
                    //     .unwrap();
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
                    self.render_background();
                }
            }
            Msg::EndGame => {
                let game = Game {
                        gametype: "TootOtto".to_string(),
                        player1: self.p1_name.clone(),
                        player2: "Computer".to_string(),
                        winner: self.winner.clone(),
                        date: "temp".to_string(),
                    };
                
                _ctx.link().send_future(async move{
                    let client = reqwest::Client::new();
                    match client.post("http://127.0.0.1:8000/post/games").body(serde_json::to_string(&game).unwrap()).send().await{
                        Ok(v) => {
                            Msg::Record()
                        }
                        Err(err) => {
                            Msg::Record()
                        }
                    }});
            }
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
                            // let game = self.game.clone();
                            let row = self.game.borrow_mut().top_row(col);
                            if row != 10 {
                                self.make_move(col);
                                self.render_board();
                                self.check_winner();
                            }
                        }
                    }
                } else {
                    self.new_game();
                    //let link = _ctx.link().clone();
                    self.render_background();
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
                <div class="w3-container" id="services" style="margin-left:25%">
                <h5 class="w3-xxxlarge w3-text-red"><b>{"Enter Player Name"}</b></h5>
                <hr style="width:50px;border:5px solid red" class="w3-round"/>
                    <div class="col-md-offset-3 col-md-8">

                        <input id="P1 Name" type="text" style="margin-right: 5px" placeholder="Your Name" oninput = {&self.p1_name_event}/>
                        <button
                            id="Start Button"
                            onclick={&self.start_event}
                            disabled={self.p1_name == "".to_string()}
                            title="Start Game">
                            { "Start Game" }
                        </button>

                    </div>
                    <div>
                        <label for="difficulty_drop_down"> {"Difficulty: "} </label>
                        <select id="difficulty_drop_down" style="margin-top: 5px">
                        <option value=1 selected=true> {"Beginner"}</option>
                        <option value=2 selected=false> {"Intermediate"}</option>
                        <option value=3 selected=false> {"Professional"}</option>
                        </select>
                    </div>
                </div>
                </div>
            }
        } else {
            html! {
                <div style = "margin-top: 75px">
                    <div class="w3-container" id="services" style="margin-left:30%">
                        <div>
                        <h4>{format!("New Game:  {} Vs Computer",self.p1_name)}</h4>
                        <br/>
                        <small>{format!("(Winning Combination: {} - ", self.p1_name)} <b>{"TOOT"}</b> {format!("   and    Computer - " )} <b>{"OTTO)"}</b></small>
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
