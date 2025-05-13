import React, { useState } from 'react';
import { MenuItem, FormControl, InputLabel, Skeleton } from '@mui/material';
import Select from '@mui/material/Select';
import Box from '@mui/material/Box';

// Define the type for a list item
interface ListItem {
  id: string;
  title: string;
}

// Props for the ListSelector component
interface ListSelectorProps {
  lists: ListItem[] | null; // Null indicates loading state
  onListChange?: (selectedListId: string | null) => void;
}

const ListSelector: React.FC<ListSelectorProps> = ({ lists, onListChange }) => {
  const [selectedList, setSelectedList] = useState<string | null>(null);

  const handleChange = (value: string) => {
    const selectedId = value;
    setSelectedList(selectedId);
    onListChange?.(selectedId);
  };

  return (
    <Box sx={{ minWidth: 320 }}>
        <FormControl fullWidth>
        <InputLabel id="list-selector-label">Select Task List</InputLabel>
        {lists === null ? (
            // Show skeleton while lists are loading
            <Skeleton variant="rounded" height={56} />
        ) : (
            <Select
            labelId="list-selector-label"
            label="Select Task List"
            value={selectedList || ''}
            onChange={(event) => handleChange(event.target.value)}
            displayEmpty
            >
            {lists.map((list) => (
                <MenuItem key={list.id} value={list.id}>
                {list.title}
                </MenuItem>
            ))}
            </Select>
        )}
        </FormControl>
    </Box>
  );
};

export default ListSelector;