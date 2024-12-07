document.getElementById("header-select").addEventListener("change", (e) => {
  var selectedId = e.target.options[e.target.selectedIndex].id;
  loadList(selectedId);
});

function loadList(listId) {
  fetch(`${host}/api/lists?id=${listId}&token=${userToken}`, {
    method: "GET",
  })
    .then((response) => {
      if (!response.ok) {
        throw new Error("Network response was not OK " + response.statusText);
      }
      return response.json();
    })
    .then((data) => {
      console.log(data);
      loadTasks(data);
    })
    .catch((error) => {
      console.error("There was a problem with the fetch operation:", error);
    });
}

function loadTasks(tasks) {
  tasks.forEach((task) => {
    renderTask(task);
  });
}
