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

window.addEventListener("DOMContentLoaded", () => {
  initStatusDot("http://localhost:7331/healthcheck", 3000)
});
