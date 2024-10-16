use crate::maze_maker::Maze;
use crate::maze_printer::Printer;

mod maze_maker;
mod maze_printer;
mod maze_solver;

fn main() {
    let m = Maze::make(45, 21);
    Printer::print_to_screen(&m);
    Printer::print_example(&m);
    Printer::print_status(&m, 5555);
    Printer::cursor_rst();
}
