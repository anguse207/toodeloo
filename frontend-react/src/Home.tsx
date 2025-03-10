import Button from '@mui/material/Button';

import { Link } from 'react-router-dom';

function Home() {
  return (
    <>
      <Button component={Link} to="/login" variant="contained">
        Login
      </Button>
      <Button component={Link} to="/tasks" variant="contained">
        Tasks
      </Button>
    </>
  );
}

export default Home;
