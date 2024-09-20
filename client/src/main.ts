import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { debug, error, info, warn } from '@tauri-apps/plugin-log';
import { forwardConsole, forwardUnhandledRejection } from './log';
import { open } from '@tauri-apps/plugin-dialog';
import * as fileType from 'file-type';
import { readFile } from '@tauri-apps/plugin-fs';
import { initServerStatus } from './serverStatus';


forwardConsole('log', debug);
forwardConsole('debug', debug);
forwardConsole('info', info);
forwardConsole('warn', warn);
forwardConsole('error', error);

forwardUnhandledRejection(error);

window.addEventListener("DOMContentLoaded", async () => {

  initServerStatus(3000);

  let connected = false;
  const serverButton = document.getElementById('serverButton') as HTMLButtonElement;
  serverButton.addEventListener("click", async () => {
    if (!connected) {
      invoke('join_server').then(() => {
        const connectionStatus = document.getElementById("connectionStatus") as HTMLSpanElement;
        connectionStatus.innerHTML = "Connected";
        connected = true;
        serverButton.innerHTML = "Leave server"
      })
    } else if (connected) {
      invoke('leave_server').then(() => {
        const connectionStatus = document.getElementById("connectionStatus") as HTMLSpanElement;
        connectionStatus.innerHTML = "Disconnected";
        const clientCounter = document.getElementById('clientCount') as HTMLSpanElement;
        clientCounter.style.setProperty('--value', "0");
        connected = false;
        serverButton.innerHTML = "Join server"
      })
    }
  });

  const playButton = document.getElementById('playButton') as HTMLButtonElement;
  playButton.addEventListener("click", async () => {
    if (connected) {
      //@ts-ignore
      let file: string | null = await open({
        multiple: false,
        filters: [{ name: 'Media', extensions: ['jpg', 'jpeg', 'png', 'gif', 'mp4', 'webm', 'ogg'] }]
      });
      if (file) {
        const content = await readFile(file);
        const type = await fileType.fileTypeFromBuffer(content);
        if (type && type.mime.startsWith('image/')) {
          const textInput = document.getElementById('textInput') as HTMLInputElement;
          const text = textInput.value;
          invoke('play_image', { path: file, text });
        } else if (type && type.mime.startsWith('video/')) {
          const textInput = document.getElementById('textInput') as HTMLInputElement;
          const text = textInput.value;
          invoke('play_video', { path: file, text });
        } else {
          console.error('Unsupported file type:', type ? type.mime : 'unknown');
        }
      }
    }
  });


  listen<JoinMessage>('updateClientCount', (data) => {
    const payload: JoinMessage = data.payload;
    const clientCounter = document.getElementById('clientCount') as HTMLSpanElement;
    clientCounter.style.setProperty('--value', payload.clientCount.toString());
  });
});

