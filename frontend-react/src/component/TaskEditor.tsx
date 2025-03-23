import { Box, Button, Typography } from '@mui/material';
import { ITask } from './ITask';

interface TaskEditorProps {
    task: ITask['task'];
    setIsEditorOpen: React.Dispatch<React.SetStateAction<boolean>>; // Accept setIsEditorOpen as a prop
}

const TaskEditor: React.FunctionComponent<TaskEditorProps> = ({ task, setIsEditorOpen }) => {
    const CloseEditor = () => {
        setIsEditorOpen(false);
    };
    
    return (
            <Box>
                <Typography variant="h6" component="h2">
                    {task.title}
                </Typography>
                <Typography sx={{ mt: 2 }}>
                    {JSON.stringify(task.content)}
                </Typography>
                <Button variant="contained" color="primary" sx={{ mt: 2 }} onClick={() => CloseEditor()}>
                        Close
                </Button>
            </Box>
    );
};

export default TaskEditor;
