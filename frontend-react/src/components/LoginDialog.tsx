import Button from '@mui/material/Button';
import React from 'react';
import { discordOAuthAuthorizationUrl } from '../Login';
import Dialog from '@mui/material/Dialog';
import DialogTitle from '@mui/material/DialogTitle';
import DialogContent from '@mui/material/DialogContent';
import DialogContentText from '@mui/material/DialogContentText';
import DialogActions from '@mui/material/DialogActions';
import type { TransitionProps } from '@mui/material/transitions';
import { Slide } from '@mui/material';

// Props for the ListSelector component
export interface LoginDialogProps {
  open: boolean; // Null indicates loading state
}

// const LoginPromptToast: React.FC<LoginPromptToastProps> = ({ open }) => {
//   const action = (
//     <React.Fragment>
//       <Button onClick={() => window.location.href = discordOAuthAuthorizationUrl} color="secondary" size="small">
//         Discord
//       </Button>
//       <Button color="secondary" size="small">
//         Github
//       </Button>      
//       <Button color="secondary" size="small">
//         Google
//       </Button>      
//       <Button color="secondary" size="small">
//         Apple
//       </Button>
//     </React.Fragment>
//   );

//   return (
//     <>
//       <Snackbar
//         open={open}
//         // autoHideDuration={1000}
//         onClose={() => open = false}
//         message="You're not logged in, login with..."
//         action={action}
//       />
//     </>
//   );
// };

const Transition = React.forwardRef(function Transition(
  props: TransitionProps & {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    children: React.ReactElement<any, any>;
  },
  ref: React.Ref<unknown>,
) {
  return <Slide direction="up" ref={ref} {...props} />;
});

const LoginPromptToast: React.FC<LoginDialogProps> = ({ open }) => {
  const reauthenticate = (): boolean => {
    const cookieName = 'auth-token';
    const cookies = document.cookie.split(';').map(cookie => cookie.trim());
    return cookies.some(cookie => cookie.startsWith(`${cookieName}=`));
  };

  return (
    <>
      <Dialog
        open={open}
        slots={{
          transition: Transition,
        }}
        keepMounted
        aria-describedby="alert-dialog-slide-description"
      >
        <DialogTitle>{
            reauthenticate() ? 
              "Session expired, re-authenticate to continue" : 
              "Register to continue"
          }
        </DialogTitle>
        <DialogContent>
          <DialogContentText id="alert-dialog-slide-description">
            Choose a service to use for authentication
          </DialogContentText>
        </DialogContent>
        
        <DialogActions>
          {/* Discord Button */}
          <Button
            onClick={() => window.location.href = discordOAuthAuthorizationUrl}
            style={{
              backgroundColor: '#5865F2',
              color: 'white',
              textTransform: 'none',
              fontWeight: 'bold',
            }}
          >
            Sign in with Discord
          </Button>

          {/* GitHub Button */}
          <Button
            onClick={() => window.location.href = "githubOAuthAuthorizationUrl"}
            style={{
              backgroundColor: '#333',
              color: 'white',
              textTransform: 'none',
              fontWeight: 'bold',
            }}
          >
            Sign in with GitHub
          </Button>

          {/* Google Button */}
          <Button
            onClick={() => window.location.href = "googleOAuthAuthorizationUrl"}
            style={{
              backgroundColor: '#4285F4',
              color: 'white',
              textTransform: 'none',
              fontWeight: 'bold',
            }}
          >
            Sign in with Google
          </Button>

          {/* Apple Button */}
          <Button
            onClick={() => window.location.href = "appleOAuthAuthorizationUrl"}
            style={{
              backgroundColor: '#000',
              color: 'white',
              textTransform: 'none',
              fontWeight: 'bold',
            }}
          >
            Sign in with Apple
          </Button>
        </DialogActions>
      </Dialog>
    </>
  );
}

export default LoginPromptToast;