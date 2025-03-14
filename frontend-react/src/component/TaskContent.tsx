// src/Tiptap.tsx
import { useEditor, EditorContent, JSONContent } from '@tiptap/react';
import StarterKit from '@tiptap/starter-kit';
import Highlight from '@tiptap/extension-highlight';
import Typography from '@tiptap/extension-typography';
import { Markdown } from 'tiptap-markdown';

// define your extension array
const extensions = [StarterKit, Highlight, Typography, Markdown];

let content: JSONContent[] = [
    {
      "type": "heading",
      "attrs": {
        "level": 2
      },
      "content": [
        {
          "type": "text",
          "text": "Introduction"
        }
      ]
    },
    {
      "type": "paragraph",
      "content": [
        {
          "type": "text",
          "text": "The Box component is a generic container for grouping other components. It's a fundamental building block when working with Material UI—you can think of it as a "
        },
        {
          "type": "text",
          "marks": [
            {
              "type": "code"
            }
          ],
          "text": "<div>"
        },
        {
          "type": "text",
          "text": " with extra built-in features, like access to your app's theme and the "
        },
        {
          "type": "text",
          "marks": [
            {
              "type": "code"
            }
          ],
          "text": "sx"
        },
        {
          "type": "text",
          "text": " prop."
        }
      ]
    },
    {
      "type": "heading",
      "attrs": {
        "level": 3
      },
      "content": [
        {
          "type": "text",
          "text": "Usage"
        }
      ]
    },
    {
      "type": "paragraph",
      "content": [
        {
          "type": "text",
          "text": "The Box component differs from other containers available in Material UI in that its usage is intended to be multipurpose and open-ended, just like a "
        },
        {
          "type": "text",
          "marks": [
            {
              "type": "code"
            }
          ],
          "text": "<div>"
        },
        {
          "type": "text",
          "text": ". Components like Container, Stack and Paper, by contrast, feature usage-specific props that make them ideal for certain use cases: Container for main layout orientation, Stack for one-dimensional layouts, and Paper for elevated surfaces."
        }
      ]
    }
  ]
let last_update = Date.now();

const TaskContent = () => {
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
            loadContent(content);
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
