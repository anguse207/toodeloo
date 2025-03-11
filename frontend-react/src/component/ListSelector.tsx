import * as React from 'react';

import InputLabel from '@mui/material/InputLabel';
import MenuItem from '@mui/material/MenuItem';
import FormControl from '@mui/material/FormControl';
import Select, { SelectChangeEvent } from '@mui/material/Select';

export default function ListSelector() {
    const [selectedList, changeList] = React.useState('');

    const handleChange = (event: SelectChangeEvent) => {
        console.log(('Changing to list - ' + event.target.value) as string);
        changeList(event.target.value as string);
    };

    return (
        <>
            <FormControl fullWidth>
                <InputLabel id="demo-simple-select-label">
                    Select List
                </InputLabel>
                <Select
                    labelId="demo-simple-select-label"
                    id="demo-simple-select"
                    value={selectedList}
                    label="Selected List"
                    onChange={handleChange}
                >
                    <MenuItem value={'list-uuid-1'}>Todo App Features</MenuItem>
                    <MenuItem value={'list-uuid-2'}>Shopping List</MenuItem>
                    <MenuItem value={'list-uuid-3'}>Birthday Presents</MenuItem>
                </Select>
            </FormControl>
        </>
    );
}
