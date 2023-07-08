<script>
	import { onMount } from 'svelte';
// @ts-nocheck

    // TODO! Actually paths should be located in other places...
    import { curr_path, dir, files_and_directories, is_menu_visible } from '../stores';
    import { Button, Banner, Breadcrumb, BreadcrumbItem } from 'flowbite-svelte'
	import { clickOutside, get_files_and_directories } from '../funcs/funcs';
	import Popup from './Popup.svelte';
    
    function splitPath(path) {
        return path.split("/").slice(1);
    }


    // to use getNext we need to store hisory... bruh.
    function getPrev(path) {
        let v = path.split("/").slice(0);
        v.pop();
        return v.toString().replaceAll(",", "/");
    }

    async function goPrev() {
        if($dir.length > 1) {
            $dir = getPrev($dir);
            files_and_directories.set(await get_files_and_directories($dir)); // Not iterable wtf lol.
        }
    }

</script>

<section>
   
    <div class="header w-full">
        <div class="grid grid-cols-12 grid-rows-1 gap-3 m-2">
            <Button class="col-span-1" on:click={goPrev}> &lt </Button>
            <Button class="col-span-1"> &gt </Button>
            <Breadcrumb class="col-span-9">
                <!---/home/usr - должен быть  Home..--->
                {#each splitPath($dir) as path}
                    <BreadcrumbItem href="/{path}" >{path}</BreadcrumbItem>
                {/each}
            </Breadcrumb>
            <Button class="col-span-1" on:click={() => {$is_menu_visible = true}}>+</Button> 
            <Popup/>           
        </div>    
    </div>

</section>
