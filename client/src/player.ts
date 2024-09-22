import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { currentMonitor, PhysicalSize } from "@tauri-apps/api/window";

const message = {
  mediaUrl: "https://images.unsplash.com/photo-1726828581304-1bd8a2b90246?q=80&w=2070&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
  topMessage: "Ye",
  bottomMessage: "Lol",
  user: { username: "Pipou" },
  timeout: 8
}

window.addEventListener("DOMContentLoaded", async () => {
  const img = document.getElementById('img') as HTMLImageElement;
  img.addEventListener("load", async () => {
    const width = img.naturalWidth;
    const height = img.naturalHeight;

    let newWidth, newHeight;
    if (width > height) {
      newWidth = 400;
      newHeight = Math.floor((height / width) * 400);
    } else {
      newHeight = 400;
      newWidth = Math.floor((width / height) * 400);
    }

    console.log(`Image width: ${newWidth}, height: ${newHeight}`);
    const window = getCurrentWebviewWindow();
    const monitor = await currentMonitor()
    const scaleFactor = monitor!.scaleFactor;
    await window.setSize(new PhysicalSize(400, 400).toLogical(scaleFactor))
  })
  img.src = message.mediaUrl;

});
