<script>
    import { endpointHandler } from '../scripts/utils';
    export let server, name, method, type;
    let value = '';
</script>

<div>
    <div>
        <button on:click={() => endpointHandler(server, name, type, method, value)}>{name} {method}</button>
        {#if method != 'GET'}
            <input type="text" bind:value={value}/>
        {/if}
    </div>
</div>