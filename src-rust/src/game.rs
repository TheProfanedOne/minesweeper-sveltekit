use {
    crate::random::random_fields,
    implicit_clone::ImplicitClone,
    js_sys::{Array as JsArray, Uint8Array}
};

pub type Pos = (usize, usize);
pub type BoardSize = [usize; 3];
pub type FieldInfo = (Pos, FieldStatus);

#[derive(Clone, Copy, PartialEq)]
pub enum FieldState {
    Open,
    Closed,
    Flag,
}

#[derive(Clone, Copy, PartialEq)]
pub enum FieldType {
    Empty(u8),
    Mine,
}

#[derive(Clone, Copy, PartialEq)]
pub struct FieldStatus {
    pub fs: FieldState,
    pub ft: FieldType,
}

impl From<(FieldState, FieldType)> for FieldStatus {
    fn from((fs, ft): (FieldState, FieldType)) -> Self {
        Self { fs, ft }
    }
}

#[derive(Clone, PartialEq)]
pub struct Game {
    fields: Vec<Vec<FieldStatus>>,
    finished: Option<bool>,
    lose: bool,
}

impl ImplicitClone for Game {}

impl Game {
    pub fn new([width, height, mine_count]: BoardSize) -> Game {
        Game {
            fields: {
                let mut fields: Vec<Vec<FieldStatus>> = vec![vec![(FieldState::Closed, FieldType::Empty(0)).into(); width]; height];

                random_fields(width, height, mine_count).for_each(|(x, y)| {
                    fields[y][x].ft = FieldType::Mine;
                });

                fields
            },
            finished: None,
            lose: false,
        }.init_empty_fields()
    }

    fn init_empty_fields(&mut self) -> Self {
        let mut ms = self.clone();

        ms.fields.iter_mut().enumerate().for_each(|(y, row)| row.iter_mut().enumerate().for_each(|(x, field)| {
            if field.ft != FieldType::Mine {
                let mines = self.neighboring_mines((x, y));
                field.ft = FieldType::Empty(mines)
            }
        }));

        ms
    }

    pub fn board_state(&self) -> impl Iterator<Item = FieldInfo> {
        self.fields.clone().into_iter().enumerate().flat_map(move |(y, row)| {
            row.into_iter().enumerate().map(move |(x, field)| ((x, y), field))
        })
    }

    pub fn wasm_board_state(&self) -> JsArray {
        let width = self.width() as u32;
        let height = self.height() as u32;

        let board_array = JsArray::new_with_length(height);

        self.fields.clone().into_iter().enumerate().for_each(|(i, row)| {
            let row_array = Uint8Array::new_with_length(width);

            row.into_iter().enumerate().for_each(|(j, FieldStatus { fs, ft })| {
                row_array.set_index(j as u32, match (fs, ft) {
                    (FieldState::Closed, FieldType::Empty(mines)) => mines,
                    (FieldState::Closed, FieldType::Mine)         => 9,
                    
                    (FieldState::Flag, FieldType::Empty(mines))   => 10 + mines,
                    (FieldState::Flag, FieldType::Mine)           => 19,
    
                    (FieldState::Open, FieldType::Empty(mines))   => 20 + mines,
                    (FieldState::Open, FieldType::Mine)           => 29
                });
            });

            board_array.set(i as u32, row_array.into());
        });

        board_array
    }

    pub fn width(&self) -> usize {
        self.fields[0].len()
    }

    pub fn height(&self) -> usize {
        self.fields.len()
    }

    pub fn mine_count(&self) -> usize {
        self.board_state().filter(|(pos, _)| self.is_mine(*pos)).count()
    }

    pub fn fields_opened(&self) -> usize {
        self.board_state().filter(|(_, status)| matches!((*status).fs, FieldState::Open)).count()
    }

    fn fields_flagged(&self) -> usize {
        self.board_state().filter(|(_, status)| matches!((*status).fs, FieldState::Flag)).count()
    }

    pub fn flags_remaining(&self) -> usize {
        self.mine_count() - self.fields_flagged()
    }

    pub fn board_reset(&mut self) {
        let mine_count = self.mine_count();

        self.fields.iter_mut().for_each(|row| row.iter_mut().for_each(|field| {
            *field = (FieldState::Closed, FieldType::Empty(0)).into()
        }));

        random_fields(self.fields[0].len(), self.fields.len(), mine_count).for_each(|(x, y)| {
            self.fields[y][x].ft = FieldType::Mine
        });

        *self = self.init_empty_fields();
    }

    // pub fn false_alarm(&mut self) {
    //     self.finished.take();
    // }

    pub fn lost(&mut self) {
        self.finished.replace(true);
    }

    pub fn is_finished(&self) -> Option<bool> {
        self.finished
    }

    // fn lose_state((_, s): FieldInfo) -> bool {
    //     s.fs == FieldState::Open && s.ft == FieldType::Mine
    // }

    pub fn show_loss(&mut self) {
        self.fields.iter_mut().for_each(|r| r.iter_mut().filter(|s| !matches!(s.fs, FieldState::Open)).for_each(|f| {
            f.fs = FieldState::Open
        }))
    }

    pub fn win_check(&mut self) {
        let won = self.win_state();
        if won && matches!(self.finished, Some(true) | None) {
            self.finished.replace(false);
        }
    }

    fn win_state(&self) -> bool {
        if self.flags_remaining() == 0 {
            if self.fields_opened() + self.fields_flagged() == self.board_state().count() {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn is_open(&self, (x, y): Pos) -> bool {
        self.fields[y][x].fs == FieldState::Open
    }

    pub fn is_flagged(&self, (x, y): Pos) -> bool {
        self.fields[y][x].fs == FieldState::Flag
    }

    fn is_mine(&self, (x, y): Pos) -> bool {
        self.fields[y][x].ft == FieldType::Mine
    }

    fn iter_neighbors(&self, (x, y): Pos) -> impl Iterator<Item = Pos> {
        let width = self.fields[0].len();
        let height = self.fields.len();

        (x.max(1) - 1..=(x + 1).min(width - 1))
            .flat_map(move |i| (y.max(1) - 1..=(y + 1).min(height - 1)).map(move |j| (i, j)))
            .filter(move |&pos| pos != (x, y))
    }

    pub fn neighboring_mines(&self, pos: Pos) -> u8 {
        self.iter_neighbors(pos).filter(move |&p| self.is_mine(p)).count() as u8
    }

    pub fn open(&mut self, pos: Pos) -> bool {
        if self.is_open(pos) {
            let mine_count = self.neighboring_mines(pos);
            let flag_count = self
                .iter_neighbors(pos)
                .filter(|&neighbor| self.is_flagged(neighbor))
                .count() as u8;
      
            if mine_count == flag_count {
                for neighbor in self.iter_neighbors(pos) {
                    if !self.is_flagged(neighbor)
                       && !self.is_open(neighbor)
                    {
                        if self.open(neighbor) {
                            return true;
                        }
                    }
                }
            }
      
            return false;
        }
    
        self.fields[pos.1][pos.0].fs = FieldState::Open;

        if self.is_mine(pos) {
            true
        } else {
            let mines = self.neighboring_mines(pos);

            if mines == 0 {
                self.iter_neighbors(pos).for_each(|neighbor| {
                    self.open(neighbor);
                });
            }

            false
        }
    }

    pub fn toggle_flag(&mut self, pos: Pos) {
        self.fields[pos.1][pos.0].fs = if self.is_flagged(pos) {
            FieldState::Closed
        } else {
            FieldState::Flag
        }
    }
}
