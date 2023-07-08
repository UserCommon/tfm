import { writable } from 'svelte/store';

// FIXME: dir and curr_path 1) variables name is strange \
//2) can we get rid of array of string by using function,
// that getting iter out of string?
export let dir = writable("");
export let curr_path = writable([""]);
// FIXME:

export let files_and_directories = writable([]);

// FIXME: pretty_files_and_directories. no need in them.
export let pretty_files_and_directories = writable([""]);
export let hidden_dot_files = writable(true);
//export let inner_width = writable(window.innerWidth);
export let iconbox_size = 200;
// Row capacity
export let cols = writable(Math.trunc(window.innerWidth / iconbox_size) * (window.devicePixelRatio));
// Page -> selected_files -> select in frontend

export let is_control_down = writable(false);
export let is_shift_down   = writable(false);
export let keys_down 	   = writable(Array(26).fill(0));

export let is_menu_visible = writable(false);
export let is_key_listener_enabled = writable(true);