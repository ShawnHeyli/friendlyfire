import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

type PlayImageMessage = {
  remotePath: string
  text: string
}

type JoinMessage = {
  clientCount: number
}

window.addEventListener("DOMContentLoaded", async () => {

  const joinButton = document.getElementById('joinButton') as HTMLButtonElement;
  joinButton.addEventListener('click', async () => {
    invoke('join_server');
  })

  const leaveButton = document.getElementById('leaveButton') as HTMLButtonElement;
  leaveButton.addEventListener('click', async () => {
    invoke('leave_server');
  })

  const playButton = document.getElementById('playButton') as HTMLButtonElement;
  playButton.addEventListener('click', async () => {
    const textInput = document.getElementById('textInput') as HTMLInputElement
    const text = textInput.value;
    invoke('play_image', { text });
  })


  listen<JoinMessage>('updateClientCount', (data) => {
    const payload: JoinMessage = data.payload;
    const clientCounter = document.getElementById('clientCount') as HTMLSpanElement;
    clientCounter.innerHTML = payload.clientCount.toString();
  })

  listen<PlayImageMessage>('playImage', (data) => {
    const payload: PlayImageMessage = data.payload;
    console.log(payload);
    displayMessage(payload);
  })

});

function displayText(text: string) {
  var element = document.getElementById('message-text') as HTMLSpanElement; 
  element.style.display = 'none';

  if (text) {
    element.innerHTML = text;
    element.style.display = 'block';
  }
}

function clearMessage() {
  const element = document.getElementById('message') as HTMLDivElement;
  const element_text = document.getElementById('message-text') as HTMLDivElement;
  element.style.display = "none";
  element_text.style.display = "none";
}

function generateImg(src: string) {
  return '<img id="message-img" ' + ' src="' + src + '" />';
}

function displayContent(message: PlayImageMessage) {
  var element = document.getElementById('message') as HTMLDivElement;

  element.innerHTML = generateImg(message.remotePath);
}

var timeout: number | undefined;
function displayMessage(message: PlayImageMessage) {
  console.log(message);
  if (timeout) {
    clearTimeout(timeout);
  }

  timeout = setTimeout(() => {
    clearMessage()
  }, 8 * 1000);

  displayText(message.text)
  displayContent(message);
}
