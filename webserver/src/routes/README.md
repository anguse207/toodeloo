Here’s a simple API specification for a Todo app with users, lists, and tasks. The API uses tokens for authentication, which are tied to a user.

---

## **API Specification**

### **Authentication**
- **Token-based authentication**: Users must include a valid token in the `Authorization` header for protected routes.

---

### **Endpoints**

#### **1. Users**
##### **POST /users**
- **Description**: Create a new user.
- **Headers**:
  - `Username`: The username of the user.
  - `Password`: The password of the user.
- **Response**:
  - `201 Created`: User created successfully.
  - `409 Conflict`: User already exists.

##### **POST /users/login**
- **Description**: Log in a user and return an authentication token.
- **Headers**:
  - `Username`: The username of the user.
  - `Password`: The password of the user.
- **Response**:
  - `200 OK`: Returns a JSON object with the token.
  - `401 Unauthorized`: Invalid username or password.

##### **GET /users**
- **Description**: Fetch all users (admin-only route).
- **Headers**:
  - `Authorization`: Bearer token.
- **Response**:
  - `200 OK`: Returns a list of users.
  - `403 Forbidden`: Unauthorized access.

---

#### **2. Lists**
##### **POST /lists**
- **Description**: Create a new list.
- **Headers**:
  - `Authorization`: Bearer token.
- **Body**:
  ```json
  {
    "name": "Shopping List"
  }
  ```
- **Response**:
  - `201 Created`: List created successfully.
  - `400 Bad Request`: Invalid input.

##### **GET /lists**
- **Description**: Fetch all lists for the authenticated user.
- **Headers**:
  - `Authorization`: Bearer token.
- **Response**:
  - `200 OK`: Returns a list of lists.

##### **DELETE /lists/{list_id}**
- **Description**: Delete a specific list.
- **Headers**:
  - `Authorization`: Bearer token.
- **Response**:
  - `204 No Content`: List deleted successfully.
  - `404 Not Found`: List not found.

---

#### **3. Tasks**
##### **POST /lists/{list_id}/tasks**
- **Description**: Create a new task in a specific list.
- **Headers**:
  - `Authorization`: Bearer token.
- **Body**:
  ```json
  {
    "title": "Buy milk",
    "description": "Get 2 liters of milk",
    "due_date": "2025-05-10"
  }
  ```
- **Response**:
  - `201 Created`: Task created successfully.
  - `400 Bad Request`: Invalid input.

##### **GET /lists/{list_id}/tasks**
- **Description**: Fetch all tasks in a specific list.
- **Headers**:
  - `Authorization`: Bearer token.
- **Response**:
  - `200 OK`: Returns a list of tasks.

##### **PUT /lists/{list_id}/tasks/{task_id}**
- **Description**: Update a specific task.
- **Headers**:
  - `Authorization`: Bearer token.
- **Body**:
  ```json
  {
    "title": "Buy almond milk",
    "description": "Get 2 liters of almond milk",
    "due_date": "2025-05-12",
    "completed": true
  }
  ```
- **Response**:
  - `200 OK`: Task updated successfully.
  - `404 Not Found`: Task not found.

##### **DELETE /lists/{list_id}/tasks/{task_id}**
- **Description**: Delete a specific task.
- **Headers**:
  - `Authorization`: Bearer token.
- **Response**:
  - `204 No Content`: Task deleted successfully.
  - `404 Not Found`: Task not found.

---

### **Authentication Token**
- Tokens are issued upon successful login and are tied to a user.
- Tokens expire after 24 hours.
- Example token format:
  ```json
  {
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "expiry": "2025-05-08T12:00:00Z"
  }
  ```

---

### **Error Responses**
- `400 Bad Request`: Invalid input or missing required fields.
- `401 Unauthorized`: Invalid or missing token.
- `403 Forbidden`: Access denied.
- `404 Not Found`: Resource not found.
- `500 Internal Server Error`: Unexpected server error.

---

This spec provides a clear structure for managing users, lists, and tasks while ensuring secure access through token-based authentication.