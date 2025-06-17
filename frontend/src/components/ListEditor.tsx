import React, { useEffect, useState } from 'react';
import TextField from '@mui/material/TextField';
import { ReadList } from '../api/ReadList';

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
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [listId]);

  const handleTextChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setLabel(event.target.value); // Update the state as the user types
    console.log('Text changed:', event.target.value); // Call a method (e.g., log the change)
  };

  return (
    <div style={{
      margin: '16px auto', 
      width: '60%'
    }}>
    <TextField
      style={{
        marginTop: '48px',
        padding: '16px',
      }}
        id="list-label"
        label="List Label"
        variant="standard"
        value={label} // Display list.label if list is not null
        onChange={handleTextChange} // Call method on text change
        fullWidth
        type="text"
      />
    </div>

  );
};

export default ListTextField;