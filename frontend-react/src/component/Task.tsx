import { useEffect, useState } from 'react';

import { TextField, Box } from '@mui/material';
import Tiptap from './Tiptap';

const Task = () => {
    useEffect(() => {}, []);

    const [title, setTitle] = useState('');

    return (
        <Box sx={{ maxWidth: 600, margin: '0 auto', padding: 2 }}>
            {/* Task Title */}
            <TextField
                label="Task Title"
                variant="outlined"
                fullWidth
                value={title}
                onChange={(e) => setTitle(e.target.value)}
                sx={{ marginBottom: 2 }}
            />

            {/* Quill Editor for Task Description */}
            <Tiptap />
        </Box>
    );
};

export default Task;
