import './Login.css'
import Box from '@mui/material/Box';

function Login() {
  const oauthUrl = 'https://discord.com/oauth2/authorize?client_id=1371559638784278568&response_type=code&redirect_uri=http%3A%2F%2Flocalhost%3A1337%2Fauth%2Fdiscord%2Fcallback&scope=identify';

  return (
    <>
      {/* <h1>Login</h1> */}
      <Box>
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
        Continue with Discord
      </button>
      </Box>
    </>
  )
}

export default Login