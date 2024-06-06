import { writable } from 'svelte/store';
import type { Writable } from 'svelte/store';
import type { Claims } from '../app';

export const navbarVisible = writable(true);
export const user: Writable<Claims | null> = writable(null);
