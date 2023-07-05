<script>
	import { onMount } from "svelte";
    import { dir, curr_path, files_and_directories, pretty_files_and_directories, cols  } from "../stores.js";
    import { get_files_and_directories, prettify_files_and_directories, get_grids } from "../funcs/funcs.js"
    import { Button, Card } from "flowbite-svelte";
	import IconBox from "./IconBox.svelte";

    // async function scale(val) {
        // - scale up + scale down
      //  cols += val;
    //}

    function log_files_and_dirs() {
        console.log($files_and_directories);
    }

   /*  onMount(async () => {
        window.addEventListener('resize', async (event) => {
            cols.set( await get_grids())
        });
        cols.set(await get_grids());
        console.log($cols);
    }); */

    
    // TODO!  1) write name under 2) Icon of file or of directory check file type
</script>

<section>

    <div class="body grid grid-cols-6 gap-4 m-3">
        {#each $pretty_files_and_directories as file_or_dir}
            <IconBox name="{file_or_dir}" />
        {/each}
    </div>
</section>