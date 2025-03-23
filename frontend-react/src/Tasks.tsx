import ListSelector from './component/ListSelector';
import Task from './component/Task';
import { content_1, content_2 } from './component/test_data/Task';
import GridLayout from 'react-grid-layout'; // Import react-grid-layout
import 'react-grid-layout/css/styles.css'; // Import default styles
import 'react-resizable/css/styles.css'; // Import resizable styles

// For Testing
export const tasks_list = {
    "tasks": [
        {
            "id": "1",
            "creationDate": 1741903638,
            "dueDate": 1742573638,
            "title": "MUI Box Component",
            "content": content_1,
            "completed": true,
        },
        {
            "id": "2",
            "creationDate": 1741903638,
            "dueDate": 1742170638,
            "title": "Research Lorem Ipsum",
            "content": content_2,
            "completed": true,
        },        {
            "id": "3",
            "creationDate": 1741903638,
            "dueDate": 1742573638,
            "title": "MUI Box Component",
            "content": content_1,
            "completed": true,
        },
        {
            "id": "4",
            "creationDate": 1741903638,
            "dueDate": 1742170638,
            "title": "Research Lorem Ipsum",
            "content": content_2,
            "completed": true,
        }
    ]
};

// TODO: Hide task content, and only show the title
function Tasks() {
    // Define the layout for the grid
    const layout = tasks_list.tasks.map((task, index) => ({
        i: task.id, // Unique identifier for each grid item
        x: (index % 2) * 6, // Start position (2 items per row)
        y: Math.floor(index / 2), // Row position
        w: 6, // Width (6/12 = half the grid width)
        h: 2, // Height (default height)
    }));

    return (
        <>
            <ListSelector />
            <GridLayout
                className="layout"
                layout={layout} // Pass the layout array
                cols={12} // Total columns in the grid
                rowHeight={200} // Height of each row in pixels
                width={1200} // Total width of the grid in pixels
                // draggableHandle=".drag-handle" // Optional: Add a drag handle
            >
                {tasks_list.tasks.map((task, index) => (
                    <div
                        key={task.id}
                        data-grid={{
                            i: task.id,
                            x: (index % 2) * 6,
                            y: Math.floor(index / 2),
                            w: 6,
                            h: 2,
                        }}
                    >
                        <Task task={task} />
                    </div>
                ))}
            </GridLayout>
        </>
    );
}

export default Tasks;