import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { debug, error, info, warn } from '@tauri-apps/plugin-log';
import { forwardConsole, forwardUnhandledRejection } from './log';


forwardConsole('log', debug);
forwardConsole('debug', debug);
forwardConsole('info', info);
forwardConsole('warn', warn);
forwardConsole('error', error);

forwardUnhandledRejection(error);

window.addEventListener("DOMContentLoaded", async () => {

  const joinButton = document.getElementById('joinServerButton') as HTMLButtonElement;
  joinButton.addEventListener('click', async () => {
    invoke('join_server');
  });

  const leaveButton = document.getElementById('leaveServerButton') as HTMLButtonElement;
  leaveButton.addEventListener('click', async () => {
    invoke('leave_server');
    const clientCounter = document.getElementById('clientCount') as HTMLSpanElement;
    clientCounter.style.setProperty('--value', "0");
  });

  const playImageButton = document.getElementById('playImageButton') as HTMLButtonElement;
  playImageButton.addEventListener('click', async () => {
    const textInput = document.getElementById('textInput') as HTMLInputElement;
    const text = textInput.value;
    invoke('play_image', { text });
  });

  const playVideoButton = document.getElementById('playVideoButton') as HTMLButtonElement;
  playVideoButton.addEventListener('click', async () => {
    const textInput = document.getElementById('textInput') as HTMLInputElement;
    const text = textInput.value;
    invoke('play_video', { text });
  });

  listen<JoinMessage>('updateClientCount', (data) => {
    const payload: JoinMessage = data.payload;
    const clientCounter = document.getElementById('clientCount') as HTMLSpanElement;
    clientCounter.style.setProperty('--value', payload.clientCount.toString());
  });
});

