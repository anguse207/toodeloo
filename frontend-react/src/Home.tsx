import React, { useState, useEffect } from 'react';
import Snackbar, { type SnackbarCloseReason } from '@mui/material/Snackbar';
import Button from '@mui/material/Button';
import IconButton from '@mui/material/IconButton';
import { type ListItem } from './components/DashboardLayout';
import { useParams } from 'react-router-dom';
import ListSelector from './components/DashboardLayout';
import { ReadLists } from './api/ReadLists';

const Home: React.FC = () => {
  const { listId } = useParams<{ listId: string | undefined }>(); // Access the taskId parameter
  const [listSelectorItems, setListSelectorItems] = useState<ListItem[] | null>(null);
  const [loginPromptToastOpen, setloginPromptToastOpen] = React.useState(false);

  useEffect(() => {
    refreshListSelectorItems();
    console.log(listId);
  }, []);

  const refreshListSelectorItems = async () => {
    const lists_to_set: ListItem[] = [];
    const newLists = await ReadLists() ?? [];

    if (newLists) {
      for (const list of newLists) {
        lists_to_set.push({ id: list.id, title: list.label });
      }
    } else {
      setloginPromptToastOpen(true);
    }

    setListSelectorItems(lists_to_set);
  }

  const handleClose = (
    event: React.SyntheticEvent | Event,
    reason?: SnackbarCloseReason,
  ) => {
    if (reason === 'clickaway') {
      return;
    }

    setloginPromptToastOpen(false);
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
      <ListSelector lists={listSelectorItems}/>
      <Snackbar
        open={loginPromptToastOpen}
        autoHideDuration={1000}
        onClose={() => setloginPromptToastOpen(false)}
        message="You're not logged in!"
        action={action}
      />
    </>
  );
};

export default Home;