import { useEffect, useState } from 'react';
import './App.css';
import GameBoard from './GameBoard'


// Main app conponent
export default function App() {

  const joinRandom = () => {
    setTimeout(() => setContent(<GameBoard />), 250);
  };

  const joinMatch = () => {
    setTimeout(() => setContent(<GameBoard />), 250);
  };

  const [content, setContent] = useState(
    <>
      <button onClick={joinRandom}>Join Random</button>
      <button onClick={joinMatch}>Join Match</button>
      {/* <div id='search-bar'> */}
      {/*   <input type='search' placeholder='please enter match id!'/> */}
      {/* </div> */}
    </>
  );

  return (
    <>
      {content}
    </>
  )
}
