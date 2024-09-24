import { listen } from "@tauri-apps/api/event";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { LogicalSize } from "@tauri-apps/api/window";

type MediaMessage = {
  media_url: string,
  top_message: string,
  bottom_message: string,
  sender: User,
  timeout: number,
}

type User = {
  username: string
}

listen<MediaMessage>("ff://media_play", (event) => {
  const message = event.payload;
  console.log(message)
  const img = document.getElementById('mediaPreview') as HTMLImageElement;
  const topMessage = document.getElementById("topMessage") as HTMLSpanElement;
  const bottomMessage = document.getElementById("bottomMessage") as HTMLSpanElement;

  img.addEventListener("load", async () => {
    const width = img.naturalWidth;
    const height = img.naturalHeight;

    let newWidth = width;
    let newHeight = height;
    if (width > 400 | height > 400){
      if (width > height) {
        newWidth = 400;
        newHeight = Math.floor((height / width) * 400);
      } else {
        newHeight = 400;
        newWidth = Math.floor((width / height) * 400);
      }
    }

    const window = getCurrentWebviewWindow();
    await window.setSize(new LogicalSize(newWidth, newHeight))
    await window.show();

    setTimeout(async () => {
      topMessage.innerText = "";
      bottomMessage.innerText = "";
      img.src = "";
      await window.hide();
    }, message.timeout)
  })

  topMessage.innerText = message.top_message;
  bottomMessage.innerText = message.bottom_message;
  img.src = message.media_url;

})

