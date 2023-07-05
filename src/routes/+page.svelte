<script>
    // TODO! Calculate grid cols (screen width / width of grid)  need to get window size(probably with tauri)
    // GET SCREEN SIZE import { currentMonitor } from '@tauri-apps/api/window';
    // https://tauri.app/v1/api/js/window/#currentmonitor
    import Header from "./header/Header.svelte";
    import Body from "./body/Body.svelte"
    import { get_home_path, get_breadcrumb_items, get_files_and_directories, prettify_files_and_directories } from './funcs/funcs';
    import { dir, curr_path, files_and_directories, pretty_files_and_directories,  } from "./stores";
    import { onDestroy, onMount } from "svelte";

    let path = "";
    let inner_width = 1280;
    onMount(async () => {
        dir.set(await get_home_path());
        dir.subscribe((n) => {path = n;});
        console.log(path);
        curr_path.set(await get_breadcrumb_items(path));

        files_and_directories.set(await get_files_and_directories(path));
        pretty_files_and_directories.set(await prettify_files_and_directories($files_and_directories));

        return () => {
            [path, inner_width];
        }
    });
    console.log(path);
    
    
</script>

<header>
    <Header/>
    <!-- <div>{$inner_width}</div> -->
</header>

<section>
    <Body/>
</section>

<style>
/* 	section {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		flex: 0.6;
	}

	h1 {
		width: 100%;
	}

	.welcome {
		display: block;
		position: relative;
		width: 100%;
		height: 0;
		padding: 0 0 calc(100% * 495 / 2048) 0;
	}

	.welcome img {
		position: absolute;
		width: 100%;
		height: 100%;
		top: 0;
		display: block;
	}*/
</style>
