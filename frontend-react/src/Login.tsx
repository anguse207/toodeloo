import './Login.css'
import { useNavigate } from 'react-router-dom'
import TextField from '@mui/material/TextField';
import Box from '@mui/material/Box';

function Login() {
  const navigate = useNavigate();
  const oauthUrl = 'https://discord.com/oauth2/authorize?client_id=1371559638784278568&response_type=code&redirect_uri=http%3A%2F%2Flocalhost%3A1337%2Fauth%2Fdiscord%2Fcallback&scope=identify';

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
      
      {/* Add the Login with Discord button */}
      <button 
        onClick={() => window.location.href = oauthUrl} 
        style={{
          marginTop: '20px',
          padding: '10px 20px',
          backgroundColor: '#5865F2',
          color: 'white',
          border: 'none',
          borderRadius: '5px',
          cursor: 'pointer',
          fontSize: '16px'
        }}
      >
        Login with Discord
      </button>
    </>
  )
}

export default Login