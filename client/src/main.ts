import { Store } from "@tauri-apps/plugin-store";
import { initStoredValues, restoreStoreValues } from "./store";
import { initPingStatus } from "./ping";
import { initDropListener, initMediaPreview } from "./media";
import { initUpdateAvatarPlaceHolder } from "./avatar";
import { initServerToggle } from "./server";

window.addEventListener("DOMContentLoaded", async () => {
  const store = new Store('store.bin');
  restoreStoreValues(store);
  initPingStatus();
  initUpdateAvatarPlaceHolder();
  initDropListener();
  initMediaPreview();
  initServerToggle();
  initStoredValues(store);
});
