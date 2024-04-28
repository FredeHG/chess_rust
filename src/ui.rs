use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
  };


use super::app;

const CELL_WIDTH: u16 = 8;
const CELL_HEIGHT: u16 = 4;
const GRID_WIDTH: u16 = CELL_WIDTH * (9 as u16);
const GRID_HEIGHT: u16 = CELL_HEIGHT * (9 as u16);

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut app::App) {
  let main_block = Block::default()
    .borders(Borders::ALL)
    .style(Style::default().bg(Color::Black).fg(Color::Cyan))
    .title(format!(
      "{}",
      app.title,
    ));

  f.render_widget(main_block, f.size());

  let vertical_pad_block_height = f.size().height.checked_sub(GRID_HEIGHT).unwrap_or_default() / 2;
  let v_chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
      Constraint::Min(vertical_pad_block_height),
      Constraint::Length(GRID_HEIGHT + 1),
      Constraint::Min(vertical_pad_block_height),
    ])
    .split(f.size());

  let header = Paragraph::new(
    app.command.clone(),
  )
  .style(Style::default().fg(Color::Gray))
  .block(Block::default().borders(Borders::NONE))
  .alignment(Alignment::Center);

  f.render_widget(header, v_chunks[2]);

  let board_chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Percentage(50)].as_ref())
    .split(v_chunks[1]);

  let player_chunk = board_chunks[0];

  draw_board(f, player_chunk, "", app);
}

fn draw_board<B: Backend>(
  f: &mut Frame<B>,
  player_chunk: Rect,
  title: &str,
  app: &mut app::App,
) {
  let row_constraints = std::iter::repeat(Constraint::Length(CELL_HEIGHT))
    .take(9)
    .collect::<Vec<_>>();
  let col_constraints = std::iter::repeat(Constraint::Length(CELL_WIDTH))
    .take(9)
    .collect::<Vec<_>>();

  let horizontal_pad_block_width = (player_chunk.width - GRID_WIDTH) / 2;
  let h_main_rects: Vec<Rect> = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(vec![
      Constraint::Min(horizontal_pad_block_width),
      Constraint::Length(GRID_WIDTH),
      Constraint::Min(horizontal_pad_block_width),
    ])
    .split(player_chunk);

  let v_main_rects = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![Constraint::Min(1), Constraint::Length(GRID_HEIGHT)])
    .split(h_main_rects[1]);

  let title = Paragraph::new(title)
    .style(
      Style::default()
        .fg(Color::Green)
        .add_modifier(Modifier::BOLD),
    )
    .block(Block::default().borders(Borders::NONE))
    .alignment(Alignment::Center);

  f.render_widget(title, v_main_rects[0]);

  let board_block = Block::default()
    .borders(Borders::ALL)
    .border_type(BorderType::Plain);

  let board_rect = v_main_rects[1];
  f.render_widget(board_block, board_rect);

  let row_rects = Layout::default()
    .direction(Direction::Vertical)
    .constraints(row_constraints)
    .split(board_rect);

  for (r, row_rect) in row_rects.into_iter().enumerate() {
    let col_rects = Layout::default()
      .direction(Direction::Horizontal)
      .constraints(col_constraints.clone())
      .split(row_rect);

    for (c, cell_rect) in col_rects.into_iter().enumerate() {
      let cell = app.cell((r, c));
      let single_row_text = format!(
        "{:^length$}",
        cell.get_piece(),
        length = usize::from(CELL_WIDTH - 2)
      );
      let pad_line = " ".repeat(usize::from(CELL_WIDTH));

      let num_pad_lines = usize::from(CELL_HEIGHT.checked_sub(3).unwrap_or_default());

      let text: String = std::iter::repeat(pad_line.clone())
        .take(num_pad_lines / 2)
        .chain(std::iter::once(single_row_text.clone()))
        .chain(std::iter::repeat(pad_line).take(num_pad_lines / 2))
        .collect::<Vec<_>>()
        .join("\n");
      if c == 0 || r == 8 {
        f.render_widget(cell.display_without_border(text), cell_rect);
      } else {
        f.render_widget(cell.display(text), cell_rect);
      }
    }
  }
}