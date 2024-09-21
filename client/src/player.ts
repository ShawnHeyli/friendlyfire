import { listen } from "@tauri-apps/api/event";
import { displayImage, displayVideo } from "./displayMessage";
import { error } from "@tauri-apps/plugin-log";
import { forwardUnhandledRejection } from "./log";

// forwardConsole('log', debug);
// forwardConsole('debug', debug);
// forwardConsole('info', info);
// forwardConsole('warn', warn);
// forwardConsole('error', error);

forwardUnhandledRejection(error);

window.addEventListener("DOMContentLoaded", async () => {

  listen<PlayImageMessage>('playImage', async (data) => {
    const payload: PlayImageMessage = data.payload;
    await displayImage(payload);
  });

  listen<PlayVideoMessage>('playVideo', async (data) => {
    const payload: PlayVideoMessage = data.payload;
    await displayVideo(payload);
  });
});

