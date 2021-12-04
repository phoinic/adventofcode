static INPUT_DATA: &str = include_str!("input.txt");

#[derive(Debug, Clone)]
struct Cell {
    pub number: u64,
    pub is_marked: bool,
}

#[derive(Debug, Clone)]
struct Board {
    is_win: bool,
    cells: Vec<Vec<Cell>>,
}

impl Board {
    fn new(numbers: Vec<Vec<u64>>) -> Self {
        let mut cells = Vec::<Vec<Cell>>::new();
        for row_numbers in numbers.iter() {
            cells.push(
                row_numbers
                    .iter()
                    .map(|&number| Cell {
                        number,
                        is_marked: false,
                    })
                    .collect::<Vec<_>>(),
            );
        }
        Self {
            is_win: false,
            cells,
        }
    }

    fn mark_number(&mut self, number: u64) {
        for cell in self.cells.iter_mut().flatten() {
            if cell.number == number {
                cell.is_marked = true;
            }
        }
    }

    fn sum_unmarked(&self) -> u64 {
        self.cells
            .iter()
            .flatten()
            .filter(|item| !item.is_marked)
            .map(|item| item.number)
            .sum()
    }

    fn check_win(&self) -> bool {
        self.is_win
    }

    fn play(&mut self) {
        for row in self.cells.iter() {
            if row
                .iter()
                .map(|item| if item.is_marked { 1 } else { 0 })
                .sum::<u8>()
                == 5
            {
                self.is_win = true;
                return;
            }
        }
        for col in 0..self.cells[0].len() {
            if self
                .cells
                .iter()
                .map(|item| if item[col].is_marked { 1 } else { 0 })
                .sum::<u8>()
                == 5
            {
                self.is_win = true;
                return;
            }
        }
    }
}

fn main() {
    let mut drawn_numbers = vec![];
    let mut boards = vec![];
    let mut board_numbers = Vec::<Vec<u64>>::new();

    for (row, str) in INPUT_DATA.split("\n").into_iter().enumerate() {
        match row {
            0 => {
                drawn_numbers = str
                    .split(",")
                    .into_iter()
                    .map(|str| str.parse::<u64>().unwrap())
                    .collect::<Vec<_>>();
            }
            _ => {
                if str != "" {
                    let numbers = str
                        .split_whitespace()
                        .into_iter()
                        .map(|str| str.parse::<u64>().unwrap())
                        .collect::<Vec<_>>();
                    board_numbers.push(numbers);
                    if board_numbers.len() == 5 {
                        boards.push(Board::new(board_numbers));
                        board_numbers = vec![];
                    }
                }
            }
        }
    }

    let mut first_winner = None;
    let mut last_winner = None;

    for number in drawn_numbers {
        for board in boards.iter_mut().filter(|item| !item.check_win()) {
            board.mark_number(number);
            board.play();
            if board.check_win() {
                if first_winner.is_none() {
                    first_winner = Some((board.clone(), number));
                }
                last_winner = Some((board.clone(), number));
            }
        }
    }

    println!(
        "Round 1: {}",
        first_winner.as_ref().unwrap().0.sum_unmarked() * first_winner.unwrap().1
    );

    println!(
        "Round 2: {}",
        last_winner.as_ref().unwrap().0.sum_unmarked() * last_winner.unwrap().1
    );
}
