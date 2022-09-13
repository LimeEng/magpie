use crate::agent::{Action, Agent, HumanAgent, RandomAgent};
use magpie::othello::{Board, Stone};

pub mod agent;

fn main() {
    play();
}

fn play() {
    let mut agent1 = HumanAgent;
    let mut agent2 = RandomAgent;

    let mut active_agent = ActiveAgent::new(&mut agent1, &mut agent2);

    let mut board = Board::standard();
    let mut passed_last_turn = false;
    while board.empty_squares() != 0 {
        let current_stone = active_agent.current_stone();
        println!("{}", board.display().with_stone(current_stone));
        let legal_moves = board.moves_for(current_stone);
        if legal_moves == 0 {
            println!("{:?} have no moves to make", current_stone);
            if passed_last_turn {
                // In Othello, if both players passed their turn in succession,
                // the game is over.
                println!("No player have any legal moves to make, exiting");
                break;
            }
            passed_last_turn = true;
        } else {
            let action = active_agent.play(&board);
            passed_last_turn = false;
            match action {
                Action::Move(next_move) => {
                    board
                        .place_stone(current_stone, next_move)
                        .unwrap_or_else(|_| {
                            panic!("{:?} tried to make an illegal move", current_stone)
                        });
                    println!("{:?} played {}", current_stone, next_move.to_notation());
                }
                Action::Pass => {
                    passed_last_turn = true;
                    println!("{:?} passed their turn", current_stone);
                }
            }
        }
        active_agent.next_player();
    }
    println!("Final board");
    println!("{}", board.display());
    let black = board.bits_for(Stone::Black).count_set();
    let white = board.bits_for(Stone::White).count_set();
    println!("Game finished with {} - {} (black - white)", black, white);
}

struct ActiveAgent<'a> {
    black: &'a mut dyn Agent,
    white: &'a mut dyn Agent,
    active_stone: Stone,
}

impl<'a> ActiveAgent<'_> {
    pub fn new(black: &'a mut dyn Agent, white: &'a mut dyn Agent) -> ActiveAgent<'a> {
        ActiveAgent {
            black,
            white,
            active_stone: Stone::Black,
        }
    }

    pub fn play(&mut self, board: &Board) -> Action {
        match self.active_stone {
            Stone::Black => self.black.play(self.active_stone, board),
            Stone::White => self.white.play(self.active_stone, board),
        }
    }

    pub fn current_stone(&self) -> Stone {
        self.active_stone
    }

    pub fn next_player(&mut self) {
        self.active_stone = self.active_stone.flip();
    }
}
