const host = window.location.origin;
console.log(host);

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
      window.alert("User created successfully!");
    } else {
      window.alert("Failed to create user:", response.statusText);
    }
  } catch (error) {
    window.alert("Error creating user:", error);
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
      window.alert("Login successfully!");
      const data = await response.json();
      console.log("Login data:", data);
    } else {
      window.alert("Failed to Login:", response.statusText);
    }
  } catch (error) {
    window.alert("Login error:", error);
  }
}