import { getServerDomain } from "./server";
import { fetch } from "@tauri-apps/plugin-http";

export function initPingStatus() {
  const endpoint = "https://" + getServerDomain() + "/healthcheck";
  pingStatusDot(endpoint)
  setInterval(() => {
    const endpoint = "https://" + getServerDomain() + "/healthcheck";
    pingStatusDot(endpoint)
  }, 3000)
}

async function pingStatusDot(endpoint: string) {
  const forwardDot = document.getElementById('forwardDot') as HTMLSpanElement
  const backDot = document.getElementById('backDot') as HTMLSpanElement
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
}
