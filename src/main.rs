use crate::maze_maker::Maze;
use crate::maze_printer::Printer;
use crate::maze_solver::MazeSolver;

mod maze_maker;
mod maze_printer;
mod maze_solver;

fn main() {
    let m = Maze::make(45, 21);
    MazeSolver::make(m).init().solve(1, 1);
}
