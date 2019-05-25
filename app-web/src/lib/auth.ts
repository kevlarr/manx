/**
 * Utilities for handling session management
 */

function isAuthenticated(): boolean {
  const cookie = document.cookie.replace(/(?:(?:^|.*;\s*)manx\s*\=\s*([^;]*).*$)|^.*$/, '$1');
  return !!cookie && cookie.includes('user_id');
}

export default {
  isAuthenticated,
};
