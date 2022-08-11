use crate::{
    agent::{Action, Agent, HumanAgent, RandomAgent},
    coord::Coord,
};
use magpie::othello::{Board, Stone};

pub mod agent;
pub mod coord;

fn main() {
    play();
}

fn play() {
    let mut agent1 = HumanAgent;
    let mut agent2 = RandomAgent;
    let mut active_agent = ActiveAgent::AgentOne;

    let mut board = Board::standard();
    let mut passed_last_turn = false;
    while board.empty_squares().count_ones() > 0 {
        println!("{}", board.display().with_stone(active_agent.stone()));
        let legal_moves = board.moves_for(active_agent.stone());
        if legal_moves == 0 {
            println!("{:?} have no moves to make", active_agent.stone());
            if passed_last_turn {
                // In Othello, if both players passed their turn in succession,
                // the game is over.
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
                    // It is safe to unwrap since the previous "place_stone"
                    // succeeded.
                    let played_pos = Coord::try_from(ply).unwrap().as_notation();
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
    println!("{}", board.display());
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
    pub fn next_agent(self) -> ActiveAgent {
        match self {
            ActiveAgent::AgentOne => ActiveAgent::AgentTwo,
            ActiveAgent::AgentTwo => ActiveAgent::AgentOne,
        }
    }

    pub fn stone(self) -> Stone {
        match self {
            ActiveAgent::AgentOne => Stone::Black,
            ActiveAgent::AgentTwo => Stone::White,
        }
    }
}
