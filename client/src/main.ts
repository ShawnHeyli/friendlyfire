import { Store } from "@tauri-apps/plugin-store";
import { initStoredValues, restoreStoreValues } from "./store";
import { initPingStatus } from "./ping";
import { initDropListener, initMediaPreview, initSendMedia } from "./media";
import { initUpdateAvatarPlaceHolder } from "./avatar";
import { initServerToggle } from "./server";
import { listen } from "@tauri-apps/api/event";

type ClientCount = {
  client_count: number
}

window.addEventListener("DOMContentLoaded", async () => {
  const store = new Store('store.bin');
  restoreStoreValues(store);
  initPingStatus();
  initUpdateAvatarPlaceHolder();
  initDropListener();
  initMediaPreview();
  initServerToggle();
  initStoredValues(store);
  initSendMedia();

  const timeoutRange = document.getElementById("timeoutRange") as HTMLInputElement;
  const timeoutFeedback = document.getElementById("timeoutFeedback") as HTMLSpanElement;
  timeoutRange.addEventListener("input", ()=>{
    console.log(timeoutFeedback.innerText)
    timeoutFeedback.textContent = timeoutRange.value.toString() + "s"
  })

  listen<ClientCount>("ff://client_count", (event) => {
    console.log(event)
  })
});
