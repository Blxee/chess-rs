import { useEffect, useState } from 'react';
import './App.css';
import GameBoard from './GameBoard'
import { BrowserRouter, Route, Routes, useNavigate } from 'react-router-dom';
import Home from './pages/Home';


// Main app conponent
export default function App() {


  return (
    <BrowserRouter>
      <Routes>

        <Route path='/' element={<Home />} />

        <Route path='random' element={<GameBoard />} />

        <Route path='join' element={<GameBoard />} />

      </Routes>
    </BrowserRouter>
  );

}
