import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { readFile } from "@tauri-apps/plugin-fs";
import { popAlert } from "./alert";
import { listen } from "@tauri-apps/api/event";
import clipboard from "tauri-plugin-clipboard-api";

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
      const contents = await readFile(file);
      const blob = new Blob([contents]);
      await enablePreview(URL.createObjectURL(blob));
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
  listen("tauri://drag-drop", async (event) => {
    const contents = await readFile((event.payload as { paths: string[] }).paths[0]);
    const blob = new Blob([contents]);
    enablePreview(URL.createObjectURL(blob));
  })
}

export async function initPasteListener() {
  document.addEventListener("paste", async (_event) => {
    const binaryImage = await clipboard.readImageBinary('Uint8Array') as Uint8Array;
    const blob = new Blob([binaryImage]);
    enablePreview(URL.createObjectURL(blob));
  })
}


async function enablePreview(src: string) {
  const mediaPreview = document.getElementById("mediaPreview") as HTMLImageElement;
  const sendMediaButton = document.getElementById("sendMediaButton") as HTMLButtonElement;

  mediaPreview.src = src;
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

