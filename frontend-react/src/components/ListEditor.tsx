import React, { useEffect, useState } from 'react';
import TextField from '@mui/material/TextField';
import { ReadList, type List } from '../api/ReadList';

interface ListTextFieldProps {
  listId: string | undefined;
}

const ListTextField: React.FC<ListTextFieldProps> = ({ listId }) => {
  const [label, setLabel] = useState<string>(''); // State for the text field value

  const fetchListData = React.useCallback(async () => {
    const currentList = await ReadList(listId!); // Fetch the list by ID
    if (currentList) {
      setLabel(currentList.label); // Pre-fill the text field with the fetched label
    }
  }, [listId]);

  useEffect(() => {
    if (listId) {
      fetchListData();
    }
  }, [listId]);

  const handleTextChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setLabel(event.target.value); // Update the state as the user types
    console.log('Text changed:', event.target.value); // Call a method (e.g., log the change)
  };

  return (
    <TextField
      id="list-label"
      label="Name your task!"
      variant="standard"
      value={label} // Display list.label if list is not null
      onChange={handleTextChange} // Call method on text change
      fullWidth
    />
  );
};

export default ListTextField;