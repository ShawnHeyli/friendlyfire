import { listen } from "@tauri-apps/api/event";
import { displayImage, displayVideo } from "./displayMessage";
import { debug, error, info, warn } from "@tauri-apps/plugin-log";
import { forwardConsole, forwardUnhandledRejection } from "./log";

forwardConsole('log', debug);
forwardConsole('debug', debug);
forwardConsole('info', info);
forwardConsole('warn', warn);
forwardConsole('error', error);

forwardUnhandledRejection(error);

window.addEventListener("DOMContentLoaded", async () => {

  listen<PlayImageMessage>('playImage', (data) => {
    const payload: PlayImageMessage = data.payload;
    displayImage(payload);
  });

  listen<PlayVideoMessage>('playVideo', (data) => {
    const payload: PlayVideoMessage = data.payload;
    displayVideo(payload);
  });

});
