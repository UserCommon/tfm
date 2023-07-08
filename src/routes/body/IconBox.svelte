<script>
// @ts-nocheck

    import { Card } from "flowbite-svelte";
	import { onDestroy } from "svelte";
    import { dir, files_and_directories, is_control_down } from "../stores";
	import { clickOutside, get_files_and_directories } from "../funcs/funcs";

    export let file;

    function prettify(path) {
        return path.split("/").pop();
    }


    function handleClick() {
        // FIXME: not optimal code O(n) for each click?

        $files_and_directories.forEach((element) => {
            // if not is_control down
            if($is_control_down == 0)
                element.selected = false; // wth reactivity?
            if(element === file) {
                element.selected = true;
            }
        });

        $files_and_directories = $files_and_directories;
        
    }

    function clear() {
        // Gross code, but i dunno FIXME:
        if(!$is_control_down) {
            $files_and_directories.forEach((element) => {
                element.selected = false;
            })
            $files_and_directories = $files_and_directories;
        }
    }

    async function changeDirectoryOrOpenFile(path) {
        $dir = path;
        console.log($dir);
        $files_and_directories = await get_files_and_directories($dir);
        console.log($files_and_directories);
    }
</script>


<section class="IconBox grow-0 h-24 w-16" on:dblclick={changeDirectoryOrOpenFile(file.path)} on:click={handleClick} use:clickOutside on:click_outside={clear}>
    <Card class="bg-gray-300">
        <!-- Create widget -->
    </Card>
    <div class="grid place-content-center text-sm break-all">
        {#if file.selected}
            <div class="bg-orange-400 rounded">
                {prettify(file.path)}
            </div>
        {:else}
            {prettify(file.path)}
        {/if}
    </div>
</section>

<style>

</style>