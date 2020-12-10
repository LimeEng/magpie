use crate::agent::Action;
use crate::agent::Agent;
use crate::coord::Coord;
use agent::human_agent::HumanAgent;
use agent::random_agent::RandomAgent;
use magpie::othello_board::OthelloBoard;
use magpie::stone::Stone;

pub mod agent;
pub mod coord;
pub mod util;

fn main() {
    play();
}

fn play() {
    let mut agent1 = HumanAgent;
    let mut agent2 = RandomAgent;
    let mut active_agent = ActiveAgent::AgentOne;

    let mut board = OthelloBoard::standard();
    let mut passed_last_turn = false;
    while board.empty_cells().count_ones() > 0 {
        let repr = util::display(&board, Some(active_agent.stone()));
        println!("{}", repr);
        let legal_moves = board.legal_moves_for(active_agent.stone());
        if legal_moves == 0 {
            println!("{:?} have no moves to make", active_agent.stone());
            if passed_last_turn {
                println!("No player have any legal moves to make, exiting");
                break;
            }
            passed_last_turn = true;
        } else {
            let chosen_move = match active_agent {
                ActiveAgent::AgentOne => agent1.play(active_agent.stone(), &board),
                ActiveAgent::AgentTwo => agent2.play(active_agent.stone(), &board),
            };
            passed_last_turn = false;
            match chosen_move {
                Action::Move(ply) => {
                    board
                        .place_stone(active_agent.stone(), ply)
                        .unwrap_or_else(|_| {
                            panic!("{:?} tried to make an illegal move", active_agent.stone())
                        });
                    let played_pos = Coord::from_bitboard(ply).unwrap().as_notation();
                    println!("{:?} played {}", active_agent.stone(), played_pos);
                }
                Action::Pass => {
                    passed_last_turn = true;
                    println!("{:?} passed their turn", active_agent.stone());
                }
            }
        }
        active_agent = active_agent.next_agent();
    }
    println!("Final board");
    let repr = util::display(&board, None);
    println!("{}", repr);
    let black = board.bits_for(Stone::Black).count_ones();
    let white = board.bits_for(Stone::White).count_ones();
    println!("Game finished with {} - {} (black - white)", black, white);
}

#[derive(Clone, Copy, Debug)]
enum ActiveAgent {
    AgentOne,
    AgentTwo,
}

impl ActiveAgent {
    pub fn next_agent(&self) -> ActiveAgent {
        use ActiveAgent::*;
        match &self {
            AgentOne => AgentTwo,
            AgentTwo => AgentOne,
        }
    }

    pub fn stone(&self) -> Stone {
        use ActiveAgent::*;
        match &self {
            AgentOne => Stone::Black,
            AgentTwo => Stone::White,
        }
    }
}
