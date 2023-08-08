//
// import { goto } from '$app/navigation';
// import { ndClient } from '$lib/navidrome/client';
// import type { NDAlbum, NDAuthenticate } from '$lib/navidrome/types';
// import { derived, writable } from 'svelte/store';
//
// function createAlbumList() {
//   const { subscribe, set } = writable<NDAlbum[]>([]);
//
//   function loadUser() {
//     const user = localStorage.getItem('user');
//     if (user) {
//       setUser(JSON.parse(user));
//     }
//   }
//
//   function signOut() {
//     set(undefined);
//     localStorage.removeItem('user');
//     ndClient.signOut();
//     goto('/sign-in');
//   }
//
//   loadUser();
//   return {
//     subscribe,
//     loadUser,
//     setUser,
//     signOut,
//   };
// }
