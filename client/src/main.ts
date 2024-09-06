import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { debug, error, info, warn } from '@tauri-apps/plugin-log';
import { forwardConsole, forwardUnhandledRejection } from './log';


forwardConsole('log', debug);
forwardConsole('debug', debug);
forwardConsole('info', info);
forwardConsole('warn', warn);
forwardConsole('error', error);

forwardUnhandledRejection(error);

window.addEventListener("DOMContentLoaded", async () => {

  const joinButton = document.getElementById('joinButton') as HTMLButtonElement;
  joinButton.addEventListener('click', async () => {
    invoke('join_server');
  });

  const testButton = document.getElementById('testButton') as HTMLButtonElement;
  testButton.addEventListener('click', async () => {
    // invoke('test_command');
    getCurrentWindow().hide();
  });

  const leaveButton = document.getElementById('leaveButton') as HTMLButtonElement;
  leaveButton.addEventListener('click', async () => {
    invoke('leave_server');
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
    clientCounter.innerHTML = payload.clientCount.toString();
  });
});

