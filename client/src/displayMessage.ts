function displayText(text: string) {
  var element = document.getElementById('message-text') as HTMLSpanElement; 
  element.style.display = 'none';

  if (text) {
    element.innerHTML = text;
    element.style.display = 'block';
  }
}

function clearMessage() {
  const element = document.getElementById('message') as HTMLDivElement;
  const element_text = document.getElementById('message-text') as HTMLDivElement;
  element.style.display = "none";
  element_text.style.display = "none";
}

function generateImg(src: string) {
  return '<img id="message-img" ' + ' src="' + src + '" />';
}

function displayContent(message: PlayImageMessage) {
  var element = document.getElementById('message') as HTMLDivElement;

  element.innerHTML = generateImg(message.remotePath);
}

var timeout: number | undefined;
export function displayMessage(message: PlayImageMessage) {
  console.log(message);
  if (timeout) {
    clearTimeout(timeout);
  }

  timeout = setTimeout(() => {
    clearMessage()
  }, 8 * 1000);

  displayText(message.text)
  displayContent(message);
}
