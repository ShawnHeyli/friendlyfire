import { open } from "@tauri-apps/plugin-dialog";
import { readFile } from "@tauri-apps/plugin-fs";

export function initMediaPreview() {
  const mediaInput = document.getElementById("mediaInput") as HTMLButtonElement;
  const mediaPreview = document.getElementById("mediaPreview") as HTMLImageElement;

  const messageTopInput = document.getElementById("messageTopInput") as HTMLInputElement;
  const messageBottomInput = document.getElementById("messageBottomInput") as HTMLInputElement;

  const topMessage = document.getElementById("topMessage") as HTMLSpanElement;
  const bottomMessage = document.getElementById("bottomMessage") as HTMLSpanElement;

  const sendMediaButton = document.getElementById("sendMediaButton") as HTMLButtonElement;
  sendMediaButton.classList.add("btn-disabled");

  mediaInput.addEventListener("click", async (event) => {
    event.preventDefault();
    const file = await open({
      multiple: false,
      directory: false,
      filters: [{
        name: "Default",
        extensions: ['png', 'jpg', 'jpeg']
      }]
    })

    if (file) {
      mediaInput.setAttribute("data-title", "TNMT");
      const contents = await readFile(file);
      const blob = new Blob([contents]);

      mediaPreview.src = URL.createObjectURL(blob);
      mediaPreview.style.display = "block";
      mediaPreview.addEventListener("load", () => {
        URL.revokeObjectURL(mediaPreview.src);
      })

      sendMediaButton.classList.remove("btn-disabled");
    }
  })

  messageTopInput.addEventListener("input", () => {
    topMessage.textContent = messageTopInput.value;
  });

  messageBottomInput.addEventListener("input", () => {
    bottomMessage.textContent = messageBottomInput.value;
  });
}
