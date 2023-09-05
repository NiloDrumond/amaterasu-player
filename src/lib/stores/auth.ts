import { goto } from '$app/navigation';
import { ndClient } from '$lib/navidrome/client';
import type { NDAuthenticate } from '$lib/navidrome/types';
import { derived, writable } from 'svelte/store';

const STORAGE_KEY = 'user';
function createUser() {
  const { subscribe, set } = writable<NDAuthenticate | undefined>(undefined);

  async function signIn(user: NDAuthenticate) {
    ndClient.setAuth(user, () => {
      signOut();
    });
    set(user);
  }

  async function authenticate(args: { username: string; password: string }) {
    const user = await ndClient.authenticate(args);
    signIn(user);
  }

  function loadUser(): void {
    const user = localStorage.getItem(STORAGE_KEY);
    if (user) {
      const parsed = JSON.parse(user) as NDAuthenticate;
      signIn(parsed);
    }
  }

  function signOut(): void {
    set(undefined);
    localStorage.removeItem(STORAGE_KEY);
    ndClient.signOut();
    goto('/sign-in');
  }

  loadUser();
  return {
    authenticate,
    subscribe,
    loadUser,
    signOut,
  };
}

export const user = createUser();

user.subscribe((val) => {
  if (val) {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(val));
  }
});

export const auth = derived(user, ($user) => {
  if (!$user) {
    goto('/sign-in');
    throw new Error('Inside /app without user');
  }
  return $user;
});
