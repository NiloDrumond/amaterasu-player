import { writable } from 'svelte/store';
import type { SignInSchema } from './data';

export const signin = writable<Partial<SignInSchema>>({});
