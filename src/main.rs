mod state;

use crate::state::{CellData, Flag, Grid};
use gloo::timers::callback::Interval;
use yew::{events::MouseEvent, html, Callback, Component, ComponentLink, Html, ShouldRender};

// ToDo: Check for winner status
// ToDo: Add right click flagging
// ToDo: Add logging

const NUMBER_OF_ROWS: usize = 10;
const NUMBER_OF_COLUMNS: usize = 10;

pub enum Msg {
    // User actions
    Clicked(usize),
    Flagged((usize, Flag)),
    ChangeFlag,
    Reset,
    // Grid logic responses
    Loss,
    Win,
    OK,
    IncrementTimer,
}

#[derive(Eq, PartialEq)]
pub enum GameStatus {
    Playing,
    GameOver,
}

pub struct Model {
    link: ComponentLink<Self>,
    state: Grid,
    play_status: GameStatus,
    selected_flag: Flag,
    elapsed_time: usize,
    timer_handle: Option<Interval>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = Grid::new(NUMBER_OF_ROWS, NUMBER_OF_COLUMNS);
        let timer_link = link.clone();
        Model {
            link,
            state,
            play_status: GameStatus::Playing,
            selected_flag: Flag::Mine,
            elapsed_time: 0,
            timer_handle: Some(Interval::new(1000, move || {
                timer_link.send_message(Msg::IncrementTimer)
            })),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked(idx) => {
                if !self.state.grid_vec[idx].is_clicked {
                    if self.state.grid_vec[idx].is_mine {
                        self.link.callback(|_| Msg::Loss).emit(());
                    } else {
                        self.state.grid_vec[idx].is_clicked = true;
                    }
                    return true;
                }
                false
            }
            Msg::Loss => {
                self.play_status = GameStatus::GameOver;
                self.timer_handle = None;
                true
            }
            Msg::Reset => {
                self.play_status = GameStatus::Playing;
                self.state = Grid::new(NUMBER_OF_ROWS, NUMBER_OF_COLUMNS);
                self.elapsed_time = 0;
                let new_link = self.link.clone();
                self.timer_handle = Some(Interval::new(1000, move || {
                    new_link.send_message(Msg::IncrementTimer)
                }));
                true
            }
            Msg::Flagged((idx, flag)) => {
                self.state.grid_vec[idx].flag = Some(flag);
                true
            }
            Msg::ChangeFlag => {
                match self.selected_flag {
                    Flag::Mine => self.selected_flag = Flag::Empty,
                    Flag::Empty => self.selected_flag = Flag::Mine,
                }
                true
            }
            Msg::IncrementTimer => {
                self.elapsed_time += 1;
                true
            }
            _ => false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        todo!()
    }

    fn view(&self) -> Html {
        html! {
            <div id="game">
                <h1>
                    { "A minesweeper game made with Rust and Yew." }
                </h1>
                <div id="controls">
                    <div id="reset" onclick={ self.link.callback(|_| Msg::Reset ) }>
                        { "Reset" }
                    </div>
                    <div id="flag" onclick={ self.link.callback(|_| Msg::ChangeFlag )}>
                        {
                            match self.selected_flag {
                                Flag::Mine => String::from("🚩"),
                                Flag::Empty => String::from("⛏"),
                        }
                        }
                    </div>
                    <div id="timer">
                        { self.elapsed_time }
                    </div>
                </div>
                <div id="grid">
                    <div class="column-container">
                        { for (0..self.state.n_rows).map(|row| self.view_row(row)) }
                    </div>
                </div>
            </div>
        }
    }
}

impl Model {
    pub fn view_cell(&self, cell_idx: usize) -> Html {
        html! {
            <div class="cell" id={ format!("cell-{}", cell_idx) }
                onclick={ self.link.callback(move |_| Msg::Clicked(cell_idx.clone())) }
            >
                {
                    if self.play_status == GameStatus::GameOver || self.state.grid_vec[cell_idx].is_clicked {
                        match self.state.grid_vec[cell_idx].data {
                            CellData::Mine => String::from("💣"),
                            CellData::MineNeighbor(cnt) => format!("{}", cnt),
                        }
                    } else {
                        match self.state.grid_vec[cell_idx].flag {
                            Some(Flag::Mine) => String::from("🚩"),
                            Some(Flag::Empty) => String::from("⛏"),
                            None => String::from("❓"),
                        }
                    }
                }
            </div>
        }
    }

    pub fn view_row(&self, row_idx: usize) -> Html {
        // let a = (0..self.state.n_cols)
        //     .map(|col| Grid::xy_to_idx((row_idx, col), self.state.n_rows, self.state.n_cols))
        //     .map(|idx| self.cell_view(idx));
        html! {
            <div class="row-container">
                { for (0..self.state.n_cols)
                        .map(|col| Grid::xy_to_idx((row_idx, col),
                            self.state.n_rows, self.state.n_cols))
                        .map(|idx| self.view_cell(idx.unwrap()))
                }
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
