extern crate chess;

use chess::{ MoveGen, Board };

pub mod chessengine {
    pub fn get_moves(fen: String) -> String {
        let board = Board::from_fen(fen.to_owned()).unwrap();
        let iterable = MoveGen::new(board, true);

        let mut ret: String = "[".to_string();
        let mut count: usize = 0;
        let size: usize = iterable.len();

        for item in iterable {
            ret.push_str(&item.to_string());

            if count < size - 1 {
                ret.push_str(&", ");
            }

            count += 1;
        }

        ret.push_str(&"]");

        return ret;
    }
}
