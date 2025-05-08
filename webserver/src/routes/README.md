# Toodeloo API Documentation

## Base URL
```
http://localhost:4000
```

---

## Authentication
- **Authentication**: The API uses tokens for authentication. Tokens are issued upon successful login and must be included in the `Authorization` header for protected routes.
- **Header Format**: 
  ```
  Authorization: Bearer <token>
  ```

---

## Endpoints

### 1. **Users**
Endpoints for managing users.

#### **POST /users/create**
Create a new user.

- **Headers**:
  - `Username`: The username of the new user.
  - `Password`: The password of the new user.

- **Response**:
  - `201 Created`: User created successfully.
  - `409 Conflict`: Username already exists.

- **Example**:
  ```bash
  curl -X POST http://localhost:4000/users/create \
       -H "Username: JohnDoe" \
       -H "Password: password123"
  ```

---

#### **POST /users/login**
Log in a user and retrieve a token.

- **Headers**:
  - `Username`: The username of the user.
  - `Password`: The password of the user.

- **Response**:
  - `200 OK`: Returns a JSON object with the token.
  - `401 Unauthorized`: Incorrect password.
  - `409 Conflict`: User does not exist.

- **Example**:
  ```bash
  curl -X POST http://localhost:4000/users/login \
       -H "Username: JohnDoe" \
       -H "Password: password123"
  ```

---

#### **GET /users**
Fetch all users.

- **Response**:
  - `200 OK`: Returns a JSON array of users.

- **Example**:
  ```bash
  curl -X GET http://localhost:4000/users
  ```

---

#### **PUT /users/update**
Update a user's information.

- **Body**:
  ```json
  {
    "name": "NewName",
    "pass": "NewPassword"
  }
  ```

- **Response**:
  - `200 OK`: User updated successfully.
  - `404 Not Found`: User not found.

- **Example**:
  ```bash
  curl -X PUT http://localhost:4000/users/update \
       -H "Content-Type: application/json" \
       -d '{"name": "NewName", "pass": "NewPassword"}'
  ```

---

#### **DELETE /users/remove**
Delete a user.

- **Response**:
  - `200 OK`: User deleted successfully.
  - `404 Not Found`: User not found.

- **Example**:
  ```bash
  curl -X DELETE http://localhost:4000/users/remove
  ```

---

### 2. **Tokens**
Endpoints for managing tokens.

#### **POST /tokens/refresh**
Refresh an expired token.

- **Headers**:
  - `Authorization`: Bearer token.

- **Response**:
  - `200 OK`: Returns a new token.
  - `401 Unauthorized`: Invalid or expired token.

- **Example**:
  ```bash
  curl -X POST http://localhost:4000/tokens/refresh \
       -H "Authorization: Bearer <token>"
  ```

---

### 3. **Lists**
Endpoints for managing lists.

#### **POST /lists/create**
Create a new list.

- **Body**:
  ```json
  {
    "label": "Groceries"
  }
  ```

- **Response**:
  - `201 Created`: List created successfully.

- **Example**:
  ```bash
  curl -X POST http://localhost:4000/lists/create \
       -H "Content-Type: application/json" \
       -d '{"label": "Groceries"}'
  ```

---

#### **GET /lists**
Fetch all lists for the authenticated user.

- **Response**:
  - `200 OK`: Returns a JSON array of lists.

- **Example**:
  ```bash
  curl -X GET http://localhost:4000/lists \
       -H "Authorization: Bearer <token>"
  ```

---

#### **PUT /lists/update/:id**
Update a list.

- **Body**:
  ```json
  {
    "label": "Updated Label"
  }
  ```

- **Response**:
  - `200 OK`: List updated successfully.
  - `404 Not Found`: List not found.

- **Example**:
  ```bash
  curl -X PUT http://localhost:4000/lists/update/<list_id> \
       -H "Content-Type: application/json" \
       -d '{"label": "Updated Label"}'
  ```

---

#### **DELETE /lists/remove/:id**
Delete a list.

- **Response**:
  - `200 OK`: List deleted successfully.
  - `404 Not Found`: List not found.

- **Example**:
  ```bash
  curl -X DELETE http://localhost:4000/lists/remove/<list_id>
  ```

---

### 4. **Tasks**
Endpoints for managing tasks.

#### **POST /tasks/create**
Create a new task.

- **Body**:
  ```json
  {
    "list_id": "<list_id>",
    "title": "Buy Milk",
    "content": "Get milk from the store",
    "done": false
  }
  ```

- **Response**:
  - `201 Created`: Task created successfully.

- **Example**:
  ```bash
  curl -X POST http://localhost:4000/tasks/create \
       -H "Content-Type: application/json" \
       -d '{"list_id": "<list_id>", "title": "Buy Milk", "content": "Get milk from the store", "done": false}'
  ```

---

#### **GET /tasks/:list_id**
Fetch all tasks for a specific list.

- **Response**:
  - `200 OK`: Returns a JSON array of tasks.

- **Example**:
  ```bash
  curl -X GET http://localhost:4000/tasks/<list_id>
  ```

---

#### **PUT /tasks/update/:id**
Update a task.

- **Body**:
  ```json
  {
    "title": "Updated Title",
    "content": "Updated Content",
    "done": true
  }
  ```

- **Response**:
  - `200 OK`: Task updated successfully.
  - `404 Not Found`: Task not found.

- **Example**:
  ```bash
  curl -X PUT http://localhost:4000/tasks/update/<task_id> \
       -H "Content-Type: application/json" \
       -d '{"title": "Updated Title", "content": "Updated Content", "done": true}'
  ```

---

#### **DELETE /tasks/remove/:id**
Delete a task.

- **Response**:
  - `200 OK`: Task deleted successfully.
  - `404 Not Found`: Task not found.

- **Example**:
  ```bash
  curl -X DELETE http://localhost:4000/tasks/remove/<task_id>
  ```

---

## Error Codes
- `200 OK`: Request succeeded.
- `201 Created`: Resource created successfully.
- `400 Bad Request`: Invalid request data.
- `401 Unauthorized`: Authentication failed.
- `404 Not Found`: Resource not found.
- `409 Conflict`: Resource already exists.
