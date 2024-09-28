export function initUpdateAvatarPlaceHolder() {
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
