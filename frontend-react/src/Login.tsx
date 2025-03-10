import Button from '@mui/material/Button';

import { Link } from 'react-router-dom';

function Login() {
  return (
    <>
      <Button variant="contained">Login</Button>
      <Button component={Link} to="/tasks" variant="contained">
        Tasks
      </Button>
    </>
  );
}

export default Login;
