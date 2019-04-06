/**
 * Utilities for handling session management
 */

function isAuthenticated(): boolean {
  return Boolean(document.cookie.replace(/(?:(?:^|.*;\s*)_manx_key\s*\=\s*([^;]*).*$)|^.*$/, '$1'));
}

export default {
  isAuthenticated,
};
