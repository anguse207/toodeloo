import ListSelector from './component/ListSelector';
import TaskPreview from './component/TaskPreview';
import { content_1, content_2 } from './component/test_data/Task';
import 'react-grid-layout/css/styles.css';
import 'react-resizable/css/styles.css';
import { Responsive, WidthProvider } from 'react-grid-layout';
import { useState } from 'react';
import { Button } from '@mui/material';

const ResponsiveGridLayout = WidthProvider(Responsive);

function Tasks() {
    // For Testing
    const [tasks, setTasks] = useState([
        {
            id: '1',
            creationDate: 1741903638,
            dueDate: 1742573638,
            title: 'Task 1',
            content: content_1,
            completed: true,
        },
        {
            id: '2',
            creationDate: 1741903638,
            dueDate: 1742170638,
            title: 'Task 2',
            content: content_2,
            completed: true,
        },
        {
            id: '3',
            creationDate: 1741903638,
            dueDate: 1742573638,
            title: 'Task 3',
            content: content_1,
            completed: true,
        },
        {
            id: '4',
            creationDate: 1741903638,
            dueDate: 1742170638,
            title: 'Task 4',
            content: content_2,
            completed: true,
        },
    ]);


    const [layouts, setLayouts] = useState({
        lg: tasks.map((task, index) => ({
            i: task.id,
            x: index % 3,
            y: Math.floor(index / 3),
            w: 1,
            h: 1,
        })),
        md: tasks.map((task, index) => ({
            i: task.id,
            x: index % 2,
            y: Math.floor(index / 2),
            w: 1,
            h: 1,
        })),
        sm: tasks.map((task, index) => ({
            i: task.id,
            x: 0,
            y: index,
            w: 1,
            h: 1,
        })),
    });

    const addTask = () => {
        const newTask = {
            id: (tasks.length + 1).toString(),
            creationDate: Date.now(),
            dueDate: Date.now() + 1000000,
            title: `Task ${tasks.length + 1}`,
            content: content_1,
            completed: false,
        };

        const updatedTasks = [...tasks, newTask];
        setTasks(updatedTasks);

        setLayouts({
            lg: [
                ...layouts.lg,
                {
                    i: newTask.id,
                    x: (updatedTasks.length -1) % 3,
                    y: Math.floor(updatedTasks.length / 3),
                    w: 1,
                    h: 1,
                },
            ],
            md: [
                ...layouts.md,
                {
                    i: newTask.id,
                    x: (updatedTasks.length -1) % 2,
                    y: Math.floor(updatedTasks.length / 2),
                    w: 1,
                    h: 1,
                },
            ],
            sm: [
                ...layouts.sm,
                {
                    i: newTask.id,
                    x: 0,
                    y: updatedTasks.length - 1,
                    w: 1,
                    h: 1,
                },
            ],
        });
    };

    return (
        <>
            <ListSelector />
            <Button variant="contained" color="primary" onClick={addTask} sx={{ mb: 2 }}>
                Add Task
            </Button>
            <div>
                <ResponsiveGridLayout
                    className="layout"
                    layouts={layouts}
                    breakpoints={{ lg: 1920, md: 996, sm: 768 }} // Breakpoints
                    cols={{ lg: 3, md: 2, sm: 1 }} // Column count, for each breakpoint
                    rowHeight={300}
                    margin={[15, 20]} // Spacing
                    autoSize={true}
                    isResizable={false}
                    isDraggable={false}
                >
                    {tasks.map((task) => (
                        <div key={task.id}>
                            <TaskPreview task={task} />
                        </div>
                    ))}
                </ResponsiveGridLayout>
            </div>
        </>
    );
}

export default Tasks;
