mod state;

use crate::state::{CellData, Flag, Grid};
use gloo_timers::callback::Interval;
use yew::services::ConsoleService;
use yew::{events::MouseEvent, html, Component, ComponentLink, Html, ShouldRender};

// ToDo: Change background colors based on game result

const NUMBER_OF_ROWS: usize = 10;
const NUMBER_OF_COLUMNS: usize = 10;
const MINE_PROPORTION: [usize; 3] = [10, 5, 3];
const DEFAULT_DIFFICULTY: usize = 0;

pub enum Msg {
    Clicked((usize, MouseEvent)),
    Flagged((usize, Flag)),
    ChangeFlag,
    Reset,
    Loss,
    Win,
    IncrementTimer,
    ChangeDifficulty,
}

#[derive(Eq, PartialEq)]
pub enum GameStatus {
    Playing,
    Lost,
    Won,
}

pub struct Model {
    link: ComponentLink<Self>,
    state: Grid,
    play_status: GameStatus,
    selected_flag: Flag,
    elapsed_time: usize,
    timer_handle: Option<Interval>,
    empty_cells_left: usize,
    selected_difficulty_idx: usize,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = Grid::new(NUMBER_OF_ROWS, NUMBER_OF_COLUMNS, DEFAULT_DIFFICULTY);
        let timer_link = link.clone();
        let empty_cells_left = state.grid_vec.len() - state.mine_count();
        ConsoleService::log(state.to_string().as_str());
        Model {
            link,
            state,
            play_status: GameStatus::Playing,
            selected_flag: Flag::Dig,
            elapsed_time: 0,
            // the Interval tells the model to increment the timer every second
            timer_handle: Some(Interval::new(1000, move || {
                timer_link.send_message(Msg::IncrementTimer)
            })),
            empty_cells_left,
            selected_difficulty_idx: DEFAULT_DIFFICULTY,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked((idx, event)) => {
                ConsoleService::log(format!("Processing a mouse click on cell #{}", idx).as_str());
                match self.selected_flag {
                    Flag::Dig => {
                        // only reveal a cell if it has not been clicked
                        // and the game is still progressing
                        if !self.state.grid_vec[idx].is_clicked
                            && self.play_status == GameStatus::Playing
                        {
                            ConsoleService::log(format!("Digging cell #{}.", idx).as_str());
                            if let CellData::Mine = self.state.grid_vec[idx].data {
                                self.link.send_message(Msg::Loss);
                            } else {
                                let clicked_cells_count = self.state.reveal_empty_cells(idx);
                                ConsoleService::log(
                                    format!(
                                        "Empty cells left: {}\nNewly revealed: {}",
                                        self.empty_cells_left, clicked_cells_count
                                    )
                                    .as_str(),
                                );
                                // ToDo: bugs out
                                self.empty_cells_left -= clicked_cells_count;
                                if self.empty_cells_left == 0 {
                                    self.link.send_message(Msg::Win);
                                }
                            }
                            return true;
                        }
                    }
                    Flag::Tag => {
                        ConsoleService::log(format!("Tagging cell #{}", idx).as_str());
                        self.state.grid_vec[idx].flag = Some(self.selected_flag);
                        return true;
                    }
                }

                false
            }
            Msg::Loss => {
                ConsoleService::log("Game lost.");
                self.play_status = GameStatus::Lost;
                self.timer_handle = None;
                true
            }
            Msg::Reset => {
                self.play_status = GameStatus::Playing;
                self.state = Grid::new(
                    NUMBER_OF_ROWS,
                    NUMBER_OF_COLUMNS,
                    self.selected_difficulty_idx,
                );
                ConsoleService::log(self.state.to_string().as_str());
                self.elapsed_time = 0;
                // dump the old timer and create a new one
                let new_link = self.link.clone();
                self.timer_handle = Some(Interval::new(1000, move || {
                    new_link.send_message(Msg::IncrementTimer)
                }));
                self.empty_cells_left = self.state.grid_vec.len() - self.state.mine_count();
                true
            }
            Msg::Flagged((idx, flag)) => {
                self.state.grid_vec[idx].flag = Some(flag);
                true
            }
            Msg::ChangeFlag => {
                ConsoleService::log("Switching the flag.");
                match self.selected_flag {
                    Flag::Dig => self.selected_flag = Flag::Tag,
                    Flag::Tag => self.selected_flag = Flag::Dig,
                }
                true
            }
            Msg::IncrementTimer => {
                self.elapsed_time += 1;
                true
            }
            Msg::Win => {
                ConsoleService::log("Game won.");
                self.timer_handle = None;
                self.play_status = GameStatus::Won;

                true
            }
            Msg::ChangeDifficulty => {
                self.selected_difficulty_idx =
                    (self.selected_difficulty_idx + 1).rem_euclid(MINE_PROPORTION.len());
                self.play_status = GameStatus::Playing;
                self.state = Grid::new(
                    NUMBER_OF_ROWS,
                    NUMBER_OF_COLUMNS,
                    self.selected_difficulty_idx,
                );
                ConsoleService::log(self.state.to_string().as_str());
                self.elapsed_time = 0;
                // dump the old timer and create a new one
                let new_link = self.link.clone();
                self.timer_handle = Some(Interval::new(1000, move || {
                    new_link.send_message(Msg::IncrementTimer)
                }));
                self.empty_cells_left = self.state.grid_vec.len() - self.state.mine_count();

                true
            }
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
                    <div id="game-status">
                        {
                            match self.play_status {
                                GameStatus::Lost => String::from("🤯"),
                                GameStatus::Won => String::from("😎"),
                                GameStatus::Playing => String::from("🤔"),
                            }
                        }
                    </div>
                    <div id="reset" onclick={ self.link.callback(|_| Msg::Reset ) }>
                        <button>
                        { "Reset" }
                        </button>
                    </div>
                    <div id="current-difficulty">
                        {
                            format!("Mines/Cells: 1/{}",
                                MINE_PROPORTION[self.selected_difficulty_idx])
                        }
                    </div>
                    <div id="change-difficulty" onclick={ self.link.callback(|_| Msg::ChangeDifficulty ) }>
                        <button>
                        { "Change difficulty" }
                        </button>
                    </div>
                    <div id="flag" onclick={ self.link.callback(|_| Msg::ChangeFlag )}>
                        {
                            match self.selected_flag {
                                Flag::Tag => String::from("🚩"),
                                Flag::Dig => String::from("⛏"),
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
    /// Returns Html for a single grid cell
    pub fn view_cell(&self, cell_idx: usize) -> Html {
        html! {
            <div class="cell" id={ format!("cell-{}", cell_idx) }
                onclick={ self.link.callback(move |event| Msg::Clicked((cell_idx, event))) }
            >
                {
                    if self.play_status == GameStatus::Lost || self.state.grid_vec[cell_idx].is_clicked {
                        match self.state.grid_vec[cell_idx].data {
                            CellData::Mine => String::from("💣"),
                            CellData::MineNeighbor(cnt) => format!("{}", cnt),
                        }
                    } else {
                        match self.state.grid_vec[cell_idx].flag {
                            Some(Flag::Tag) => String::from("🚩"),
                            Some(Flag::Dig) => String::from("⛏"),
                            None => String::from("❓"),
                        }
                    }
                }
            </div>
        }
    }

    /// Returns Html for a row of cells
    pub fn view_row(&self, row_idx: usize) -> Html {
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
