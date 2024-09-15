import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { debug, error, info, warn } from '@tauri-apps/plugin-log';
import { forwardConsole, forwardUnhandledRejection } from './log';
import { fetch } from '@tauri-apps/plugin-http';


forwardConsole('log', debug);
forwardConsole('debug', debug);
forwardConsole('info', info);
forwardConsole('warn', warn);
forwardConsole('error', error);

forwardUnhandledRejection(error);

window.addEventListener("DOMContentLoaded", async () => {

  const forwardDot = document.getElementById('forwardDot') as HTMLSpanElement
  const backDot = document.getElementById('backDot') as HTMLSpanElement
  setInterval(function() {
    backDot.classList.add('animate-ping');
    fetch('http://localhost:7331/healthcheck')
      .then(response => {
        if (response.ok) {
          // Server is up, pulse the status dot
          forwardDot.classList.remove('bg-gray-500');
          forwardDot.classList.remove('bg-red-500');
          forwardDot.classList.add('bg-green-500');
          backDot.classList.remove('bg-gray-400');
          forwardDot.classList.remove('bg-red-400');
          backDot.classList.add('bg-green-400');
          setTimeout(() => {
            backDot.classList.remove('animate-ping');
          }, 1000); // Remove the pulse after 1 second
        } else {
          // Server is down, set the status dot to red
          forwardDot.classList.remove('bg-gray-500');
          forwardDot.classList.remove('bg-green-500');
          forwardDot.classList.add('bg-red-500');
          backDot.classList.remove('bg-gray-400');
          forwardDot.classList.remove('bg-green-400');
          backDot.classList.add('bg-red-400');
          setTimeout(() => {
            backDot.classList.remove('animate-ping');
          }, 1000); // Remove the pulse after 1 second
        }
      })
      .catch(_error => {
        // Error occurred, assume server is down
        forwardDot.classList.remove('bg-gray-500');
        forwardDot.classList.remove('bg-green-500');
        forwardDot.classList.add('bg-red-500');
        backDot.classList.remove('bg-gray-400');
        backDot.classList.remove('bg-green-400');
        backDot.classList.add('bg-red-400');
        setTimeout(() => {
          backDot.classList.remove('animate-ping');
        }, 1000); // Remove the pulse after 1 second
      });
  }, 3000);

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

