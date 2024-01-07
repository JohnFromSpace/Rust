use std::collections::HashMap;
use std::io;

#[derive(Clone, Copy, PartialEq)]
enum Color {
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq)]
enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Clone, Copy)]
struct Piece {
    color: Color,
    piece_type: PieceType,
}

#[derive(Clone)]
struct Board {
    squares: [[Option<Piece>; 8]; 8],
    turn: Color,
}

impl Board {
    fn new() -> Board {
        Board {
            squares: [[None; 8]; 8],
            turn: Color::White,
        }
    }

    fn init(&mut self) {
        // Initialize the board with pieces in their starting positions
        self.add_piece(0, 0, PieceType::Rook, Color::White);
        self.add_piece(0, 1, PieceType::Knight, Color::White);
        self.add_piece(0, 2, PieceType::Bishop, Color::White);
        self.add_piece(0, 3, PieceType::Queen, Color::White);
        self.add_piece(0, 4, PieceType::King, Color::White);
        self.add_piece(0, 5, PieceType::Bishop, Color::White);
        self.add_piece(0, 6, PieceType::Knight, Color::White);
        self.add_piece(0, 7, PieceType::Rook, Color::White);

        for i in 0..8 {
            self.add_piece(1, i, PieceType::Pawn, Color::White);
            self.add_piece(6, i, PieceType::Pawn, Color::Black);
        }
        
        self.add_piece(7, 0, PieceType::Rook, Color::Black);
        self.add_piece(7, 1, PieceType::Knight, Color::Black);
        self.add_piece(7, 2, PieceType::Bishop, Color::Black);
        self.add_piece(7, 3, PieceType::Queen, Color::Black);
        self.add_piece(7, 4, PieceType::King, Color::Black);
        self.add_piece(7, 5, PieceType::Bishop, Color::Black);
        self.add_piece(7, 6, PieceType::Knight, Color::Black);
        self.add_piece(7, 7, PieceType::Rook, Color::Black);
    }
    
    fn add_piece(&mut self, row: usize, col: usize, piece_type: PieceType, color: Color) {
        self.squares[row][col] = Some(Piece {
            color,
            piece_type,
        });
    }

    fn print(&self) {
        for i in 0..8 {
            for j in 0..8 {
                if let Some(piece) = self.squares[i][j] {
                    let piece_str = match piece.piece_type {
                        PieceType::Pawn => 'P',
                        PieceType::Rook => 'R',
                        PieceType::Knight => 'N',
                        PieceType::Bishop => 'B',
                        PieceType::Queen => 'Q',
                        PieceType::King => 'K',
                    };
                    let color_str = match piece.color {
                        Color::White => 'W',
                        Color::Black => 'B',
                    };
                    print!("{}{} ", color_str, piece_str);
                } else {
                    print!("-- ");
                }
            }
            println!();
        }
    }

     fn is_valid_move(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        let (from_row, from_col) = from;
        let (to_row, to_col) = to;

        if from_row >= 8 || from_col >= 8 || to_row >= 8 || to_col >= 8 {
            return false; // Out of bounds
        }

        if let Some(piece) = self.squares[from_row][from_col] {
            if piece.color != self.turn {
                return false; // Wrong player's turn
            }

            match piece.piece_type {
                PieceType::Pawn => self.is_valid_pawn_move(from, to),
                PieceType::Rook => self.is_valid_rook_move(from, to),
                PieceType::Knight => self.is_valid_knight_move(from, to),
                PieceType::Bishop => self.is_valid_bishop_move(from, to),
                PieceType::Queen => self.is_valid_queen_move(from, to),
                PieceType::King => self.is_valid_king_move(from, to),
            }
        } else {
            false // No piece at the starting square
        }
    }

    
}
