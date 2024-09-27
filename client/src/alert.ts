export function popAlert(level: "info" | "success" | "warning" | "error", message: string, description: string | null) {
  switch (level) {
    case 'info':
      popInfoAlert(message, description);
      break;
    case 'success':
      popSuccessAlert(message, description);
      break;
    case 'warning':
      popWarningAlert(message, description);
      break;
    case 'error':
      popErrorAlert(message, description);
      break;
    default:
      console.error('Invalid alert level');
  }
}

function createAlertDiv(className: string, svgPath: string, message: string, description: string | null) {
  const alertDiv = document.createElement('div');
  alertDiv.setAttribute('role', 'alert');
  alertDiv.classList.add('alert', className, 'shadow-lg');

  const svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
  svg.setAttribute('xmlns', 'http://www.w3.org/2000/svg');
  svg.setAttribute('fill', 'none');
  svg.setAttribute('viewBox', '0 0 24 24');
  svg.classList.add('h-6', 'w-6', 'shrink-0', 'stroke-current');

  const path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
  path.setAttribute('stroke-linecap', 'round');
  path.setAttribute('stroke-linejoin', 'round');
  path.setAttribute('stroke-width', '2');
  path.setAttribute('d', svgPath);

  svg.appendChild(path);

  if (description) {
    const contentDiv = document.createElement('div');

    const h3 = document.createElement('h3');
    h3.classList.add('font-bold');
    h3.textContent = message;

    const descriptionDiv = document.createElement('div');
    descriptionDiv.classList.add('text-xs');
    descriptionDiv.textContent = description;

    contentDiv.appendChild(h3);
    contentDiv.appendChild(descriptionDiv);

    alertDiv.appendChild(contentDiv);
  } else {
    // Create the span element
    const span = document.createElement('span');
    span.textContent = message;
    alertDiv.appendChild(span);
  }

  alertDiv.appendChild(svg);

  document.getElementById("notifications")!.appendChild(alertDiv);
  alertDiv.addEventListener("click", () => {
    alertDiv.classList.add("hide")
    setTimeout(() => alertDiv.remove(), 500)
  })
  setTimeout(() => {
    alertDiv.classList.add("hide")
    setTimeout(() => alertDiv.remove(), 500)
  }, 5000)
}

function popInfoAlert(message: string, description: string | null) {
  const svgPath = 'M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z';
  createAlertDiv('alert-info', svgPath, message, description);
}

function popSuccessAlert(message: string, description: string | null) {
  const svgPath = 'M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z';
  createAlertDiv('alert-success', svgPath, message, description);
}

function popWarningAlert(message: string, description: string | null) {
  const svgPath = 'M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z';
  createAlertDiv('alert-warning', svgPath, message, description);
}

function popErrorAlert(message: string, description: string | null) {
  const svgPath = 'M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z';
  createAlertDiv('alert-error', svgPath, message, description);
}
