import { enUS } from 'date-fns/locale';
import { twMerge } from 'tailwind-merge';
import { type ClassValue, clsx } from 'clsx';
import { type DateArg, formatDistanceToNow } from 'date-fns';

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function since(timestamp: DateArg<Date>) {
  return formatDistanceToNow(timestamp, {
    locale: { formatDistance: enUS.formatDistance },
  });
}

export function handleError(err: unknown, throwErr = false) {
  if (err instanceof Error) {
    console.error(err.message);
    if (throwErr) throw err;
  }
}
