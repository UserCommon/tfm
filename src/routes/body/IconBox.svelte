<script>
    import { Card } from "flowbite-svelte";
	import { onDestroy } from "svelte";

    export let name;

    onDestroy(name);
</script>

<section class="p-3 col-span-1">
    <Card class="bg-slate-400 max-w-sm">
        <!-- Create widget -->
    </Card>
    <div class="grid place-content-center text-sm">
        {name}
    </div>
</section>