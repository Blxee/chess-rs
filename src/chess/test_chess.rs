use super::*;

#[test]
fn test_chess_board_fen() -> Result<(), &'static str> {
    let mut board = ChessBoard::new();

    assert_eq!(
        board.to_fen(),
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w"
    );

    board.move_piece(cvec!("b2"), cvec!("b4"))?;
    assert_eq!(
        board.to_fen(),
        "rnbqkbnr/pppppppp/8/8/1P6/8/P1PPPPPP/RNBQKBNR b"
    );

    board.move_piece(cvec!("c8"), cvec!("a6"))?;
    assert_eq!(
        board.to_fen(),
        "rn1qkbnr/pppppppp/b7/8/1P6/8/P1PPPPPP/RNBQKBNR w"
    );

    board.select_piece(cvec!("g1"))?;
    board.move_piece(cvec!("e1"), cvec!("h6"))?;
    assert_eq!(
        board.to_fen(),
        "rn1qkbnr/pppppppp/b6K/8/1P6/8/P1PPPPPP/RNBQ1BNR b"
    );

    board.select_piece(cvec!("g7"))?;
    board.move_selected(cvec!("g3"))?;
    assert_eq!(
        board.to_fen(),
        "rn1qkbnr/pppppp1p/b6K/8/1P6/6p1/P1PPPPPP/RNBQ1BNR w"
    );

    board.select_piece(cvec!("d1"))?;
    board.move_selected(cvec!("a5"))?;
    assert_eq!(
        board.to_fen(),
        "rn1qkbnr/pppppp1p/b6K/Q7/1P6/6p1/P1PPPPPP/RNB2BNR b"
    );

    Ok(())
}

#[test]
fn test_chess_board_select_piece() -> Result<(), &'static str> {
    let mut board = ChessBoard::new();

    assert!(matches!(
        board.select_piece(cvec!("h5")),
        Err("[Warning]: there is no piece to select")
    ));

    assert!(matches!(
        board.select_piece(cvec!("a8")),
        Err("[Warning]: this is not your piece to select")
    ));

    board.select_piece(cvec!("a1"))?;
    assert_eq!(board.selected_pos, Some(cvec!("a1")));

    board.move_selected(cvec!("a4"))?;
    assert_eq!(board.selected_pos, None);
    assert_eq!(
        board.to_fen(),
        "rnbqkbnr/pppppppp/8/8/R7/8/PPPPPPPP/1NBQKBNR b"
    );

    board.select_piece(cvec!("g7"))?;
    assert_eq!(board.selected_pos, Some(cvec!("g7")));

    board.move_piece(cvec!("h7"), cvec!("h4"))?;
    assert_eq!(
        board.to_fen(),
        "rnbqkbnr/ppppppp1/8/8/R6p/8/PPPPPPPP/1NBQKBNR w"
    );

    Ok(())
}
