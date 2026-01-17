use engine::board::{Board, Color, Piece, PieceType, Square, SquareExt};

fn main() {
    println!("=== Board API Test ===\n");

    // 1. Create a new empty board
    let mut board = Board::new();
    println!("1. Created empty board:");
    println!("{}\n", board);

    // 2. Set pieces using IndexMut + Option::replace
    let e1 = Square::from_coords(0, 4);  // e1
    let e8 = Square::from_coords(7, 4);  // e8
    let b1 = Square::from_coords(0, 1);  // b1
    let c3 = Square::from_coords(2, 2);  // c3

    board[e1].place(Piece::new(PieceType::King, Color::White));
    board[e8].place(Piece::new(PieceType::King, Color::Black));
    board[b1].place(Piece::new(PieceType::Knight, Color::White));

    println!("2. After placing kings on e1/e8 and knight on b1:");
    println!("{}\n", board);

    // 3. Check a square using Index
    println!("3. Checking squares:");
    println!("   e1 has piece: {}", board[e1].is_some());
    println!("   c3 is empty: {}", board[c3].is_none());
    if let Some(piece) = board[e1] {
        println!("   e1 piece: {} {:?}", piece, piece.piece_type());
    }
    println!();

    // 4. Set with replace (returns old value)
    println!("4. Replace knight on b1 with a queen:");
    let old = board[b1].place(Piece::new(PieceType::Queen, Color::White));
    println!("   Old piece was: {:?}", old.map(|p| p.piece_type()));
    println!("{}\n", board);

    // 5. Remove a piece using take
    println!("5. Remove the queen from b1:");
    let removed = board[b1].lift();
    println!("   Removed: {:?}", removed.map(|p| p.piece_type()));
    println!("{}\n", board);

    // 6. Move a piece (atomic operation)
    println!("6. Move white king from e1 to e2:");
    let e2 = Square::from_coords(1, 4);
    let captured = board.move_piece(e1, e2);
    println!("   Captured: {:?}", captured);
    println!("{}\n", board);

    // 7. Iterate over all pieces
    println!("7. All pieces on board:");
    for (sq, piece) in board.pieces() {
        println!("   {} at {}", piece, sq);
    }
    println!();

    // 8. Test different square representations
    println!("8. Indexing with different types:");
    let _ = board[0usize];           // usize index
    let _ = board[(0u8, 4u8)];       // (rank, file) tuple
    let _ = board[e2];               // Square directly
    println!("   All indexing methods work!\n");

    println!("=== All tests passed! ===");
}
