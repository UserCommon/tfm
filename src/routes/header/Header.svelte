<script>
// @ts-nocheck

    // TODO! Actually paths should be located in other places...
    import { curr_path } from '../stores';
    import { Button, Banner, Breadcrumb, BreadcrumbItem } from 'flowbite-svelte'

    let splitted_paths = [""];

    curr_path.subscribe((paths) => splitted_paths = paths);
</script>

<section>
   
    <div class="header w-full">
        <div class="grid grid-cols-12 grid-rows-1 gap-3 m-2">
            <Button class="col-span-1"> &lt </Button>
            <Button class="col-span-1"> &gt </Button>
            <Breadcrumb class="col-span-9">
                <!---/home/usr - должен быть  Home..--->
                {#each splitted_paths as path }
                    <BreadcrumbItem href="/{path}" >{path}</BreadcrumbItem>
                {/each}
            </Breadcrumb>
            <Button class="col-span-1">+</Button>            
        </div>    
    </div>

</section>
