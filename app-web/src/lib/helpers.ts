/**
 * Creates a simple URL-safe version of a given string by replacing
 * all sequences of non-alphanumeric text with '-'
 */
export function slugerize(str: string): string {
  return str.replace(/[^a-zA-Z0-9]+/g, '-')
}
