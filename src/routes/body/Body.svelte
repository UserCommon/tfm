<script>
	import { onMount } from "svelte";
    import { dir, curr_path, files_and_directories, pretty_files_and_directories, cols, iconbox_size, is_dot_visible  } from "../stores.js";
    import { clickOutside, get_files_and_directories, get_grids, } from "../funcs/funcs.js"
    import { Button, Card } from "flowbite-svelte";
	import IconBox from "./IconBox.svelte";

</script>



<section>
    <div class="relative justify-items-center grid grid-cols-4 sm:grid-cols-6 md:grid-cols-8 lg:grid-cols-10 xl:grid-cols-12 2xl:grid-cols-12 gap-3 m-3 justify-between">
        {#each $files_and_directories as file}
            {#if file.is_dot == 0 || $is_dot_visible }
                <IconBox file={file}/>
            {/if}
        {/each}
    </div>
</section>

<style>

</style>