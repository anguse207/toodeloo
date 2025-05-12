import './Login.css'
import { useNavigate } from 'react-router-dom'
import TextField from '@mui/material/TextField';
import Box from '@mui/material/Box';

function Login() {
  const navigate = useNavigate();

  return (
    <>
      <h1>Login</h1>
      <Box
      component="form"
      sx={{ '& > :not(style)': { m: 1, width: '25ch' } }}
      noValidate
      autoComplete="off"
    >
        <div>
          <TextField id="filled-basic" label="Username" variant="filled" />
        </div>
        <div>
          <TextField id="filled-basic" label="Password" variant="filled" type='password' />
        </div>
      </Box>
      
      <button onClick={() => navigate('/')}>Login</button>
      {/* <button onClick={() => navigate('/')}>Go to Home</button> */}
    </>
  )
}

export default Login
