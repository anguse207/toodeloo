
const task_1 = {
    id: "1",
    list_id: "1",
    origin_time: 1732901689,
    title: "Progress",
    content: {
      ops: [
        {
          insert:
            "Completed - ✅\nProgressing - 🏋️\nIncomplete - ❌\n\nImplement backend API",
        },
        { attributes: { list: "bullet" }, insert: "\n" },
        { insert: "Determine between REST and / or WS" },
        { attributes: { indent: 1, list: "bullet" }, insert: "\n" },
        { insert: "Look into openAPI + codegen" },
        { attributes: { indent: 1, list: "bullet" }, insert: "\n" },
        { insert: "Mock API for frontend, " },
        { attributes: { list: "bullet" }, insert: "\n" },
      ],
    }, // Assuming 'Content' is a nested object or structure; replace with an appropriate type or value.
    done: false,
    snoozed_until: 0,
    deleted_time: 0,
  };
  
  const task_2 = {
    id: "2",
    list_id: "1",
    origin_time: 1730901689,
    title: "Daily Jobs",
    content: {
      ops: [
        { insert: "Take out bins" },
        { attributes: { list: "ordered" }, insert: "\n" },
        { insert: "Feed dogs" },
        { attributes: { list: "ordered" }, insert: "\n" },
        { insert: "Satanic ritual" },
        { attributes: { list: "ordered" }, insert: "\n" },
      ],
    }, // Assuming 'Content' is a nested object or structure; replace with an appropriate type or value.
    done: false,
    snoozed_until: 0,
    deleted_time: 0,
  };
  
  const task_3 = {
    id: "3",
    list_id: "1",
    origin_time: 1730901689,
    title: "",
    content: "", // Assuming 'Content' is a nested object or structure; replace with an appropriate type or value.
    done: false,
    snoozed_until: 0,
    deleted_time: 0,
  };
  
  const taskHTML = (id, title, date) => `
    <div class="task-container fade-in" id="task-container-${id}">
      <div class="task-header">
      <div>
          <input 
            class="task-title"
            id="task-title-${id}" 
            placeholder="Enter task title..." 
            value="${title}">
          </input>
          <br />
          <span class="task-date">${date}</span>
      </div>
      <div class="task-buttons">
          <button class="toggle-completion">
            <img src="res/icons/complete.png" alt="snooze">
          </button>
          <button class="snooze">
            <img src="res/icons/snooze.png" alt="snooze">
          </button>
          <button class="remove">
            <img src="res/icons/bin.png" alt="snooze">
          </button>
      </div>
    </div>
    <div id="task-content-${id}"></div>
  `;
  
  const tasksContainer = document.getElementById(`tasks-container`);
  function renderTask(task) {
    // Insert the template
    var date = new Date(task.origin_time * 1000).toISOString().split("T")[0];
    tasksContainer.insertAdjacentHTML(
      "beforeend",
      taskHTML(task.id, task.title, date)
    );
  
    var title_input = document.getElementById(`task-title-${task.id}`);
  
    const characterLimit = 35;
    title_input.addEventListener("input", (e) => {
      console.log("Title changed!");
      const text = e.target.value;
      if (text.length > characterLimit) {
        // TODO: Show a message to the user / animate the input box
        const truncated = text.substring(0, characterLimit);
        e.target.value = truncated;
        // TODO: Save the (trunacted) title to the backend
      } else {
        // TODO: Save the title to the backend
      }
    });
  
    // Initialize the Quill editor
    var quill_content = new Quill(`#task-content-${task.id}`, {
      theme: "snow",
      placeholder: "Dear Diary...", // TODO: Make this random, like 'Complete your masterpiece'...
    });
  
    quill_content.setContents(task.content);
    var editor = document
      .getElementById(`task-content-${task.id}`)
      .getElementsByClassName("ql-editor")[0];
  
    var taskContainer = document.getElementById(`task-container-${task.id}`);
  
    var toolbar = taskContainer.getElementsByClassName("ql-toolbar")[0];
    var in_toolbar = false;
    hide(toolbar);
  
    editor.addEventListener("focusout", () => {
      setTimeout(() => {
        if (!in_toolbar) {
          hide(toolbar);
        }
      }, 0);
    });
  
    editor.addEventListener("focusin", () => {
      show(toolbar);
    });
  
    toolbar.addEventListener("focusin", () => {
      in_toolbar = true;
    });
  
    toolbar.addEventListener("focusout", () => {
      in_toolbar = false;
    });
  
    taskContainer.addEventListener("focusout", () => {
      // TODO: Save the content to the backend
      var contents = quill_content.getContents();
      console.log(
        `Content Not Saved! (Task ID: ${task.id})`
      );
    });
  
    // Add event listeners to the buttons...
    var toggleCompletionButton = document
      .getElementById(`task-container-${task.id}`)
      .getElementsByClassName("toggle-completion")[0];
  }
  
  document.addEventListener("DOMContentLoaded", () => {
    renderTask(task_1);
    renderTask(task_2);
    renderTask(task_3);
  });
  
  function hide(toolbarContainer) {
    toolbarContainer.classList.add("hidden");
  }
  
  function show(toolbarContainer) {
    toolbarContainer.classList.remove("hidden");
  }