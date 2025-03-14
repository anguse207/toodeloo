// src/Tiptap.tsx
import { useEditor, EditorContent, JSONContent } from '@tiptap/react';
import StarterKit from '@tiptap/starter-kit';
import Highlight from '@tiptap/extension-highlight';
import Typography from '@tiptap/extension-typography';
import { Markdown } from 'tiptap-markdown';
import { TaskProps } from './Task';

// define your extension array
const extensions = [StarterKit, Highlight, Typography, Markdown];

let content: JSONContent[];
let last_update = Date.now();

const TaskContent: React.FunctionComponent<TaskProps> = ({task}) => {
    const editor = useEditor({
        extensions: extensions,
        onUpdate: () => {
            const debounce_time = 1500;
            last_update = Date.now();

            setTimeout(() => {
                if (last_update + debounce_time < Date.now()) {
                    SaveContent();
                }
            }, debounce_time * 1.25); // * 1.25, so that the timeout 
            //  is longer than the last_update check
        },
        onCreate: () => {
            loadContent(task.content);
        }
    });

    const loadContent = (content: JSONContent[]) => {
        if (editor) {
            editor.commands.setContent(content);
        }
    };

    const SaveContent = () => {
        content = editor!.getJSON().content!;
        // console.log(editor!.storage.markdown.getMarkdown()); < Markdown
        console.log(content);
    };

    return (
        <>
            <EditorContent editor={editor} />
        </>
    );
};

export default TaskContent;
