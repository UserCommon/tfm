import { writable } from 'svelte/store';
export let dir = writable("");
export let curr_path = writable([""]);
export let files_and_directories = writable([""]);
export let pretty_files_and_directories = writable([""]);
export let hidden_dot_files = writable(true);
//export let inner_width = writable(window.innerWidth);
export let cols = writable(1);