const host = window.location.origin;
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
        "Username": username,
        "Password": password,
      },
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
        "Username": username,
        "Password": password,
      },
    });

    if (response.ok) {
      const token = await response.json();
      user_id = token.user_id;
      auth_token = token.id;
      
      document.getElementById("user-id-mnt").innerText = user_id;
      document.getElementById("user-token-mnt").innerText = auth_token;
      
      loadLists();

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