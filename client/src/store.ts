import { Store } from "@tauri-apps/plugin-store";

export function initStoredValues(store: Store){
  const usernameInput = document.getElementById("usernameInput") as HTMLInputElement;
  const serverInput = document.getElementById("serverInput") as HTMLInputElement;

  usernameInput.addEventListener("change", async () => {
    await store.set("username", usernameInput.value);
    await store.save();
  })

  serverInput.addEventListener("change", async () => {
    await store.set("url", serverInput.value);
    await store.save();
  })
}

export async function restoreStoreValues(store: Store){
  const usernameInput = document.getElementById("usernameInput") as HTMLInputElement;
  const serverInput = document.getElementById("serverInput") as HTMLInputElement;

  const username = await store.get<string>('username');
  const url= await store.get<string>('url');

  if(username){
    usernameInput.value = username;
  }

  if(url){
    serverInput.value = url;
  }
}
