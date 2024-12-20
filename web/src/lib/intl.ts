const sortCollator = new Intl.Collator(undefined, {
  numeric: true,
  sensitivity: 'variant',
  usage: 'sort',
});

export function compare(a: string, b: string): number {
  return sortCollator.compare(a, b);
}

const integerIntl = new Intl.NumberFormat('default', {
  maximumFractionDigits: 0,
  minimumFractionDigits: 0,
  roundingMode: 'trunc',
  roundingPriority: 'lessPrecision',
  style: 'decimal',
});

export function formatInteger(num: number | bigint) {
  return integerIntl.format(num);
}
