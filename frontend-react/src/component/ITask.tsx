import { JSONContent } from "@tiptap/react";

export interface ITask {
    task: {
        readonly id: string;
        readonly creationDate: number;
        dueDate: number;
        title: string;
        content: JSONContent[];
        completed: boolean;
    };
}