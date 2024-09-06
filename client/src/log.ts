export function forwardConsole(
  fnName: 'log' | 'debug' | 'info' | 'warn' | 'error',
  logger: (message: string) => Promise<void>
) {
  const original = console[fnName];
  console[fnName] = (message) => {
    original(message);
    logger(message);
  };
}

export function forwardUnhandledRejection(logger: (message: string) => Promise<void>) {
 window.addEventListener('unhandledrejection', async (event) => {
    event.preventDefault(); // Prevent the default browser handling
    const message = `Unhandled Promise Rejection: ${event.reason}`;
    await logger(message);
  });
};
