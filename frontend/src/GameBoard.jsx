import React, { useState, useEffect, useRef } from 'react';
import './GameBoard.css';
import blackKing from '/icons/black-king.svg';
import blackQueen from '/icons/black-queen.svg';
import blackBishop from '/icons/black-bishop.svg';
import blackKnight from '/icons/black-knight.svg';
import blackRook from '/icons/black-rook.svg';
import blackPawn from '/icons/black-pawn.svg';
import whiteKing from '/icons/white-king.svg';
import whiteQueen from '/icons/white-queen.svg';
import whiteBishop from '/icons/white-bishop.svg';
import whiteKnight from '/icons/white-knight.svg';
import whiteRook from '/icons/white-rook.svg';
import whitePawn from '/icons/white-pawn.svg';


const CHAR_PIECE_MAP = new Map([
  ["k", blackKing],
  ["q", blackQueen],
  ["b", blackBishop],
  ["n", blackKnight],
  ["r", blackRook],
  ["p", blackPawn],
  ["K", whiteKing],
  ["Q", whiteQueen],
  ["B", whiteBishop],
  ["N", whiteKnight],
  ["R", whiteRook],
  ["P", whitePawn],
]);


/**
 * Represents the chess board and handles socket IO
 */
export default function GameBoard() {

  // Reference to the WebSocket connection
  const socketRef = useRef(null);

  // Connect to the server using a WebSocket
  useEffect(() => {
    const socketLink = `ws://${window.location.host}/ws`;
    const socket = new WebSocket(socketLink);

    socketRef.current = socket;

    socket.onopen = () => {
      console.log("connection successful!");
    };

    socket.onmessage = event => {
      const { result, message } = JSON.parse(event.data);
      if (result === "error") {
        alert(message);
        return;
      }

      const [board, turn] = message.split(" ");

      const grid = {};
      for (let [y, row] of board.split("/").entries()) {
        let x = 0;
        for (let c of row.split("")) {
          if (CHAR_PIECE_MAP.has(c)) {
            grid[[(x++) + y * 8]] = CHAR_PIECE_MAP.get(c);
          } else {
            x += parseInt(c);
          }
        }
      }

      setGrid(grid);
      setTurn(turn);
    }

    socket.onclose = () => {
      console.log("connection closed");
    }

    socket.onerror = err => {
      console.log(err);
    };
  }, []);

  const [turn, setTurn] = useState('w');

  // Initialize the chess grid with the default layout
  const [grid, setGrid] = useState(() => {
    let obj = {
      0: whiteRook,
      1: whiteKnight,
      2: whiteBishop,
      3: whiteQueen,
      4: whiteKing,
      5: whiteBishop,
      6: whiteKnight,
      7: whiteRook,
      56: blackRook,
      57: blackKnight,
      58: blackBishop,
      59: blackQueen,
      60: blackKing,
      61: blackBishop,
      62: blackKnight,
      63: blackRook,
    };
    for (let i = 0; i < 8; i++) {
      obj[8 + i] = whitePawn;
      obj[48 + i] = blackPawn;
    }

    return obj;
  });

  const onCellClicked = (x) => {
    if (socketRef.current === null)
      return;
    const col = "abcdefgh"[x % 8];
    const row = 8 - Math.floor(x / 8);
    socketRef.current.send(`${col}${row}`);
  };

  return (
    <>
      <h1>{"turn: " + turn}</h1>
      <div id='board'>
        {[...Array(8 * 8).keys()].map((x) => {
          return (
            <>
              <div className='cell' onClick={() => onCellClicked(x)} key={x}>
                {grid[x] && <img src={grid[x]} />}
                <p style={{position: "absolute"}}>{
                  `${"abcdefgh"[x % 8]}${8 - Math.floor(x / 8)}`
                }</p>
              </div>
              {x % 8 === 7 && <div className='cell' key={1} hidden></div>}
            </>
          );
        })}
      </div>

    </>
  )
}
