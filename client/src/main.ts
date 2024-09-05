import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { displayImage, displayVideo } from './displayMessage';

window.addEventListener("DOMContentLoaded", async () => {
  const joinButton = document.getElementById('joinButton') as HTMLButtonElement;
  joinButton.addEventListener('click', async () => {
    invoke('join_server');
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
    console.log(data)
    const payload: JoinMessage = data.payload;
    const clientCounter = document.getElementById('clientCount') as HTMLSpanElement;
    clientCounter.innerHTML = payload.clientCount.toString();
  });

  listen<PlayImageMessage>('playImage', (data) => {
    const payload: PlayImageMessage = data.payload;
    displayImage(payload);
  });

  listen<PlayVideoMessage>('playVideo', (data) => {
    const payload: PlayVideoMessage = data.payload;
    displayVideo(payload);
  });

});
