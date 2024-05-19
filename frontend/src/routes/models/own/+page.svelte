<svelte:options runes={true} />

<script lang="ts">
    import Model from "./model.svelte"
    const {data} = $props()

    const {models} = data
</script>

<div class="flex flex-col p-8 gap-8">
    {#each models as model}
    <Model {model} />
    {/each}
</div>
