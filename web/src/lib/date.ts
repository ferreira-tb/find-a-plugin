import { enUS } from 'date-fns/locale';
import { type DateArg, formatDistanceToNow } from 'date-fns';

export function since(timestamp: DateArg<Date>) {
  return formatDistanceToNow(timestamp, {
    locale: { formatDistance: enUS.formatDistance },
  });
}
