import { useNavigate } from 'react-router-dom';

export default function Home() {
  const navigate = useNavigate();

  const joinRandom = () => {
    setTimeout(() => navigate('/random'), 250);
  };

  const joinMatch = () => {
    setTimeout(() => navigate('/join'), 250);
  };

  return (
    <>
      <button onClick={joinRandom}>Join Random</button>
      <button onClick={joinMatch}>Join Match</button>
    </>
  );
}
