use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;

use common::Connect4::{Connect4, Piece, Player};
use stdweb::traits::*;
use stdweb::web::document;
use stdweb::web::event::{ClickEvent, MouseDownEvent, ResizeEvent};
use stdweb::web::html_element::{CanvasElement, SelectElement};
use wasm_bindgen::{JsCast, JsValue, prelude::Closure};
use wasm_bindgen_futures::JsFuture;
use web_sys::{RequestInit, window};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlInputElement, MouseEvent, Request, Response};
use yew::prelude::*;

pub struct Connect4Human {
    game: Rc<RefCell<Connect4>>,
    is_game_started: bool,
    player1: String,
    player2: String,
    winner: String,
    is_game_over: bool,
    clicked_column: usize,
    columns: usize,
    rows: usize,
    current_player: Player,
    end_event: Callback<MouseEvent>,
    canvas: NodeRef,
    context: Option<CanvasRenderingContext2d>,
}

pub enum Msg {
    SetPlayer1Name(InputEvent),
    SetPlayer2Name(InputEvent),
    Connect4,
    EndGame,
    ClickedColumn(Option<usize>),
}

impl Connect4Human {
    fn render_board(&mut self) {
        let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();
        let context: CanvasRenderingContext2d = canvas.get_context("2d").unwrap().unwrap().unchecked_into();
        context.save();
        let board = self.game.borrow().get_grid();
        for (row, row_elems) in board.iter().enumerate() {
            for (col, elem) in row_elems.iter().enumerate() {
                if let Some(piece) = elem {
                    let color = match piece {
                        Piece::R => "#ff0000",
                        Piece::Y => "#ffff00",
                    };
                    context.set_fill_style(&JsValue::from(color));
                    context.begin_path();
                    context.arc(
                        (75 * col + 100) as f64,
                        (75 * row + 50) as f64,
                        25.0,
                        0.0,
                        2.0 * PI,
                    );
                    context.fill();
                    context.set_font("bold 25px serif");
                    context.set_fill_style(&JsValue::from("#111"));
                    let text = match piece {
                        Piece::R => "R",
                        Piece::Y => "Y",
                    };
                    context.fill_text(text, (75 * col + 92) as f64, (75 * row + 58) as f64);
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
        for y in 0..self.rows {
            for x in 0..self.columns {
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

    fn check_winner(&mut self) {
        match self.game.borrow_mut().winner() {
            Some(x) => {
                self.is_game_over = true;
                if x == Player::Red {
                    self.winner = self.player1.clone();
                } else {
                    self.winner = self.player2.clone();
                }
                let message = self.winner.to_string() + " wins - Click on game board to reset";
                let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();
                let context: CanvasRenderingContext2d = canvas.get_context("2d").unwrap().unwrap().unchecked_into();
                context.save();
                context.set_font("bold 25px serif");
                context.set_fill_style(&JsValue::from("#111"));
                context.begin_path();
                context.fill_text(&message, (50) as f64, (20) as f64);
                context.restore();
            }
            None => {}
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
            context.fill_text(message, (50) as f64, (20) as f64);
            context.restore();
        }
    }

    fn make_move(&mut self, col: usize) {
        self.game.borrow_mut().user_move(col);
    }

    fn new_game(&mut self) {
        self.game = Rc::new(RefCell::new(Connect4::new()));
        self.winner = "".to_string();
        self.is_game_over = false;
        let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();
        let context: CanvasRenderingContext2d = canvas.get_context("2d").unwrap().unwrap().unchecked_into();
        context.clear_rect(0 as f64, 0 as f64, canvas.width() as f64, canvas.height() as f64);
    }
}


impl Component for Connect4Human {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let connect4 = Rc::new(RefCell::new(Connect4::new()));
        Self {
            game: connect4.clone(),
            is_game_started: false,
            player1: "".to_string(),
            player2: "".to_string(), //just done to add name on the scoreboard
            winner: "".to_string(),
            is_game_over: false,
            clicked_column: 0,
            columns: 7,
            rows: 6,
            current_player: Player::Red,
            end_event: Default::default(),
            canvas: NodeRef::default(),
            context: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Connect4 => {
                // handle starting the game here
                self.is_game_started = true;
                let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();
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
            Msg::SetPlayer1Name(e) => {
                if e.data().is_some() {
                    self.player1 += &e.data().unwrap().to_owned();
                }
            }
            Msg::SetPlayer2Name(e) => {
                if e.data().is_some() {
                    self.player2 += &e.data().unwrap().to_owned();
                }
            }
            Msg::EndGame => {}
            Msg::ClickedColumn(column) => {
                if self.is_game_over == false {
                    match column {
                        None => {}
                        Some(col) => {
                            let row = self.game.borrow_mut().top_row(col);
                            if row < (self.rows as usize) {
                                self.make_move(col);
                                self.render_board();
                                self.check_winner();
                            }
                        }
                    }
                } else {
                    self.new_game();
                    self.render_background();
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
        {if !self.is_game_started {
            html! {
                <div id="main"  style="margin-left:30%">
                <div class="w3-container" id="services" style="margin-top:75px">
                    <h5 class="w3-xxxlarge w3-text-red"><b>{"Enter Your Name"}</b></h5>
                    <hr style="width:50px;border:5px solid red" class="w3-round"/>
                </div>
                <div class="col-md-offset-3 col-md-8">
                    <input id="textbox1" type="text" placeholder="Player1 Name" oninput={ctx.link().callback(|e: InputEvent| Msg::SetPlayer1Name(e))}/>
                    <input id="textbox2" type="text" placeholder="Player2 Name" oninput={ctx.link().callback(|e: InputEvent| Msg::SetPlayer2Name(e))}/>
                    <input id="startbutton" class="button" type="Submit" onclick={ctx.link().callback(|_| Msg::Connect4)} disabled = {self.player1 == "".to_string() || self.player2 == "".to_string()}/>
                    </div>
                </div>
            }
        } else {
            html! {
                <div style = "margin-top: 75px">
                    <div class="w3-container" id="services" style="margin-left:30%">
                        <div>
                        <h4>{format!("New Game:  {} Vs {}",self.player1, self.player2)}</h4>
                        <br/>
                        <small>{format!("(Piece Alloted: {} - ", self.player1)} <b>{"Red"}</b> {format!("   and    {} - ", self.player2)} <b>{"Yellow)"}</b></small>
                        <br/>
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


