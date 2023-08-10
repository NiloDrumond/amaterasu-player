import { goto } from '$app/navigation';
import { ndClient } from '$lib/navidrome/client';
import type { NDAuthenticate } from '$lib/navidrome/types';
import { derived, writable } from 'svelte/store';

function createUser() {
  const { subscribe, set } = writable<NDAuthenticate | undefined>(undefined);

  function setUser(user: NDAuthenticate) {
    set(user);
  }

  function loadUser() {
    const user = localStorage.getItem('user');
    if (user) {
      const parsed = JSON.parse(user) as NDAuthenticate;
      set(parsed);
      ndClient.setAuth(parsed);
    }
  }

  function signOut() {
    set(undefined);
    localStorage.removeItem('user');
    ndClient.signOut();
    goto('/sign-in');
  }

  loadUser();
  return {
    subscribe,
    loadUser,
    setUser,
    signOut,
  };
}

export const user = createUser();
user.subscribe((val) => {
  if (val) {
    localStorage.setItem('user', JSON.stringify(val));
  }
});

export const auth = derived(user, ($user) => {
  if (!$user) {
    goto('/sign-in');
    throw new Error('Inside /app without user');
  }
  return $user;
});
