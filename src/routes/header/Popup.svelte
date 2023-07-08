
<script>
    import {Label, Input } from 'flowbite-svelte';
	import { clickOutside } from '../funcs/funcs';
    import { is_key_listener_enabled, is_menu_visible } from '../stores';

    let input = "a";
    let myInput;

    function handleKeyDown() {
        console.log("AAH");
    }

    function onFocus() {
        $is_key_listener_enabled = false;
    }

    function outFocus() {
        $is_key_listener_enabled = true;
        $is_menu_visible = false;
    }
    // make non visible on esc button   
</script>


{#if $is_menu_visible}
<section class="justify-end">

    <div class='mb-6 border-3 '>
        <form on:submit={() => console.log(input)}>
            <input
                class = "border-orange-300 placeholder-orange-300"
                bind:value={input}
                on:focus={onFocus}
                on:focusout={outFocus}
            />
        </form>       
      </div>
</section>
{/if}