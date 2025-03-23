import { TextField, Box, Modal, Button } from '@mui/material';
import TaskContent from './TaskContent';
import { useEffect, useRef, useState } from 'react';
import { ITask } from './ITask';
import TaskEditor from './TaskEditor';


const Task: React.FunctionComponent<ITask> = ({ task }) => {
    const [title, setTitle] = useState<string>(task.title); // Initialize title state
    const [titlePrompt, setTitlePrompt] = useState<string>(""); // Initialize title state

    const last_title_update = useRef<number>(Date.now()); // Using useRef to persist the time
    useEffect(() => {
        if (title.length === 0) {
            setTitlePrompt("Name your creation!");
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

    const [isEditorOpen, setIsEditorOpen] = useState(false); // State to control modal visibility

    const OpenEditor = () => {
        setIsEditorOpen(true);
    };

    const CloseEditor = () => {
        setIsEditorOpen(false);
    };

    return (
        <>
        <Box
                // onClick={OpenEditor} // Open the modal when the box is clicked
                sx={{
                padding: 2, // Adds padding inside the box
                boxShadow: 3, // Material-UI shadow level (3 is a moderate shadow)
                border: '1px solid', // Adds a border
                borderColor: 'grey.300',
                borderRadius: 2,
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
            <Button variant="contained" color="primary" sx={{ mt: 2 }} onClick={() => OpenEditor()}>
                Edit
            </Button>
            <Box
                sx={{
                    maxHeight: 150,
                    overflow: 'hidden',
                    position: 'relative',
                }}
            >
                <TaskContent task={task} />

                <Box
                    sx={{
                        position: 'absolute',
                        bottom: 0,
                        left: 0,
                        right: 0,
                        height: 50,
                        background:
                            'linear-gradient(to bottom, rgba(255, 255, 255, 0) 0%, rgba(255, 255, 255, 1) 100%)',
                        pointerEvents: 'none',
                    }}
                />
            </Box>
        </Box>
        <Modal open={isEditorOpen} onClose={CloseEditor}>
                <Box
                    sx={{
                        height: '80vh',
                        width: '85vw',
                        position: 'absolute',
                        top: '50%',
                        left: '50%',
                        transform: 'translate(-50%, -50%)',
                        bgcolor: 'background.paper',
                        boxShadow: 24,
                        p: 4,
                        borderRadius: 2,
                    }}
                >
                    <TaskEditor task={task} setIsEditorOpen={setIsEditorOpen}/>
                </Box>
            </Modal>
        </>
    );
};

export default Task;
