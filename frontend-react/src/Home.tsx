import React, { useState, useEffect } from 'react';
import ListSelector, { type ListItem } from './components/ListSelector';
import { ReadLists } from './api/ReadLists';
import Snackbar, { type SnackbarCloseReason } from '@mui/material/Snackbar';
import Button from '@mui/material/Button';
import IconButton from '@mui/material/IconButton';
import NavigationBar from './components/NavigationBar';

const Home: React.FC = () => {
  const [listSelectorItems, setListSelectorItems] = useState<ListItem[] | null>(null);
  const [loginPromptToastOpen, setloginPromptToastOpen] = React.useState(false);

  // const AddListItem: ListItem = { id: '0', title: 'Add new list' };
  
  useEffect(() => {
    refreshListSelectorItems();

    // // Simulate loading lists from an API
    // setTimeout(async () => {
    //   refreshListSelectorItems();
    // }, 1000);
  });

  const refreshListSelectorItems = async () => {
    const lists_to_set: ListItem[] = [];
    const newLists = await ReadLists();
    // lists_to_set.push(AddListItem);

    if (newLists) {
      for (const list of newLists) {
        lists_to_set.push({ id: list.id, title: list.label });
      }
    } else {
      setloginPromptToastOpen(true);
    }

    setListSelectorItems(lists_to_set);
  }

  const handlelistSelectorChange = (selectedListId: string | null) => {
    console.log('Selected List ID:', selectedListId);
  };

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
      <NavigationBar />
      <ListSelector lists={listSelectorItems} onListChange={handlelistSelectorChange} />
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