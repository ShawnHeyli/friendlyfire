import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

type PlayMessage = {
  media: PlayMessageMedia
}

type PlayMessageMedia = {
  url: string
}

window.addEventListener("DOMContentLoaded", async () => {
  
  const joinButton = document.getElementById('joinButton') as HTMLButtonElement;
  joinButton.addEventListener('click', async () => {
    invoke('join_server', { message: "can I join pwease ??" });
  })

  const playButton = document.getElementById('playButton') as HTMLButtonElement;
  playButton.addEventListener('click', async () => {
    invoke('send_play_message');
  })

  const unlistent = listen('updateClientCount',(payload) => {
    
  })

});

function clearMessage() {
  const element = document.getElementById('message') as HTMLDivElement;
  element.style.display = "none";
}

function generateImg(src: string) {
  return '<img id="message-img" ' + ' src="' + src + '" />';
}

function displayContent(message: PlayMessage) {
  var element = document.getElementById('message') as HTMLDivElement;

  element.innerHTML = '';
  element.innerHTML = generateImg(message.media.url);
}

var timeout: number | undefined;
function displayMessage(message: PlayMessage) {
  if (timeout) {
    clearTimeout(timeout);
  }

  timeout = setTimeout(() => {
    clearMessage()
  }, 4 * 1000);

  var elementMessage = document.getElementById('message') as HTMLDivElement;
  var elementText = document.getElementById('message-text') as HTMLParagraphElement;
  elementMessage.innerHTML = '';
  elementText.style.display = 'none';

  displayContent(message);
}
