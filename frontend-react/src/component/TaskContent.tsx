// src/Tiptap.tsx
import { useEditor, EditorContent, JSONContent } from '@tiptap/react';
import StarterKit from '@tiptap/starter-kit';
import Highlight from '@tiptap/extension-highlight';
import Typography from '@tiptap/extension-typography';
import { Markdown } from 'tiptap-markdown';
import { ITask } from './ITask';

// define your extension array
const extensions = [StarterKit, Highlight, Typography, Markdown];


interface TaskContentProps {
    task: ITask['task'];
    editable: boolean;
}

const TaskContent: React.FunctionComponent<TaskContentProps> = ({ task, editable }) => {
    let last_update = Date.now();

    const editor = useEditor({
        extensions: extensions,
        editable: editable,
        onUpdate: () => {
            if (!editable) {
                return;
            }

            const debounce_time = 1500;
            last_update = Date.now();

            setTimeout(() => {
                if (last_update + debounce_time < Date.now()) {
                    SaveContent();
                    console.log('Updating Content for task: ' + task.id);
                }
            }, debounce_time * 1.25); // * 1.25, so that the timeout
            //  is longer than the last_update check
        },
        onCreate: () => {
            loadContent(task.content);
        },
    });

    const loadContent = (content: JSONContent[]) => {
        if (editor) {
            editor.commands.setContent(content);
        }
    };

    const SaveContent = () => {
        task.content = editor!.getJSON().content!;
        // console.log(editor!.storage.markdown.getMarkdown()); < Markdown
        // console.log(content);
    };

    return (
        <>
            <EditorContent editor={editor} />
        </>
    );
};

export default TaskContent;
