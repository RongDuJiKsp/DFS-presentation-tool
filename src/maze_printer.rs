use crate::maze_maker::{Cell, Maze, MazeType};
use crossterm::terminal::ClearType;
use crossterm::{cursor, execute, style, terminal};
use std::io::stdout;
pub struct Printer;
const MARGIN_LEFT: u16 = 3;
const MARGIN_RIGHT: u16 = 3;
const MARGIN_TOP: u16 = 3;
impl Printer {
    pub fn print_to_screen(maze: &MazeType) {
        for (idx, e) in maze.iter().enumerate() {
            execute!(
                stdout(),
                cursor::MoveTo(MARGIN_LEFT + 1, idx as u16 + MARGIN_TOP)
            );
            for bit in e {
                execute!(stdout(), style::PrintStyledContent(bit.block()));
            }
        }
    }
    pub fn print_example(maze: &MazeType) {
        let w = Maze::screen_hw(maze).1 as u16;
        let bd = MARGIN_LEFT + w + MARGIN_RIGHT;
        execute!(stdout(), cursor::MoveTo(bd, MARGIN_TOP));
        execute!(stdout(), style::PrintStyledContent(Cell::Air.block()));
        execute!(stdout(), style::Print("  Air"));
        execute!(stdout(), cursor::MoveTo(bd, MARGIN_TOP + 1));
        execute!(stdout(), style::PrintStyledContent(Cell::Wall.block()));
        execute!(stdout(), style::Print("  Wall"));
        execute!(stdout(), cursor::MoveTo(bd, MARGIN_TOP + 2));
        execute!(stdout(), style::PrintStyledContent(Cell::Vising.block()));
        execute!(stdout(), style::Print("  Visiting"));
        execute!(stdout(), cursor::MoveTo(bd, MARGIN_TOP + 3));
        execute!(stdout(), style::PrintStyledContent(Cell::Vised.block()));
        execute!(stdout(), style::Print("  Visited"));
    }
    pub fn print_status(maze: &MazeType, steps: i32) {
        let w = Maze::screen_hw(maze).1 as u16;
        let bd = MARGIN_LEFT + w + MARGIN_RIGHT + 16;
        let td = bd + 2;
        execute!(stdout(), cursor::MoveTo(bd, MARGIN_TOP));
        execute!(stdout(), style::Print("已走步数"));
        execute!(stdout(), cursor::MoveTo(td, MARGIN_TOP + 1));
        execute!(stdout(), style::Print(steps));
    }
    pub fn cursor_rst() {
        let (w, h) = terminal::size().unwrap();
        execute!(stdout(),cursor::MoveTo(w-1,h-1));
    }
}
