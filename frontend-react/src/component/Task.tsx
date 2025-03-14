import { TextField, Box } from '@mui/material';
import TaskContent from './TaskContent';
import { JSONContent } from '@tiptap/react';
import { useEffect, useRef, useState } from 'react';

export interface TaskProps {
    task: {
        id: number;
        creationDate: number;
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
                    console.log("Updated Title: " + title);
                    // TODO: Impl update logicl
                }
            }, debounce_time * 1.25); // * 1.25, so that the timeout 
            //  is longer than the last_update check
        }
    }, [title, task.title]); // Dependency array ensures it only runs when title changes

    return (
        <Box sx={{ maxWidth: 600, margin: '0 auto', padding: 2, border: '1px dashed grey', borderRadius: 2}}>
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
