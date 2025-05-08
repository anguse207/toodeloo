const host = window.location.origin + "/api";
console.log(host);

var user_id = null;
var auth_token = null;

async function createUser() {
  var path = host + "/users/create";
  console.log("Creating user + ", path);
  // Get the username and password from the input fields
  const username = document.getElementById("user-name").value;
  const password = document.getElementById("user-pass").value;

  try {
    const response = await fetch(path, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ 
        username: username,
        password: password,
      }),
    });

    if (response.ok) {
      console.log("User created successfully!");
    } else {
      window.alert("Failed to create user:", response.statusText);
    }
  } catch (error) {
    window.alert("Error creating user:", error);
  }
}

async function loadLists() {
  var path = host + "/lists";
  console.log("Loading lists + ", path);

  try {
    const response = await fetch(path, {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        "Bearer": auth_token,
      },
    });

    if (response.ok) {
      const lists = await response.json();
      console.log(lists);

      // Put lists in a Select element
      const listSelect = document.getElementById("list-select");
      listSelect.innerHTML = ""; // Clear previous options
      lists.forEach((list) => {
        const option = document.createElement("option");
        option.value = list.id;
        option.textContent = list.label;
        listSelect.appendChild(option);
      });
    } else {
      window.alert("Failed to load lists:", response.statusText);
    }
  } catch (error) {
    window.alert("Error loading lists:", error);
  }
}

async function loadSelectedList() {
  var listId = document.getElementById("list-select").value;
  if (!listId) {
    console.log("No list selected");
    return;
  }
  var path = host + "/lists/" + listId;
  console.log("Loading list details + ", path);

  try {
    const response = await fetch(path, {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        "Bearer": auth_token,
      },
    });

    if (response.ok) {
      console.log("List loaded successfully!");
      const list = await response.json();
      // Turn list into a string
      const listString = JSON.stringify(list, null, 2);

      document.getElementById("list-details-mnt").innerText = listString;
      loadTasks();
    } else {
      window.alert("Failed to create list:", response.statusText);
    }
  } catch (error) {
    window.alert("Error creating list:", error);
  }
}

async function loadSelectedTask() {
  var listId = document.getElementById("list-select").value;
  var taskId = document.getElementById("task-select").value;
  if (!taskId) {
    console.log("No task selected");
    document.getElementById("task-details-mnt").innerText = "";
    return;
  }
  var path = host + "/tasks/" + listId + "/" + taskId;
  console.log("Loading task details + ", path);

  try {
    const response = await fetch(path, {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        "Bearer": auth_token,
      },
    });

    if (response.ok) {
      console.log("task loaded successfully!");
      const task = await response.json();
      // Turn task into a string
      const taskString = JSON.stringify(task, null, 2);

      document.getElementById("task-details-mnt").innerText = taskString;

    } else {
      window.alert("Failed to create task:", response.statusText);
    }
  } catch (error) {
    window.alert("Error creating task:", error);
  }
}

async function loadTasks() {
  var listId = document.getElementById("list-select").value;
  var path = host + "/tasks" + "/" + listId;
  console.log("Loading tasks + ", path);

  try {
    const response = await fetch(path, {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        "Bearer": auth_token,
      },
    });

    if (response.ok) {
      const tasks = await response.json();
      console.log(tasks);

      // Put tasks in a Select element
      const taskSelect = document.getElementById("task-select");
      taskSelect.innerHTML = ""; // Clear previous options
      tasks.forEach((task) => {
        const option = document.createElement("option");
        option.value = task.id;
        option.textContent = task.title;
        taskSelect.appendChild(option);
      });
      loadSelectedTask(); // Load details of the first task by default
    } else {
      window.alert("Failed to load tasks:", response.statusText);
    }
  } catch (error) {
    window.alert("Error loading tasks:", error);
  }
}

async function loadLists() {
  var path = host + "/lists";
  console.log("Loading lists + ", path);

  try {
    const response = await fetch(path, {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        "Bearer": auth_token,
      },
    });

    if (response.ok) {
      const lists = await response.json();
      console.log(lists);

      // Put lists in a Select element
      const listSelect = document.getElementById("list-select");
      listSelect.innerHTML = ""; // Clear previous options
      lists.forEach((list) => {
        const option = document.createElement("option");
        option.value = list.id;
        option.textContent = list.label;
        listSelect.appendChild(option);
      });
      loadSelectedList(); // Load details of the first list by default
    } else {
      window.alert("Failed to load lists:", response.statusText);
    }
  } catch (error) {
    window.alert("Error loading lists:", error);
  }
}

async function loginUser() {
  var path = host + "/users/login";
  console.log("Login user + ", path);
  // Get the username and password from the input fields
  const username = document.getElementById("user-name").value;
  const password = document.getElementById("user-pass").value;

  try {
    const response = await fetch(path, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ 
        username: username,
        password: password,
      }),
    });

    if (response.ok) {
      const token = await response.json();
      user_id = token.user_id;
      auth_token = token.id;
      
      document.getElementById("user-id-mnt").innerText = user_id;
      document.getElementById("user-token-mnt").innerText = auth_token;
      
      loadLists();
      loadSelectedList();
    } else {
      window.alert("Failed to Login:", response.statusText);
    }
  } catch (error) {
    window.alert("Login error:", error);
  }
}

async function createList() {
  var path = host + "/lists/create";
  console.log("Creating list + ", path);

  // Get the list name from the input field
  const listName = document.getElementById("list-name").value;
  try {
    const response = await fetch(path, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        "Bearer": auth_token,
      },
      body: JSON.stringify({ label: listName }),
    });

    if (response.ok) {
      console.log("List created successfully!");
      loadLists(); // Reload the lists after creating a new one
    } else {
      window.alert("Failed to create list:", response.statusText);
    }
  } catch (error) {
    window.alert("Error creating list:", error);
  }
}

async function createTask() {
  var path = host + "/tasks/create";
  console.log("Creating list + ", path);

  // Get the list name from the input field
  const listId = document.getElementById("list-select").value;
  const title = document.getElementById("task-title").value;
  const content = document.getElementById("task-content").value;
  try {
    const response = await fetch(path, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        "Bearer": auth_token,
      },
      body: JSON.stringify({
        list_id: listId,
        title: title,
        content: content,
      }),
    });

    if (response.ok) {
      console.log("Task created successfully!");
      loadTasks(); // Reload the tasks after creating a new one
    } else {
      window.alert("Failed to create task:", response.statusText);
    }
  } catch (error) {
    window.alert("Error creating task:", error);
  }
}

// Create event listener for list select
document.getElementById("list-select").addEventListener("change", function() {
  loadSelectedList();
});

// Create event listener for list select
document.getElementById("task-select").addEventListener("change", function() {
  loadSelectedTask();
});