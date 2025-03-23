import { Box, Typography } from '@mui/material';
import { ITask } from './ITask';


const TaskEditor: React.FunctionComponent<ITask> = ({ task }) => {
    return (
            <Box>
                <Typography variant="h6" component="h2">
                    {task.title}
                </Typography>
                <Typography sx={{ mt: 2 }}>
                    {JSON.stringify(task.content)}
                </Typography>
            </Box>
    );
};

export default TaskEditor;
