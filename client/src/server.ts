import { invoke } from "@tauri-apps/api/core";
import { popAlert } from "./alert";

export function initServerToggle() {
  let connected = false;
  const serverToggle = document.getElementById("serverToggle") as HTMLButtonElement;

  serverToggle.addEventListener("click", async () => {
    if (!connected) {
      let domain = getServerDomain()
      await invoke("connect_to_server", { domain }).then(() => {
        serverToggle.textContent = "Disconnect"
        connected = true;
        popAlert("success", "Connected to the server", null);
      }, (error) => {
        popAlert("error", "Could not connect to the server", error);
      })
    }
    else {
      await invoke("disconnect_from_server").then(() => {
        serverToggle.textContent = "Connect"
        connected = false;
        popAlert("success", "Disconnected from the server", null);
      }, (error) => {
        popAlert("error", "Could not connect to the server", error);
      })
    }
  })
}

export function getServerDomain(): string {
  const serverInput = document.getElementById("serverInput") as HTMLInputElement;
  const url = serverInput.value;

  let urlParts = url.split('://');
  let strippedUrl = urlParts.length > 1 ? urlParts[1] : urlParts[0];
  if (strippedUrl.endsWith('/')) {
    strippedUrl = strippedUrl.slice(0, -1); // Remove trailing slash
  }

  return strippedUrl
}
