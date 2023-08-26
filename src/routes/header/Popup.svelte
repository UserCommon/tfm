
<script>
    import {Label, Input, Button } from 'flowbite-svelte';
	import { clickOutside, get_files_and_directories } from '../funcs/funcs';
    import { is_key_listener_enabled, is_menu_visible, dir, files_and_directories } from '../stores';
    import { invoke } from '@tauri-apps/api/tauri';

    let input = "";
    let state = "File";
    let myInput;


    function onFocus() {
        $is_key_listener_enabled = false;
    }

    function outFocus() {
        $is_key_listener_enabled = true;
        $is_menu_visible = false;
    }

    async function create(path, name) {
        let arg = {path: path, name: name};
        if(state == "File")
            await invoke('create_file', arg);
        if(state == "Dir")
            await invoke('create_dir', arg);
		files_and_directories.set(await get_files_and_directories($dir));
    }
    // make non visible on esc button   
</script>


{#if $is_menu_visible}
<section class = "bg-white p-2 rounded-md bg-opacity-75">
    <div class='mb-6 max-w-sm m-3'>
        <div class="select-mode grid grid-cols-2">
            <Button class="col-span-1" on:click={() => state = "Dir"}>Dir</Button>
            <Button class="col-span-1" on:click={() => state = "File"}>File</Button>
        </div>
        <form on:submit={create($dir, input)}>
            <input
                class = "border-2 border-orange-400 placeholder-orange-300 rounded-md w-full"
                bind:value={input}
                on:focus={onFocus}
                on:focusout={outFocus}
            />
        </form>       
      </div>
</section>
{/if}