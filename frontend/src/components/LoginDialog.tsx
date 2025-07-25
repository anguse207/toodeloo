import Button from '@mui/material/Button';
import React from 'react';
import Dialog from '@mui/material/Dialog';
import DialogTitle from '@mui/material/DialogTitle';
import DialogContent from '@mui/material/DialogContent';
import DialogContentText from '@mui/material/DialogContentText';
import DialogActions from '@mui/material/DialogActions';
import type { TransitionProps } from '@mui/material/transitions';
import { Slide, Stack } from '@mui/material';

export const discordOAuthAuthorizationUrl = 'https://discord.com/oauth2/authorize?client_id=1371559638784278568&response_type=code&redirect_uri=http%3A%2F%2Flocalhost%3A1337%2Fauth%2Fdiscord%2Fcallback&scope=identify';

// Props for the ListSelector component
export interface LoginDialogProps {
  open: boolean; // Null indicates loading state
}

const Transition = React.forwardRef(function Transition(
  props: TransitionProps & {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    children: React.ReactElement<any, any>;
  },
  ref: React.Ref<unknown>,
) {
  return <Slide direction="up" ref={ref} {...props} />;
});

// Shared button styles
const buttonStyle = {
  color: 'white',
  textTransform: 'none',
  fontWeight: 'bold',
  paddingTop: 20,
  paddingRight: 50,
  paddingBottom: 20,
  paddingLeft: 50,
};

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
        <Stack direction="column" spacing={2} sx={{ width: '100%' }}>
            {/* Discord Button */}
            <Button
              onClick={() => window.location.href = discordOAuthAuthorizationUrl}
              style={{ ...buttonStyle, backgroundColor: '#5865F2' } as React.CSSProperties}
            >
              Discord
            </Button>

            {/* GitHub Button */}
            <Button
              onClick={() => window.location.href = "githubOAuthAuthorizationUrl"}
              style={{ ...buttonStyle, backgroundColor: '#333' } as React.CSSProperties }
            >
              GitHub
            </Button>

            {/* Google Button */}
            <Button
              onClick={() => window.location.href = "googleOAuthAuthorizationUrl"}
              style={{ ...buttonStyle, backgroundColor: '#4285F4' } as React.CSSProperties}
            >
              Google
            </Button>

            {/* Apple Button */}
            <Button
              onClick={() => window.location.href = "appleOAuthAuthorizationUrl"}
              style={{ ...buttonStyle, backgroundColor: '#000' } as React.CSSProperties}
            >
              Apple
            </Button>
          </Stack>
        </DialogActions>
      </Dialog>
    </>
  );
}

export default LoginPromptToast;