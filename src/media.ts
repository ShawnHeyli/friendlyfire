import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { readFile } from "@tauri-apps/plugin-fs";
import { popAlert } from "./alert";
import { listen } from "@tauri-apps/api/event";

let file: string | null;

export function initMediaPreview() {
  const mediaInput = document.getElementById("mediaInput") as HTMLButtonElement;

  const messageTopInput = document.getElementById("messageTopInput") as HTMLInputElement;
  const messageBottomInput = document.getElementById("messageBottomInput") as HTMLInputElement;

  const topMessage = document.getElementById("topMessage") as HTMLSpanElement;
  const bottomMessage = document.getElementById("bottomMessage") as HTMLSpanElement;

  const sendMediaButton = document.getElementById("sendMediaButton") as HTMLButtonElement;
  sendMediaButton.classList.add("btn-disabled");

  mediaInput.addEventListener("click", async (event) => {
    event.preventDefault();
    file = await open({
      multiple: false,
      directory: false,
      filters: [{
        name: "Default",
        extensions: [
          "avif",
          "bmp",
          "dds",
          "exr",
          "ff",
          "gif",
          "hdr",
          "ico",
          "jpg",
          "jpeg",
          "png",
          "pnm",
          "qoi",
          "tga",
          "tiff",
          "webp",
        ]
      }]
    })

    if (file) {
      await enablePreview(file);
    }
  })

  messageTopInput.addEventListener("input", () => {
    topMessage.textContent = messageTopInput.value;
  });

  messageBottomInput.addEventListener("input", () => {
    bottomMessage.textContent = messageBottomInput.value;
  });
}

export async function initDropListener() {
  listen("tauri://drag-drop", (event) => {
    enablePreview((event.payload as { paths: string[] }).paths[0]);
  })
}

async function enablePreview(filepath: string) {
  const mediaPreview = document.getElementById("mediaPreview") as HTMLImageElement;
  const sendMediaButton = document.getElementById("sendMediaButton") as HTMLButtonElement;

  const contents = await readFile(filepath);
  const blob = new Blob([contents]);

  mediaPreview.src = URL.createObjectURL(blob);
  mediaPreview.style.display = "block";
  mediaPreview.addEventListener("load", () => {
    URL.revokeObjectURL(mediaPreview.src);
  })

  sendMediaButton.classList.remove("btn-disabled");
}

export function initSendMedia() {
  const usernameInput = document.getElementById('usernameInput') as HTMLInputElement;
  const sendMediaButton = document.getElementById("sendMediaButton") as HTMLButtonElement;

  const messageTopInput = document.getElementById("messageTopInput") as HTMLInputElement;
  const messageBottomInput = document.getElementById("messageBottomInput") as HTMLInputElement;
  const timeoutRange = document.getElementById("timeoutRange") as HTMLInputElement;

  sendMediaButton.addEventListener("click", async () => {
    await invoke("send_media", {
      filepath: file,
      topMessage: messageTopInput.value,
      bottomMessage: messageBottomInput.value,
      user: { username: usernameInput.value },
      timeout: parseInt(timeoutRange.value) * 1000
    }).then(() => {
      popAlert("success", "Media fired !!", null);
    }, (error) => {
      popAlert("error", "Error while sending media", error);
    })
  })
}

