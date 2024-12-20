export function handleError(err: unknown, throwErr = false) {
  if (err instanceof Error) {
    console.error(err.message);
    if (throwErr) throw err;
  }
}
