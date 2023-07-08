<script>
// @ts-nocheck

    import { Card } from "flowbite-svelte";
	import { onDestroy } from "svelte";
    import { files_and_directories, is_control_down } from "../stores";

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
                console.log(file);
            }
        });

        $files_and_directories = $files_and_directories;
        
    }

</script>


<section class="IconBox grow-0 h-24 w-16" on:click={handleClick}>
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