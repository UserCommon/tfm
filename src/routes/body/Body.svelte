<script>
	import { onMount } from "svelte";
    import { dir, curr_path, files_and_directories, pretty_files_and_directories, cols, iconbox_size  } from "../stores.js";
    import { get_files_and_directories, prettify_files_and_directories, get_grids, } from "../funcs/funcs.js"
    import { Button, Card } from "flowbite-svelte";
	import IconBox from "./IconBox.svelte";

    let files_and_dirs = $files_and_directories;
    function reassign() {
        // it's seems like it's not good decision to make this function...
        // However...
        files_and_dirs = $files_and_directories;
        console.log(files_and_dirs)
    }

    $: $files_and_directories, reassign();

    

    onMount(async () => {
        /* window.addEventListener('resize', resize);
        await resize();
        pretty = $pretty_files_and_directories; 

        return () => {
            window.removeEventListener('resize', resize);
        } */
    });
</script>

<section>
    <div class="justify-items-center grid grid-cols-4 sm:grid-cols-6 md:grid-cols-8 lg:grid-cols-10 xl:grid-cols-12 2xl:grid-cols-12 gap-3 m-3 justify-between">
        {#each files_and_dirs as file}
                <IconBox file={file}/>
        {/each}
    </div>
</section>

<style>

</style>