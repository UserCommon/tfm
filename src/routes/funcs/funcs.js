import { onDestroy } from 'svelte';

import { invoke } from '@tauri-apps/api/tauri';
import { curr_path, dir, hidden_dot_files, is_control_down, is_shift_down, keys_down } from '../stores';
import { currentMonitor } from '@tauri-apps/api/window';

// GUI
export async function get_grids() {
    // We need scale parameter consider that 256 is scale parameter by default
    return Math.trunc((window.innerWidth / 200)* (window.devicePixelRatio));
}

// File System
export async function get_home_path() {
    return await invoke('get_home');
}

/**
 * @param {string} path
 */
// FIXME: U dont need this function if u'll calculate splitted files on frontend...
export async function get_breadcrumb_items(path) {
    return await invoke('split_dir', {dir: path});
}

export async function paste(curr_path, file_path) {
  return await invoke('paste_file', {filePath : file_path, destPath : curr_path});
}

export async function get_files_and_directories(path) {
    return await invoke('ls', {path: path});
}

export async function open_file(path) {
  return await invoke('open_file', {path: path});
}

/* export async function prettify_files_and_directories(array_of_files_and_directories) {
    let arr = [];
    let el;
    let isHidden = false;
    hidden_dot_files.subscribe((val) => {isHidden = val;});

    array_of_files_and_directories.forEach((element) => {
        el = element.path.split("/").slice(-1)[0];
        if(el[0] != ".")
            arr.push(el);
    })

    return arr;
} */

export function clickOutside(node) {
  
    const handleClick = event => {
      if (node && !node.contains(event.target) && !event.defaultPrevented) {
        node.dispatchEvent(
          new CustomEvent('click_outside', node)
        )
      }
    }
  
      document.addEventListener('click', handleClick, true);
    
    return {
      destroy() {
        document.removeEventListener('click', handleClick, true);
      }
    }
}


export const zip = (a, b) => a.map((k, i) => [k, b[i]]);

export function arrayToString(arr) {
  return "/" + arr.join("/");
}
/* export async function ddo() {
    dir.set(await get_home_path());
    console.log(dir);
    await get_breadcrumb_items();
    console.log(curr_path);
} */