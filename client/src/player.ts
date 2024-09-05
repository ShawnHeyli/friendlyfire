import { listen } from "@tauri-apps/api/event";
import { displayImage, displayVideo } from "./displayMessage";
import { getCurrentWindow } from "@tauri-apps/api/window";

window.addEventListener("DOMContentLoaded", async () => {
  listen<PlayImageMessage>('playImage', (data) => {
    const window = getCurrentWindow();
    window.setAlwaysOnTop(true);
    window.show();
    const payload: PlayImageMessage = data.payload;
    displayImage(payload);
  });

  listen<PlayVideoMessage>('playVideo', (data) => {
    const payload: PlayVideoMessage = data.payload;
    displayVideo(payload);
  });

});
