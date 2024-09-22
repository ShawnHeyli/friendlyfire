export function initMediaPreview() {
  const mediaInput = document.getElementById("mediaInput") as HTMLInputElement;
  const mediaPreview = document.getElementById("mediaPreview") as HTMLImageElement;

  const messageTopInput = document.getElementById("messageTopInput") as HTMLInputElement;
  const messageBottomInput = document.getElementById("messageBottomInput") as HTMLInputElement;

  const topMessage = document.getElementById("topMessage") as HTMLSpanElement;
  const bottomMessage = document.getElementById("bottomMessage") as HTMLSpanElement;

  const sendMediaButton = document.getElementById("sendMediaButton") as HTMLButtonElement;
  sendMediaButton.classList.add("btn-disabled");

  mediaInput.addEventListener("change", () => {
    mediaPreview.style.display = "block";
    const file = mediaInput!.files![0];
    if (file) {
      mediaPreview.src = URL.createObjectURL(file);
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
