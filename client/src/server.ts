import { invoke } from "@tauri-apps/api/core";

export function initServerToggle() {
  let connected = false;
  const serverToggle = document.getElementById("serverToggle") as HTMLButtonElement;

  serverToggle.addEventListener("click", async () => {
    if (!connected) {
      let domain = getServerDomain()
      await invoke("connect_to_server", { domain }).then(() => {
        serverToggle.textContent= "Disconnect"
        connected = true;
      })
    }
    else {
      await invoke("disconnect_from_server").then(() => {
        serverToggle.textContent = "Connect"
        connected = false;
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
