<script lang="ts">
import {onMount} from "svelte";

import * as wasm from "$rust/rust.js";


onMount(() => {
    wasm.greet("loveliness");
});
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
