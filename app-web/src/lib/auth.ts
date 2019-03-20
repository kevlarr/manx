/**
 * Utilities for handling session management, including interacting
 * with localStorage
 */

import { ParsedResponse } from '@/lib/api';

const AUTH = 'authUser';

function isAuthenticated(): boolean {
  return localStorage.getItem(AUTH) !== null;
}

function signIn({ body }: ParsedResponse) {
  localStorage.setItem(AUTH, JSON.stringify({
    organizations: body.organizations || [],
  }));
}

function signOut() {
  localStorage.removeItem(AUTH);
}

export default {
  isAuthenticated,
  signIn,
  signOut,
};
