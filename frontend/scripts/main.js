const host = window.location.origin;
console.log(host);

// POST request, include the 

async function createUser() {
  var path = host + "/users";
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
      body: JSON.stringify({}), // Add a body if needed
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