import { TextField, Box } from '@mui/material';
import TaskContent from './TaskContent';
import { JSONContent } from '@tiptap/react';
import { useEffect, useRef, useState } from 'react';

export interface TaskProps {
    task: {
        readonly id: string;
        readonly creationDate: number;
        dueDate: number;
        title: string;
        content: JSONContent[];
        completed: boolean;
    };
}

const Task: React.FunctionComponent<TaskProps> = ({task}) => {
    const [title, setTitle] = useState<string>(task.title); // Initialize title state
    
    const last_title_update = useRef<number>(Date.now()); // Using useRef to persist the time
    useEffect(() => {
        if (title !== task.title) {
            const debounce_time = 1500;
            last_title_update.current = Date.now();

            setTimeout(() => {
                if (last_title_update.current + debounce_time < Date.now()) {
                    console.log("Updating Title for task: " + task.id);
                    // TODO: Impl update logic
                }
            }, debounce_time * 1.25); // * 1.25, so that the timeout 
            //  is longer than the last_update check
        }
    // eslint-disable-next-line react-hooks/exhaustive-deps 
    }, [title, task.title]);

    return (
<Box
    sx={{
        padding: 2, // Adds padding inside the box
        maxWidth: 600,
        margin: '0 auto',
        boxShadow: 3, // Material-UI shadow level (3 is a moderate shadow)
        border: '1px solid', // Adds a border
        borderColor: 'grey.300', // Material-UI theme color for the border
        borderRadius: 2, // Rounded corners (theme spacing unit)
        backgroundColor: 'background.paper', // Ensures it matches the theme's background
    }}
>
    <TextField
        label="Task Title"
        variant="outlined"
        fullWidth
        value={title}
        onChange={(e) => setTitle(e.target.value)}
        sx={{ marginBottom: 2 }}
    />
    <TaskContent task={task} />
</Box>
    );
};

export default Task;
