// i'm writing tests currently
#![allow(dead_code)]

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    pub fn from_index(i: usize) -> Option<Self> {
        match i {
            0 => Some(File::A),
            1 => Some(File::B),
            2 => Some(File::C),
            3 => Some(File::D),
            4 => Some(File::E),
            5 => Some(File::F),
            6 => Some(File::G),
            7 => Some(File::H),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl Rank {
    pub fn from_index(i: usize) -> Option<Self> {
        match i {
            0 => Some(Rank::One),
            1 => Some(Rank::Two),
            2 => Some(Rank::Three),
            3 => Some(Rank::Four),
            4 => Some(Rank::Five),
            5 => Some(Rank::Six),
            6 => Some(Rank::Seven),
            7 => Some(Rank::Eight),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub file: File,
    pub rank: Rank,
}

impl Position {
    pub fn new(file: File, rank: Rank) -> Self {
        Self { file, rank }
    }

    pub fn add_file<T>(&self, n: T) -> Option<Self>
    where
        T: Into<i64>,
    {
        let f = self.file as i64;
        File::from_index((n.into() + f) as usize).map(|file| Self::new(file, self.rank))
    }

    pub fn add_rank<T>(&self, n: T) -> Option<Self>
    where
        T: Into<i64>,
    {
        let r = self.rank as i64;
        Rank::from_index((n.into() + r) as usize).map(|rank| Self::new(self.file, rank))
    }

    pub fn add_file_and_rank<T>(&self, file: T, rank: T) -> Option<Self>
    where
        T: Into<i64>,
    {
        self.add_file(file)?.add_rank(rank)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChessPieceMove {
    pub from: Position,
    pub to: Position,
    pub piece: ChessPiece,
}

pub struct ChessBoard {
    cells: [[Option<ChessPiece>; 8]; 8],
}

impl ChessBoard {
    pub fn new() -> Self {
        let empty_rank = [None; 8];
        Self {
            cells: [
                [
                    Some(ChessPiece::new(PieceType::Rook, PieceColor::White)),
                    Some(ChessPiece::new(PieceType::Knight, PieceColor::White)),
                    Some(ChessPiece::new(PieceType::Bishop, PieceColor::White)),
                    Some(ChessPiece::new(PieceType::Queen, PieceColor::White)),
                    Some(ChessPiece::new(PieceType::King, PieceColor::White)),
                    Some(ChessPiece::new(PieceType::Bishop, PieceColor::White)),
                    Some(ChessPiece::new(PieceType::Knight, PieceColor::White)),
                    Some(ChessPiece::new(PieceType::Rook, PieceColor::White)),
                ],
                [
                    Some(ChessPiece::new(PieceType::Pawn, PieceColor::White)),
                    Some(ChessPiece::new(PieceType::Pawn, PieceColor::White)),
                    Some(ChessPiece::new(PieceType::Pawn, PieceColor::White)),
                    Some(ChessPiece::new(PieceType::Pawn, PieceColor::White)),
                    Some(ChessPiece::new(PieceType::Pawn, PieceColor::White)),
                    Some(ChessPiece::new(PieceType::Pawn, PieceColor::White)),
                    Some(ChessPiece::new(PieceType::Pawn, PieceColor::White)),
                    Some(ChessPiece::new(PieceType::Pawn, PieceColor::White)),
                ],
                empty_rank,
                empty_rank,
                empty_rank,
                empty_rank,
                [
                    Some(ChessPiece::new(PieceType::Pawn, PieceColor::Black)),
                    Some(ChessPiece::new(PieceType::Pawn, PieceColor::Black)),
                    Some(ChessPiece::new(PieceType::Pawn, PieceColor::Black)),
                    Some(ChessPiece::new(PieceType::Pawn, PieceColor::Black)),
                    Some(ChessPiece::new(PieceType::Pawn, PieceColor::Black)),
                    Some(ChessPiece::new(PieceType::Pawn, PieceColor::Black)),
                    Some(ChessPiece::new(PieceType::Pawn, PieceColor::Black)),
                    Some(ChessPiece::new(PieceType::Pawn, PieceColor::Black)),
                ],
                [
                    Some(ChessPiece::new(PieceType::Rook, PieceColor::Black)),
                    Some(ChessPiece::new(PieceType::Knight, PieceColor::Black)),
                    Some(ChessPiece::new(PieceType::Bishop, PieceColor::Black)),
                    Some(ChessPiece::new(PieceType::Queen, PieceColor::Black)),
                    Some(ChessPiece::new(PieceType::King, PieceColor::Black)),
                    Some(ChessPiece::new(PieceType::Bishop, PieceColor::Black)),
                    Some(ChessPiece::new(PieceType::Knight, PieceColor::Black)),
                    Some(ChessPiece::new(PieceType::Rook, PieceColor::Black)),
                ],
            ],
        }
    }

    pub fn from_inverted_array(cells: [[Option<ChessPiece>; 8]; 8]) -> Self {
        Self {
            cells: [
                cells[7], cells[6], cells[5], cells[4], cells[3], cells[2], cells[1], cells[0],
            ],
        }
    }

    pub fn at(&self, position: &Position) -> Option<ChessPiece> {
        self.cells[position.rank as usize][position.file as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChessPiece {
    pub piece_type: PieceType,
    pub color: PieceColor,
}

impl ChessPiece {
    pub const fn new(piece_type: PieceType, color: PieceColor) -> Self {
        Self { piece_type, color }
    }

    fn pawn_moves(
        &self,
        position: Position,
        board: &ChessBoard,
        previous_move: Option<ChessPieceMove>,
    ) -> Vec<Position> {
        let mut potential_moves = vec![];
        let direction = if self.color == PieceColor::White {
            1
        } else {
            -1
        };

        let forward = position.add_rank(direction);
        if forward.is_none() {
            return vec![];
        }

        let can_move_forward = board.at(&forward.unwrap()).is_none();

        if can_move_forward {
            if let Some(potential_position) = position.add_rank(direction) {
                potential_moves.push(potential_position);
            }
            if self.color == PieceColor::White && position.rank == Rank::Two
                || self.color == PieceColor::Black && position.rank == Rank::Seven
            {
                if let Some(potential_position) = position.add_rank(2 * direction) {
                    potential_moves.push(potential_position);
                }
            }
        }

        let mut potential_moves = potential_moves
            .into_iter()
            .filter(|p| board.at(p).is_none())
            .collect::<Vec<_>>();

        let mut potential_captures = vec![];

        let diagonals = [
            position.add_file_and_rank(1, direction),
            position.add_file_and_rank(-1, direction),
        ];

        for diagonal in diagonals.into_iter().flatten() {
            if let Some(piece) = board.at(&diagonal) {
                if piece.color != self.color {
                    potential_captures.push(diagonal);
                }
            }

            // en passant
            if let Some(previous_move) = previous_move {
                if previous_move.piece.piece_type != PieceType::Pawn {
                    continue;
                }
                if Some(previous_move.from) == diagonal.add_rank(direction)
                    && previous_move.to.rank == position.rank
                {
                    potential_captures.push(diagonal);
                }
            }
        }
        potential_moves.append(&mut potential_captures);
        potential_moves
    }

    fn rook_moves(&self, position: Position, board: &ChessBoard) -> Vec<Position> {
        let mut potential_moves = vec![];

        // Iterate through both the file and rank directions.
        for direction in ["file", "rank"].iter() {
            for direction_value in [-1, 1].iter() {
                self.collect_potential_moves(
                    direction,
                    *direction_value,
                    &position,
                    board,
                    &mut potential_moves,
                );
            }
        }

        potential_moves
    }

    // Helper method to collect potential moves in a specific direction
    fn collect_potential_moves(
        &self,
        direction: &str,
        direction_value: i32,
        position: &Position,
        board: &ChessBoard,
        potential_moves: &mut Vec<Position>,
    ) {
        let mut i = 1;

        // Traverse until the edge of the board or until a piece is encountered.
        while let Some(potential_position) = match direction {
            "file" => position.add_file(direction_value * i),
            "rank" => position.add_rank(direction_value * i),
            _ => None,
        } {
            match board.at(&potential_position) {
                Some(piece) if piece.color != self.color => {
                    // If the piece at the potential position is of different color, add the position to potential moves.
                    potential_moves.push(potential_position);
                    break;
                }
                Some(_) => {
                    // If the piece is of same color, stop traversing in this direction.
                    break;
                }
                None => {
                    // If there's no piece at the potential position, add it to potential moves.
                    potential_moves.push(potential_position);
                }
            }
            i += 1;
        }
    }
}

pub trait Moves {
    fn possible_moves(
        &self,
        position: Position,
        board: &ChessBoard,
        previous_move: Option<ChessPieceMove>,
    ) -> Vec<Position>;
}

impl Moves for ChessPiece {
    fn possible_moves(
        &self,
        position: Position,
        board: &ChessBoard,
        previous_move: Option<ChessPieceMove>,
    ) -> Vec<Position> {
        match self.piece_type {
            PieceType::Pawn => self.pawn_moves(position, board, previous_move),
            PieceType::Rook => self.rook_moves(position, board),
            _ => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use File::*;
    use PieceColor::*;
    use PieceType::*;
    use Rank::*;

    const W_PN: Option<ChessPiece> = Some(ChessPiece::new(Pawn, White));
    const W_RK: Option<ChessPiece> = Some(ChessPiece::new(Rook, White));
    const W_KT: Option<ChessPiece> = Some(ChessPiece::new(Knight, White));
    const W_BP: Option<ChessPiece> = Some(ChessPiece::new(Bishop, White));
    const W_QN: Option<ChessPiece> = Some(ChessPiece::new(Queen, White));
    const W_KG: Option<ChessPiece> = Some(ChessPiece::new(King, White));

    const B_PN: Option<ChessPiece> = Some(ChessPiece::new(Pawn, Black));
    const B_RK: Option<ChessPiece> = Some(ChessPiece::new(Rook, Black));
    const B_KT: Option<ChessPiece> = Some(ChessPiece::new(Knight, Black));
    const B_BP: Option<ChessPiece> = Some(ChessPiece::new(Bishop, Black));
    const B_QN: Option<ChessPiece> = Some(ChessPiece::new(Queen, Black));
    const B_KG: Option<ChessPiece> = Some(ChessPiece::new(King, Black));

    #[test]
    fn test_position_arithmetic() {
        let position = Position::new(A, One);
        let mut new_position = position.add_file(1);
        assert_eq!(new_position, Some(Position::new(B, One)));

        new_position = position.add_file(-1);
        assert_eq!(new_position, None);

        new_position = position.add_rank(1);
        assert_eq!(new_position, Some(Position::new(A, Two)));

        new_position = position.add_rank(-1);
        assert_eq!(new_position, None);

        new_position = position.add_file_and_rank(1, 1);
        assert_eq!(new_position, Some(Position::new(B, Two)));
    }

    #[test]
    fn test_new_chess_board() {
        let board = ChessBoard::new();

        let position = Position::new(A, One);
        let white_rook = board.at(&position).unwrap();
        assert_eq!(white_rook.piece_type, Rook);
        assert_eq!(white_rook.color, White);

        let position = Position::new(E, Two);
        let white_pawn = board.at(&position).unwrap();
        assert_eq!(white_pawn.piece_type, Pawn);
        assert_eq!(white_pawn.color, White);

        let position = Position::new(E, One);
        let white_king = board.at(&position).unwrap();
        assert_eq!(white_king.piece_type, King);
        assert_eq!(white_king.color, White);

        let position = Position::new(B, Eight);
        let black_knight = board.at(&position).unwrap();
        assert_eq!(black_knight.piece_type, Knight);
        assert_eq!(black_knight.color, Black);

        let position = Position::new(A, Four);
        let space = board.at(&position);
        assert_eq!(space, None);
    }

    #[test]
    fn test_custom_chess_board() {
        let board = ChessBoard::from_inverted_array([
            [B_RK, B_KT, B_BP, B_QN, B_KG, B_BP, B_KT, B_RK],
            [B_PN, B_PN, B_PN, B_PN, B_PN, B_PN, B_PN, B_PN],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [W_PN, W_PN, W_PN, W_PN, W_PN, W_PN, W_PN, W_PN],
            [W_RK, W_KT, W_BP, W_QN, W_KG, W_BP, W_KT, W_RK],
        ]);

        let position = Position::new(A, One);
        let white_rook = board.at(&position).unwrap();
        assert_eq!(white_rook.piece_type, Rook);
        assert_eq!(white_rook.color, White);

        let position = Position::new(B, One);
        let white_knight = board.at(&position).unwrap();
        assert_eq!(white_knight.piece_type, Knight);
        assert_eq!(white_knight.color, White);

        let position = Position::new(C, One);
        let white_bishop = board.at(&position).unwrap();
        assert_eq!(white_bishop.piece_type, Bishop);
        assert_eq!(white_bishop.color, White);

        let position = Position::new(D, One);
        let white_queen = board.at(&position).unwrap();
        assert_eq!(white_queen.piece_type, Queen);
        assert_eq!(white_queen.color, White);

        let position = Position::new(E, One);
        let white_king = board.at(&position).unwrap();
        assert_eq!(white_king.piece_type, King);
        assert_eq!(white_king.color, White);

        let position = Position::new(A, Two);
        let white_pawn = board.at(&position).unwrap();
        assert_eq!(white_pawn.piece_type, Pawn);
        assert_eq!(white_pawn.color, White);

        let position = Position::new(H, Seven);
        let black_pawn = board.at(&position).unwrap();
        assert_eq!(black_pawn.piece_type, Pawn);
        assert_eq!(black_pawn.color, Black);

        let position = Position::new(A, Four);
        let space = board.at(&position);
        assert_eq!(space, None);
    }

    #[test]
    fn test_pawn_moves() {
        let board = ChessBoard::from_inverted_array([
            [B_RK, None, B_BP, None, B_KG, B_BP, None, B_RK],
            [B_PN, None, B_PN, None, B_PN, None, B_PN, B_PN],
            [None, None, None, None, None, None, None, None],
            [W_PN, B_PN, W_PN, B_PN, B_QN, B_PN, None, None],
            [None, None, None, None, W_PN, None, None, None],
            [None, B_KT, None, None, None, None, B_KT, W_PN],
            [None, W_PN, W_PN, W_PN, None, W_PN, W_PN, None],
            [W_RK, W_KT, W_BP, W_QN, W_KG, W_BP, W_KT, W_RK],
        ]);

        // white pawn on second rank can move one or two squares up
        let position = Position::new(D, Two);
        let white_pawn = board.at(&position).unwrap();
        let moves = white_pawn.possible_moves(position, &board, None);
        println!("{:?}", moves);
        assert_eq!(moves.len(), 2);
        assert!(moves.contains(&Position::new(D, Three)));
        assert!(moves.contains(&Position::new(D, Four)));

        // black pawn on seventh rank can move one or two squares down
        let position = Position::new(H, Seven);
        let black_pawn = board.at(&position).unwrap();
        let moves = black_pawn.possible_moves(position, &board, None);
        println!("{:?}", moves);
        assert_eq!(moves.len(), 2);
        assert!(moves.contains(&Position::new(H, Six)));
        assert!(moves.contains(&Position::new(H, Five)));

        // white pawn on second rank cannot move when blocked by a piece
        let position = Position::new(B, Two);
        let white_pawn = board.at(&position).unwrap();
        let moves = white_pawn.possible_moves(position, &board, None);
        println!("{:?}", moves);
        assert_eq!(moves.len(), 0);

        // white pawn can attack diagonally, but cannot move forward when blocked by a piece
        let position = Position::new(E, Four);
        let white_pawn = board.at(&position).unwrap();
        let moves = white_pawn.possible_moves(position, &board, None);
        println!("{:?}", moves);
        assert_eq!(moves.len(), 2);
        assert!(moves.contains(&Position::new(D, Five)));
        assert!(moves.contains(&Position::new(F, Five)));

        // white pawn on second rank cannot move and attack diagonally
        let position = Position::new(C, Two);
        let white_pawn = board.at(&position).unwrap();
        let moves = white_pawn.possible_moves(position, &board, None);
        println!("{:?}", moves);
        assert_eq!(moves.len(), 3);
        assert!(moves.contains(&Position::new(B, Three)));
        assert!(moves.contains(&Position::new(C, Three)));
        assert!(moves.contains(&Position::new(C, Four)));

        // black pawn on seventh rank can move only one square forward when blocked by a piece
        let position = Position::new(E, Seven);
        let black_pawn = board.at(&position).unwrap();
        let moves = black_pawn.possible_moves(position, &board, None);
        println!("{:?}", moves);
        assert_eq!(moves.len(), 1);
        assert!(moves.contains(&Position::new(E, Six)));

        // black pawn on fifth rank can move and attack diagonally
        let position = Position::new(D, Five);
        let black_pawn = board.at(&position).unwrap();
        let moves = black_pawn.possible_moves(position, &board, None);
        println!("{:?}", moves);
        assert_eq!(moves.len(), 2);
        assert!(moves.contains(&Position::new(D, Four)));
        assert!(moves.contains(&Position::new(E, Four)));

        // white pawn on third rank can move one square forward
        let position = Position::new(H, Three);
        let white_pawn = board.at(&position).unwrap();
        let moves = white_pawn.possible_moves(position, &board, None);
        println!("{:?}", moves);
        assert_eq!(moves.len(), 1);
        assert!(moves.contains(&Position::new(H, Four)));

        // black pawn on seventh rank can move only one square forward when blocked by a white pawn
        let position = Position::new(A, Seven);
        let black_pawn = board.at(&position).unwrap();
        let moves = black_pawn.possible_moves(position, &board, None);
        println!("{:?}", moves);
        assert_eq!(moves.len(), 1);
        assert!(moves.contains(&Position::new(A, Six)));

        // white pawn on fifth rank can capture en passant or move forward
        let position = Position::new(A, Five);
        let white_pawn = board.at(&position).unwrap();
        let previous_move = ChessPieceMove {
            from: Position::new(B, Seven),
            to: Position::new(B, Five),
            piece: B_PN.unwrap(),
        };
        let moves = white_pawn.possible_moves(position, &board, Some(previous_move));
        println!("{:?}", moves);
        assert_eq!(moves.len(), 2);
        assert!(moves.contains(&Position::new(A, Six)));
        assert!(moves.contains(&Position::new(B, Six)));

        // white pawn cannot capture en passant if the previous move was not the pawn
        let position = Position::new(A, Five);
        let white_pawn = board.at(&position).unwrap();
        let previous_move = ChessPieceMove {
            from: Position::new(H, Six),
            to: Position::new(G, Four),
            piece: B_KT.unwrap(),
        };
        let moves = white_pawn.possible_moves(position, &board, Some(previous_move));
        println!("{:?}", moves);
        assert_eq!(moves.len(), 1);
        assert!(moves.contains(&Position::new(A, Six)));

        // another postion, to test en passant capture better
        let board = ChessBoard::from_inverted_array([
            [B_RK, None, B_BP, None, B_KG, B_BP, None, B_RK],
            [B_PN, None, B_PN, None, B_PN, None, B_PN, B_PN],
            [None, None, None, None, None, None, None, None],
            [W_PN, B_PN, W_PN, B_PN, B_QN, B_PN, None, None],
            [None, None, W_PN, B_PN, W_PN, None, None, None],
            [None, B_KT, None, W_PN, None, None, B_KT, W_PN],
            [None, W_PN, None, W_PN, None, W_PN, W_PN, None],
            [W_RK, W_KT, W_BP, W_QN, W_KG, W_BP, W_KT, W_RK],
        ]);

        // black pawn can capture en passant
        let position = Position::new(D, Four);
        let black_pawn = board.at(&position).unwrap();

        // on C
        let previous_move = ChessPieceMove {
            from: Position::new(C, Two),
            to: Position::new(C, Four),
            piece: W_PN.unwrap(),
        };
        let moves = black_pawn.possible_moves(position, &board, Some(previous_move));
        println!("{:?}", moves);
        assert_eq!(moves.len(), 1);
        assert!(moves.contains(&Position::new(C, Three)));

        // on E
        let previous_move = ChessPieceMove {
            from: Position::new(E, Two),
            to: Position::new(E, Four),
            piece: W_PN.unwrap(),
        };
        let moves = black_pawn.possible_moves(position, &board, Some(previous_move));
        println!("{:?}", moves);
        assert_eq!(moves.len(), 1);
        assert!(moves.contains(&Position::new(E, Three)));

        // or cannot if the moment has passed
        let previous_move = ChessPieceMove {
            from: Position::new(A, Four),
            to: Position::new(A, Five),
            piece: W_PN.unwrap(),
        };
        let moves = black_pawn.possible_moves(position, &board, Some(previous_move));
        println!("{:?}", moves);
        assert_eq!(moves.len(), 0);
    }

    #[test]
    fn test_regular_rook_moves() {
        let board = ChessBoard::from_inverted_array([
            [None, B_KT, B_BP, B_QN, B_KG, B_BP, None, B_RK],
            [None, B_PN, B_PN, B_PN, B_PN, B_PN, B_PN, None],
            [None, None, None, None, None, None, None, None],
            [B_PN, W_KT, None, B_RK, None, None, None, B_PN],
            [W_PN, W_RK, None, None, B_KT, None, None, None],
            [None, None, None, B_RK, None, None, None, None],
            [None, W_PN, W_PN, W_PN, W_PN, W_PN, W_PN, W_PN],
            [None, None, W_BP, W_QN, W_KG, W_BP, W_KT, W_RK],
        ]);

        let position = Position::new(B, Four);
        let white_rook = board.at(&position).unwrap();
        let moves = white_rook.possible_moves(position, &board, None);
        println!("{:?}", moves);
        assert_eq!(moves.len(), 4);
        assert!(moves.contains(&Position::new(B, Three)));
        assert!(moves.contains(&Position::new(C, Four)));
        assert!(moves.contains(&Position::new(D, Four)));
        assert!(moves.contains(&Position::new(E, Four)));

        let position = Position::new(H, One);
        let white_rook = board.at(&position).unwrap();
        let moves = white_rook.possible_moves(position, &board, None);
        println!("{:?}", moves);
        assert_eq!(moves.len(), 0);

        let position = Position::new(D, Five);
        let black_rook = board.at(&position).unwrap();
        let moves = black_rook.possible_moves(position, &board, None);
        println!("{:?}", moves);
        assert_eq!(moves.len(), 7);

        let position = Position::new(H, Eight);
        let black_rook = board.at(&position).unwrap();
        let moves = black_rook.possible_moves(position, &board, None);
        println!("{:?}", moves);
        assert_eq!(moves.len(), 3);
    }
}
