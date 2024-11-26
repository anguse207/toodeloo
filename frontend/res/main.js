const titleHTML = () => '<h1 id="title">m o n i🌱o r</h1>';

const containerHTML = (uuid) => `
    <div class="task-container fade-in" id="${uuid}">
        <h2 class="label">Client Label</h2>
        <p class="uuid">${uuid}</p>

        <p class="label">Temperature °C</p>
        <p class="value temp">${20}</p>

        <p class="label">Flowing</p>
        <p class="value flow"></p>

        <p class="label">Latched Command</p>
        <p class="value command-type"></p>
        <p class="value command-time"></p>

        <p class="label">Command Type</p>
        <select class="dropdown">
            <option value="Water">Water</option>
            <option value="Reboot">Reboot</option>
            <option value="CycleTime">Cycle Time</option>
        </select>

        <p class="label">Command Duration (S)</p>
        <input class="input-box" type="number" placeholder="Command Duration (Seconds)">
        
        <button onclick="sendCommand('${uuid}')">Submit</button>
    </div>
`;

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function getTemperature(uuid) {
  return parseInt(
    document.getElementById(uuid).querySelector(".temp").textContent,
    10
  );
}
async function updateTemperature(uuid, temperature) {
  let container = document.getElementById(uuid);
  let temp = container.querySelector(".temp");

  if (getTemperature(uuid) != temperature) {
    temp.textContent = temperature;
    pulseText(temp);
  }
}

async function pulseText(element) {
  element.classList.add("pulse-text");
  setTimeout(() => {
    element.classList.remove("pulse-text");
  }, 2000);
}

async function init() {
  const response = await fetch("server/data/get", {
    method: "GET",
  });
  const clients = await response.json();

  clients.forEach((client) => {
    document.body.insertAdjacentHTML("beforeend", containerHTML(client.id));
    updateTemperature(client.id, client.internal_temp);
    updateFlow(client.id, client.is_flowing);

    if (client.command) {
      updateCommandType(client.id, client.command.command_type);
      updateCommandTime(client.id, client.command.duration_seconds);
    } else {
      updateCommandType(client.id, "None");
      updateCommandTime(client.id, 0);
    }
    pulseContainer(document.getElementById(client.id));
  });
}

init();
