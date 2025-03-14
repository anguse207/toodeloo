import { useEffect, useState } from 'react';

import { TextField, Box } from '@mui/material';
import TaskContent from './TaskContent';

const Task = () => {
    useEffect(() => {}, []);

    const [title, setTitle] = useState('');

    return (
        <Box sx={{ maxWidth: 600, margin: '0 auto', padding: 2, border: '1px dashed grey', borderRadius: 2}}>
            {/* Task Title */}
            <TextField
                label="Task Title"
                variant="outlined"
                fullWidth
                value={title}
                onChange={(e) => setTitle(e.target.value)}
                sx={{ marginBottom: 2 }}
            />  
            <TaskContent />
        </Box>
    );
};

export default Task;
