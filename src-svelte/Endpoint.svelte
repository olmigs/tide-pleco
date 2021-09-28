<script>
    import { endpointHandler } from '../scripts/utils';
    export let server, name, method, type, path;
    let value = '';
</script>

<div>
    <div>
        <h3>{name}</h3>
        <button on:click={() => endpointHandler(server, path, type, method, value)}>{method}</button>
        {#if method != 'GET'}
            <input type="text" bind:value={value}/>
        {/if}
    </div>
</div>