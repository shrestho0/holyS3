<script lang="ts">
    export let paths: String[] = [];
</script>

{#if paths}
    <ul>
        {#each paths as file}
            {file}
        {/each}
    </ul>
{/if}
