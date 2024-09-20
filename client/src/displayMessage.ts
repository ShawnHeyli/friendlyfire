import { currentMonitor, getCurrentWindow, LogicalPosition, LogicalSize } from "@tauri-apps/api/window";
import { moveWindow, Position } from '@tauri-apps/plugin-positioner';

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
  const window = getCurrentWindow();
  window.hide();
}

function generateImage(src: string) {
  var element = document.getElementById('message') as HTMLDivElement;
  element.style.display = 'block';
  element.innerHTML = '<img id="message-img" ' + ' src="' + src + '" />'
}

function generateVideo(src: string) {
  var element = document.getElementById('message') as HTMLDivElement;
  element.style.display = 'block';
  element.innerHTML = '<video id="message-video" src="' + src + '" />';

  const video = document.getElementById("message-video") as HTMLVideoElement;
  video.play()
  video.addEventListener("ended", () => {
    clearMessage();
  });
}

let timeout: number | NodeJS.Timeout | undefined;

export async function displayImage(message: PlayImageMessage) {
  if (timeout) {
    clearTimeout(timeout);
  }

  timeout = setTimeout(() => {
    clearMessage()
  }, 8 * 1000);

  const window = getCurrentWindow();

  window.setSize(new LogicalSize(message.width, message.height));
  moveWindow(Position.TopRight);
  window.show()

  displayText(message.text)
  generateImage(message.remotePath);
}

export async function displayVideo(message: PlayVideoMessage) {
  const window = getCurrentWindow();

  window.setSize(new LogicalSize(message.width, message.height));
  moveWindow(Position.TopRight);
  window.show()

  displayText(message.text)
  generateVideo(message.remotePath);
}
