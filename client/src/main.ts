import { invoke } from '@tauri-apps/api/core';

window.addEventListener("DOMContentLoaded", async () => {

  const joinButton = document.getElementById('joinButton') as HTMLButtonElement;
  joinButton.addEventListener('click', async () => {
    invoke('send_ws_message', {message: "can I join pwease ??"});
  })
  
  const uploadButton = document.getElementById('uploadButton') as HTMLButtonElement;
  uploadButton.addEventListener('click', async () => {
    invoke('upload_file');
  })
});

