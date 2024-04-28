use tui::{style::{Color, Style},
 widgets::{Block, BorderType, Borders, Paragraph}
};

pub type Coordinate = (usize, usize);
pub type Board = [[Cell; 9]; 9];
pub type Movement = (String, String);



pub struct App {
    pub title: String,
    pub board: Board,
    pub command: String,
  }
  
  impl App {
    pub fn new(title: String) -> Self {
      App {
        title,
        board: create_board(),
        command: "".into(),
      }
    }

    pub fn cell(self: &Self, c: Coordinate) -> &Cell {
        &self.board[c.0][c.1]
    }

    pub fn execute_command(self: &mut Self) {
        self.board[7][1].set_selected()
    }

    pub fn draw_command(&self) {
        
    }

    fn find_cell(self :&Self) {
        for (u, cell_row) in (&self.board).into_iter().enumerate() {
            for (r, cell) in cell_row.clone().into_iter().enumerate() {
                
            }
        }
    }
}

pub struct Cell {
    pub piece: String,
    pub selected: bool,
  }


impl Cell {
    fn new(piece: String, selected: bool) -> Self {
        Self {
        piece,
        selected,
        }
    }

    pub fn text_style(&self) -> Style {
        // cell background color
        Style::default().bg(Color::Black).fg(
            if self.is_selected() {
                Color::Yellow
            } else {
                Color::Cyan
            }
        )
    }

    pub fn get_piece(&self) -> String {
        self.piece.to_string()
    }

    pub fn set_selected(& mut self) {
        self.selected = !self.selected;
    }

    fn is_selected(&self) -> bool {
        self.selected
    }

    pub fn display(&self, text: String) -> Paragraph {
        return Paragraph::new(text)
        .block(Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain))
        .style(self.text_style());
    }

    pub fn display_without_border(&self, text: String) -> Paragraph {
        return Paragraph::new(text)
        .block(Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain))
        .style(Style::default().bg(Color::Black).fg(Color::Black));
    }
}

fn create_board() -> Board {
    let board: Board = [
        [Cell::new("8".to_string(), false),Cell::new("♜".to_string(), false), Cell::new("♞".to_string(), false), Cell::new("♝".to_string(), false), Cell::new("♛".to_string(), false), Cell::new("♚".to_string(), false), Cell::new("♝".to_string(), false), Cell::new("♞".to_string(), false), Cell::new("♜".to_string(), false)],
        [Cell::new("7".to_string(), false),Cell::new("♟".to_string(), false), Cell::new("♟".to_string(), false), Cell::new("♟".to_string(), false), Cell::new("♟".to_string(), false), Cell::new("♟".to_string(), false), Cell::new("♟".to_string(), false), Cell::new("♟".to_string(), false), Cell::new("♟".to_string(), false)],
        [Cell::new("6".to_string(), false),Cell::new(" ".to_string(), false), Cell::new(" ".to_string(), false), Cell::new(" ".to_string(), false), Cell::new(" ".to_string(), false), Cell::new(" ".to_string(), false), Cell::new(" ".to_string(), false), Cell::new(" ".to_string(), false), Cell::new(" ".to_string(), false)],
        [Cell::new("5".to_string(), false),Cell::new(" ".to_string(), false), Cell::new(" ".to_string(), false), Cell::new(" ".to_string(), false), Cell::new(" ".to_string(), false), Cell::new(" ".to_string(), false), Cell::new(" ".to_string(), false), Cell::new(" ".to_string(), false), Cell::new(" ".to_string(), false)],
        [Cell::new("4".to_string(), false),Cell::new(" ".to_string(), false), Cell::new(" ".to_string(), false), Cell::new(" ".to_string(), false), Cell::new(" ".to_string(), false), Cell::new(" ".to_string(), false), Cell::new(" ".to_string(), false), Cell::new(" ".to_string(), false), Cell::new(" ".to_string(), false)],
        [Cell::new("3".to_string(), false),Cell::new(" ".to_string(), false), Cell::new(" ".to_string(), false), Cell::new(" ".to_string(), false), Cell::new(" ".to_string(), false), Cell::new(" ".to_string(), false), Cell::new(" ".to_string(), false), Cell::new(" ".to_string(), false), Cell::new(" ".to_string(), false)],
        [Cell::new("2".to_string(), false),Cell::new("♙".to_string(), false), Cell::new("♙".to_string(), false), Cell::new("♙".to_string(), false), Cell::new("♙".to_string(), false), Cell::new("♙".to_string(), false), Cell::new("♙".to_string(), false), Cell::new("♙".to_string(), false), Cell::new("♙".to_string(), false)],
        [Cell::new("1".to_string(), false),Cell::new("♖".to_string(), false), Cell::new("♘".to_string(), false), Cell::new("♗".to_string(), false), Cell::new("♕".to_string(), false), Cell::new("♔".to_string(), false), Cell::new("♗".to_string(), false), Cell::new("♘".to_string(), false), Cell::new("♖".to_string(), false)],
        [Cell::new(" ".to_string(), false),Cell::new("A".to_string(), false), Cell::new("B".to_string(), false), Cell::new("C".to_string(), false), Cell::new("D".to_string(), false), Cell::new("E".to_string(), false), Cell::new("F".to_string(), false), Cell::new("G".to_string(), false), Cell::new("H".to_string(), false)],
    ];
    return board;
}
