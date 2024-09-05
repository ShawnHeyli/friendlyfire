import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { displayMessage } from './displayMessage';

window.addEventListener("DOMContentLoaded", async () => {
  setupJoinButton();
  setupLeaveButton();
  setupPlayButton();
  setupClientCountListener();
  setupPlayImageListener();
});

function setupJoinButton() {
  const joinButton = document.getElementById('joinButton') as HTMLButtonElement;
  joinButton.addEventListener('click', async () => {
    invoke('join_server');
  });
}

function setupLeaveButton() {
  const leaveButton = document.getElementById('leaveButton') as HTMLButtonElement;
  leaveButton.addEventListener('click', async () => {
    invoke('leave_server');
  });
}

function setupPlayButton() {
  const playButton = document.getElementById('playButton') as HTMLButtonElement;
  playButton.addEventListener('click', async () => {
    const textInput = document.getElementById('textInput') as HTMLInputElement;
    const text = textInput.value;
    invoke('play_image', { text });
  });
}

function setupClientCountListener() {
  listen<JoinMessage>('updateClientCount', (data) => {
    const payload: JoinMessage = data.payload;
    const clientCounter = document.getElementById('clientCount') as HTMLSpanElement;
    clientCounter.innerHTML = payload.clientCount.toString();
  });
}

function setupPlayImageListener() {
  listen<PlayImageMessage>('playImage', (data) => {
    const payload: PlayImageMessage = data.payload;
    console.log(payload);
    displayMessage(payload);
  });
}
