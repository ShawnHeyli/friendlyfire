import { invoke } from "@tauri-apps/api/core";
import { fetch } from "@tauri-apps/plugin-http";

async function initStatusDot(endpoint: string, interval: number) {
  const forwardDot = document.getElementById('forwardDot') as HTMLSpanElement
  const backDot = document.getElementById('backDot') as HTMLSpanElement
  setInterval(function() {
    backDot.classList.add('animate-ping');
    fetch(endpoint)
      .then(response => {
        if (response.ok) {
          // Server is up, pulse the status dot
          forwardDot.classList.remove('bg-gray-500');
          forwardDot.classList.remove('bg-red-500');
          forwardDot.classList.add('bg-green-500');
          backDot.classList.remove('bg-gray-400');
          backDot.classList.remove('bg-red-400');
          backDot.classList.add('bg-green-400');
          setTimeout(() => {
            backDot.classList.remove('animate-ping');
          }, 600); // Remove the pulse after interval
        } else {
          // Server is down, set the status dot to red
          forwardDot.classList.remove('bg-gray-500');
          forwardDot.classList.remove('bg-green-500');
          forwardDot.classList.add('bg-red-500');
          backDot.classList.remove('bg-gray-400');
          backDot.classList.remove('bg-green-400');
          backDot.classList.add('bg-red-400');
          setTimeout(() => {
            backDot.classList.remove('animate-ping');
          }, 600); // Remove the pulse after interval
        }
      })
      .catch(_error => {
        // Error occurred, assume server is down
        forwardDot.classList.remove('bg-gray-500');
        forwardDot.classList.remove('bg-green-500');
        forwardDot.classList.add('bg-red-500');
        backDot.classList.remove('bg-gray-400');
        backDot.classList.remove('bg-green-400');
        backDot.classList.add('bg-red-400');
        setTimeout(() => {
          backDot.classList.remove('animate-ping');
        }, 600); // Remove the pulse after interval
      });
  }, interval);

}

function initUpdateAvatarPlaceHolder() {
  const usernameInput = document.getElementById('usernameInput') as HTMLInputElement;
  const avatarPlaceholder = document.getElementById('avatarLetter') as HTMLSpanElement;

  usernameInput.addEventListener("input", () => {
    const username = usernameInput.value.trim();
    if (username.length > 0) {
      const words = username.split(" ");
      const initials = words.slice(0, 2).map(word => word[0]).join(''); // Takes the first letter of two words
      avatarPlaceholder.textContent = initials.toUpperCase();
    } else {
      avatarPlaceholder.textContent = '';
    }
  })
}

function initMediaPreview() {
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

function initServerToggle(){
  const serverToggle = document.getElementById("serverToggle") as HTMLButtonElement;
  const serverInput = document.getElementById("serverInput") as HTMLInputElement;

  serverToggle.addEventListener("click", () => {
    invoke("connect_to_server", {url: serverInput.value})
  })
}

window.addEventListener("DOMContentLoaded", () => {
  initStatusDot("http://localhost:7331/healthcheck", 3000);
  initUpdateAvatarPlaceHolder();
  initMediaPreview();
  initServerToggle();
});
