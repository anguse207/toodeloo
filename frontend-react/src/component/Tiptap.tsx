// src/Tiptap.tsx
import { useEditor, EditorContent, JSONContent } from '@tiptap/react';
import StarterKit from '@tiptap/starter-kit';
import Highlight from '@tiptap/extension-highlight';
import Typography from '@tiptap/extension-typography';
import { Button } from '@mui/material';
import { Markdown } from 'tiptap-markdown';

// define your extension array
const extensions = [StarterKit, Highlight, Typography, Markdown];

let content: JSONContent[];
let last_update = Date.now();

const Tiptap = () => {
    const editor = useEditor({
        extensions: extensions,
        onUpdate: () => {
            const debounce_time = 1500;
            last_update = Date.now();
            setTimeout(() => {
                if (last_update + debounce_time < Date.now()) {
                    SaveContent();
                }
            }, debounce_time * 1.25); // * 1.25, so that the timeout is longer than the last_update check
        },
    });

    const loadContent = (content: JSONContent[]) => {
        if (editor) {
            editor.commands.setContent(content);
        }
    };

    const SaveContent = () => {
        if (content) {
            content = editor!.getJSON().content!;
            console.log(editor!.storage.markdown.getMarkdown());
        }
    };

    return (
        <>
            <Button onClick={() => SaveContent()}>save</Button>
            <Button onClick={() => loadContent(content)}>load</Button>
            <EditorContent editor={editor} />
        </>
    );
};

export default Tiptap;
