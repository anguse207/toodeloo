import { TextField, Box, Button } from '@mui/material';
import TaskContent from './TaskContent';
import { useEffect, useRef, useState } from 'react';
import { ITask } from './ITask';

interface TaskEditorProps {
    task: ITask['task'];
    setIsEditorOpen: React.Dispatch<React.SetStateAction<boolean>>;
}

const Task: React.FunctionComponent<TaskEditorProps> = ({
    task,
    setIsEditorOpen,
}) => {
    const [title, setTitle] = useState<string>(task.title);
    const [titlePrompt, setTitlePrompt] = useState<string>('');

    const last_title_update = useRef<number>(Date.now());
    useEffect(() => {
        if (title.length === 0) {
            setTitlePrompt('Name your creation!');
        }

        if (title !== task.title) {
            const debounce_time = 1500;
            last_title_update.current = Date.now();

            setTimeout(() => {
                if (last_title_update.current + debounce_time < Date.now()) {
                    console.log('Updating Title for task: ' + task.id);
                    // TODO: Impl update logic
                }
            }, debounce_time * 1.25); // * 1.25, so that the timeout
            //  is longer than the last_update check
        }
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [title, task.title]);

    const CloseEditor = () => {
        setIsEditorOpen(false);
    };

    return (
        <>
            <Button
                variant="contained"
                color="primary"
                sx={{ mt: 2, margin: 1 }}
                onClick={() => CloseEditor()}
            >
                Close
            </Button>
            <Box
                sx={{
                    padding: 2,
                    backgroundColor: 'background.paper',
                    position: 'relative',
                }}
            >
                <TextField
                    label={titlePrompt}
                    variant="outlined"
                    fullWidth
                    value={title}
                    onChange={(e) => setTitle(e.target.value)}
                    sx={{ marginBottom: 2 }}
                />

                <TaskContent task={task} editable={true}/>

            </Box>
        </>
    );
};

export default Task;
