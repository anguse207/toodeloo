import Snackbar, { type SnackbarCloseReason } from '@mui/material/Snackbar';
import Button from '@mui/material/Button';
import IconButton from '@mui/material/IconButton';
import React from 'react';

// Props for the ListSelector component
export interface LoginPromptToastProps {
  open: boolean; // Null indicates loading state
}

const LoginPromptToast: React.FC<LoginPromptToastProps> = ({ open }) => {
  const handleClose = (
    _event: React.SyntheticEvent | Event,
    reason?: SnackbarCloseReason,
  ) => {
    if (reason === 'clickaway') {
      return;
    }

    // setOpen(false);
  };

  const action = (
    <React.Fragment>
      <Button color="secondary" size="small" onClick={handleClose}>
        Login
      </Button>
      <IconButton
        size="small"
        aria-label="close"
        color="inherit"
        onClick={handleClose}
      >
        {/* <CloseIcon fontSize="small" /> */}
      </IconButton>
    </React.Fragment>
  );

  return (
    <>
      <Snackbar
        open={open}
        autoHideDuration={1000}
        onClose={() => open = false}
        message="You're not logged in!"
        action={action}
      />
    </>
  );
};

export default LoginPromptToast;