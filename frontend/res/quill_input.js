const test_data = {
  ops: [
    { insert: "Shopping List\nEggs" },
    { attributes: { list: "ordered" }, insert: "\n" },
    { insert: "Milk" },
    { attributes: { list: "ordered" }, insert: "\n" },
    { insert: "Cheese" },
    { attributes: { list: "ordered" }, insert: "\n" },
    { insert: "Chicken" },
    { attributes: { list: "ordered" }, insert: "\n" },
  ],
};

const taskHTML = (uuid) => `
  <div class="task-container fade-in" id="task-container-${uuid}">
    <div class="task-header">
    <div>
        <span class="task-title">Task Title</span>
        <br />
        <span class="task-date">1963-11-22</span>
    </div>
    <div class="task-buttons">
        <button class="toggle-completion">C</button>
        <button class="snooze">S</button>
        <button class="remove">X</button>
    </div>
  </div>
  <div id="task-content-${uuid}"></div>
`;

const toolbarHTML = (uuid) => `
  <div class="hidden toolbar" id="toolbar-${uuid}">
  <!-- Add font size dropdown -->
  <select class="ql-size">
    <option value="small"></option>
    <!-- Note a missing, thus falsy value, is used to reset to default -->
    <option selected></option>
    <option value="large"></option>
    <option value="huge"></option>
  </select>
  <!-- Add a bold button -->
  <button class="ql-bold"></button>
  <!-- Add subscript and superscript buttons -->
  <button class="ql-script" value="sub"></button>
  <button class="ql-script" value="super"></button>
</div>
`;

var toolbarAnchor = document.getElementById(`toolbar-anchor`);
var toolbars = []

const tasksContainer = document.getElementById(`tasks-container`);
function renderTask(uuid) {
  // Insert the template
  tasksContainer.insertAdjacentHTML("beforeend", taskHTML(uuid));
  toolbarAnchor.insertAdjacentHTML("beforeend", toolbarHTML(uuid));

  // Initialize the Quill editor
  var _quill = new Quill(`#task-content-${uuid}`, {
    theme: "snow",
    placeholder: "Dear Diary...", // TODO: Make this random, like 'Complete your masterpiece'...
    modules: {
      toolbar: `#toolbar-${uuid}`,
    },
  });

  var toolbar = document.getElementById(`toolbar-${uuid}`);
  toolbars.push(toolbar);

  var editor = document
    .getElementById(`task-content-${uuid}`)
    .getElementsByClassName("ql-editor")[0];

  editor.addEventListener("focus", () => {
    updateToolbarVisibility(toolbar);
  });
}

document.addEventListener("DOMContentLoaded", () => {
  renderTask(0);
  renderTask(1);

  // document.body.insertAdjacentHTML("beforeend", taskHTML(0));

  // // Loading Data
  // quill.setContents(test_data);

  // // Handle the "Show Output" button
  // quill.on("text-change", () => {
  //   // Get the formatted HTML content from the Quill editor
  //   const htmlContent = quill.root.innerHTML;

  //   // Display the raw HTML in the preformatted block
  //   document.getElementById("html-output").textContent = htmlContent;

  //   // Saving Data
  //   const delta = quill.getContents();
  //   document.getElementById("quill-delta").textContent = JSON.stringify(delta);
  // });
});

function updateToolbarVisibility(selected) {
  toolbars.forEach(toolbar => {
    if (toolbar.id === selected.id) {
      // console.log(`Using: ${toolbar.id}`);
      show(toolbar);
      pulseContainer(toolbar);
    } else {
      hide(toolbar); // Hide others
    }
  });
}

function hide(toolbarContainer) {
  toolbarContainer.classList.add("hidden");
}

function show(toolbarContainer) {
  // hide all other toolbars
  toolbarContainer.classList.remove("hidden");
}

async function pulseContainer(element) {
  element.classList.add("pulse-text");
  setTimeout(() => {
    element.classList.remove("pulse-text");
  }, 400);
}