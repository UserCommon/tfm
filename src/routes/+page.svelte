<script>
    import Header from "./header/Header.svelte";
    import Body from "./body/Body.svelte"
    import { clickOutside, get_home_path, get_breadcrumb_items, get_files_and_directories, zip, paste, arrayToString	} from './funcs/funcs';
    import { dir, curr_path, files_and_directories, pretty_files_and_directories,  is_control_down, is_shift_down, keys_down, is_key_listener_enabled, is_menu_visible, is_dot_visible, on_copy} from "./stores";
    import { onDestroy, onMount } from "svelte";
	import { invoke } from "@tauri-apps/api/tauri";

    let path = "";

    onMount(async () => {
        dir.set(await get_home_path());
        dir.subscribe((n) => {path = n;});
        console.log(path);
        curr_path.set(await get_breadcrumb_items(path));
		
        files_and_directories.set(await get_files_and_directories(path));
		//console.log($files_and_directories);

        return () => {
            [path];
        }
    });

	async function deleteFileOrDir() {
		$files_and_directories.forEach(async (element) => {
			if(element.selected) {
				await invoke("delete", {file: element});
			}
		});
		files_and_directories.set(await get_files_and_directories($dir));
	}

	function keydownEvents(event) {
		if($is_key_listener_enabled) {
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
					deleteFileOrDir();
					event.preventDefault();
					break;
				case "Escape":
					console.log("ESC");
					$is_key_listener_enabled = true;
					$is_menu_visible = false;
					event.preventDefault();

				default:
					// UB WARNING
					if (event.key.length == 1) {
						$keys_down[event.key.toLowerCase().charCodeAt(0) - 97] = 1;
						//console.log($is_control_down, $is_shift_down, $files_and_directories);
					}
					event.preventDefault();
			}
			//console.log($keys_down, $is_control_down, $is_shift_down)
		}
	}

	function keyupEvents(event) {
		if($is_key_listener_enabled) {
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
	}
    
	function handleKeys() {

		// There is the bug cause shift + char = CHAR and i dunno what to do
		//rewrite keyevent	

		// DESELECT CTRL+SHIFT+A
		if($keys_down[0] == 1 && $is_control_down && $is_shift_down) {
			$files_and_directories.forEach((element) => {
				element.selected = false;
			});
			$files_and_directories = $files_and_directories;
			return;
		}

		// SELECT ALL CTRL+A
		if($keys_down[0] == 1 && $is_control_down) {
			$files_and_directories.forEach((element) => {
				element.selected = true;
			});
			$files_and_directories = $files_and_directories;
			return;
		}
		
		// COPY CTRL+C
		if($is_control_down && $keys_down[2] == 1) {
			console.log("ctrl+c");
			$on_copy = [];
			$files_and_directories.forEach((element) => {
				if(element.selected)
					$on_copy.push(element);
			});
			console.log($on_copy);
		}

		// INSERT CTRL+V
		if($is_control_down && $keys_down[21]) {
			console.log("ctrl+v");
			$on_copy.forEach(async (element) => {
				await paste($dir, element.path);
				console.log(element.path);
				files_and_directories.set(await get_files_and_directories($dir));

			});
		}

		// HIDE/SHOW DOTFILES CTRL+H
		if ($keys_down[7] && $is_control_down) {
			$is_dot_visible = !$is_dot_visible;
		}
	}

	function onKeyDown(event) {
		keydownEvents(event);
		handleKeys();
	}
    
	function clear() {
		console.log("penis")
	}

</script>


<svelte:window
	on:keydown={onKeyDown}
	on:keyup={keyupEvents}
/>

<header class="z-20 sticky top-2 bg-white">
    <Header/>
</header>

<section class="select-none z-10">
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
