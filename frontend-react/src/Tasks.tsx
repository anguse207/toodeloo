import ListSelector from './component/ListSelector';
import Task from './component/Task';
import { content_1, content_2 } from './component/test_data/Task';

export const tasks_list = {
"tasks": [
    {
    "id": 1,
    "creationDate": 1741903638,
    "dueDate": 1742573638,
    "title": "MUI Box Component",
    "content": content_1,
    "completed": true,
    },
    {
    "id": 2,
    "creationDate": 1741903638,
    "dueDate": 1742170638,
    "title": "Research Lorem Ipsum",
    "content": content_2,
    "completed": true,
    }
]
}
  

function Tasks() {
    return (
        <>
            <ListSelector />
            <Task task={tasks_list.tasks[0]}/>
            <Task task={tasks_list.tasks[1]}/>
        </>
    );
}

export default Tasks;
