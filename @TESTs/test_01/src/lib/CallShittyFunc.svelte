<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";

    let someshit = "";
    let greetMsg = "";

    async function greet(someshit) {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        greetMsg = await invoke("command", { someshit });
    }
</script>

<pre>
    {someshit}
    {greetMsg}
</pre>
<br />

<div class="style: display:flex; border:1px solid aqua;">
    <input type="text" bind:value={someshit} />
    <button
        on:click={() => greet(someshit)}
        style="background-color: aquamarine; color:black"
        >Call Some Function</button
    >
</div>
