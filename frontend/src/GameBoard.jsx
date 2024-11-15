import { useState, useEffect } from 'react';
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


// Chess board component
export default function GameBoard() {

  // Connect to the server using a WebSocket
  useEffect(() => {
    const socketLink = `ws://${window.location.host}/ws`;
    const socket = new WebSocket(socketLink);

    socket.onopen = () => {
      alert("connection successful!");
    };

    socket.onmessage = event => {
      const data = JSON.parse(event.data);
      alert(data);
    }
    socket.onerror = err => { alert(err); };
  }, []);

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

  return (
    <>
      <div id='board'>
        {[...Array(8 * 8).keys()].map((x) => {
          return (
            <>
              <div className='cell' key={x}>
                { grid[x] && <img src={grid[x]}/> }
              </div>
              { x % 8 === 7 && <div className='cell' key={1} hidden></div> }
            </>
          );
        })}
      </div>

    </>
  )
}
