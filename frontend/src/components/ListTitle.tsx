import React, { useEffect, useState } from 'react';
import TextField from '@mui/material/TextField';
import { ReadList } from '../api/ReadList';
import { UpdateList } from '../api/UpdateList';

interface ListTextFieldProps {
  listId: string;
  setIsDirty: React.Dispatch<React.SetStateAction<boolean>>;
  refreshListHandler: () => void;
}

const ListTextField: React.FC<ListTextFieldProps> = ({ listId, setIsDirty, refreshListHandler}) => {
  const [label, setLabel] = useState<string>(''); // State for the text field value

  const fetchListData = React.useCallback(async () => {
    const currentList = await ReadList(listId!); // Fetch the list by ID
    if (currentList) {
      setLabel(currentList.label); // Pre-fill the text field with the fetched label
    } else {
      console.error('List not found or could not be fetched');
      setLabel(''); // Reset label if list is not found
    }
  }, [listId]);

  useEffect(() => {
    console.log(listId);
    if (listId) {
      fetchListData();
    }
  });

  const onChange = async (event: React.ChangeEvent<HTMLInputElement>) => {
    setLabel(event.target.value);
    UpdateList(listId, event.target.value);
    refreshListHandler();
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
        label="Label this list"
        variant="standard"
        value={label}
        onChange={onChange}
        fullWidth
        type="text"
      />
    </div>
  );
};

export default ListTextField;