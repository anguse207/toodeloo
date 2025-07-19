import React, { useEffect, useRef, useState } from 'react';
import TextField from '@mui/material/TextField';
import { ReadList } from '../api/ReadList';
import { UpdateList } from '../api/UpdateList';
import { debounce } from '../utils/debounce';

interface ListTextFieldProps {
  listId: string;
  setIsDirty: React.Dispatch<React.SetStateAction<boolean>>
}

const ListTextField: React.FC<ListTextFieldProps> = ({ listId , setIsDirty}) => {
  const [label, setLabel] = useState<string>(''); // State for the text field value

// Example: mark form as dirty when user types
  const fetchListData = React.useCallback(async () => {
    const currentList = await ReadList(listId!); // Fetch the list by ID
    if (currentList) {
      setLabel(currentList.label); // Pre-fill the text field with the fetched label
    }
  }, [listId]);

  useEffect(() => {
    console.log(listId);
    if (listId) {
      fetchListData();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [listId]);

  // Create the debounced function
  const debouncedUpdate = useRef(
    debounce((value: string) => {
      console.log(`Updating List "${listId}" with "${value}"`);
      UpdateList(listId, value);
      setIsDirty(false);
    }, 2000)
  );

  // Cleanup debounce when listId changes
  useEffect(() => {
    const currentDebouncedUpdate = debouncedUpdate.current;
    return () => {
      currentDebouncedUpdate.cancel(); // Cancel the debounce
    };
  }, [listId]);

  const onChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setIsDirty(true);
    setLabel(event.target.value);
    debouncedUpdate.current(event.target.value);
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
        value={label} // Display list.label if list is not null
        onChange={onChange} // Call method on text change
        fullWidth
        type="text"
      />
    </div>
  );
};

export default ListTextField;