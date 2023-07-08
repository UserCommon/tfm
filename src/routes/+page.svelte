<script>
    import Header from "./header/Header.svelte";
    import Body from "./body/Body.svelte"
    import { get_home_path, get_breadcrumb_items, get_files_and_directories, prettify_files_and_directories, zip} from './funcs/funcs';
    import { dir, curr_path, files_and_directories, pretty_files_and_directories,  is_control_down, is_shift_down, keys_down} from "./stores";
    import { onDestroy, onMount } from "svelte";

    let path = "";
    onMount(async () => {
        dir.set(await get_home_path());
        dir.subscribe((n) => {path = n;});
        console.log(path);
        curr_path.set(await get_breadcrumb_items(path));
		
        files_and_directories.set(await get_files_and_directories(path));
		//console.log($files_and_directories);
        pretty_files_and_directories.set(await prettify_files_and_directories($files_and_directories));

        return () => {
            [path];
        }
    });

	function keydownEvents(event) {
		switch(event.key) {
			case "Control":
				$is_control_down = true;
				event.preventDefault();
				break;
			case "Shift":
				$is_shift_down = true;
				event.preventDefault();
				break;
			case "Delete":
				console.log("Delete func");
				// TODO: deletation arr of selected to function;
				break;

			default:
				// UB WARNING
				if (event.key.length == 1) {
					$keys_down[event.key.toLowerCase().charCodeAt(0) - 97] = 1;
					console.log($is_control_down, $is_shift_down, $files_and_directories);
				}
				event.preventDefault();
		}
		//console.log($keys_down, $is_control_down, $is_shift_down)

	}

	function keyupEvents(event) {
		switch(event.key) {
			case "Control":
				$is_control_down = false;
				event.preventDefault();
				break;
			case "Shift":
				$is_shift_down = false;
				event.preventDefault();
				break;
			default:
				// UB WARNING buuugg shift + a = A but we want to do... toLowerCase wontHelp
				if (event.key.length == 1) {
					$keys_down[event.key.toLowerCase().charCodeAt(0) - 97] = 0;
				}
				event.preventDefault();
		}
	}
    
	function handleKeys() {

		// There is the bug cause shift + char = CHAR and i dunno what to do
		//rewrite keyevent	
		if($keys_down[0] == 1 && $is_control_down && $is_shift_down) {
			$files_and_directories.forEach((element) => {
				element.selected = false;
			});
			$files_and_directories = $files_and_directories;;
			return;
		}

		if($keys_down[0] == 1 && $is_control_down) {
			$files_and_directories.forEach((element) => {
				element.selected = true;
			});
			$files_and_directories = $files_and_directories;;

			return;
		}
		//console.log($files_and_directories);
	}

	function onKeyDown(event) {
		keydownEvents(event);
		handleKeys();
	}
    
</script>

<svelte:window
	on:keydown={onKeyDown}
	on:keyup={keyupEvents}
/>

<header class="sticky top-2 bg-white">
    <Header/>
</header>

<section class="select-none">
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
