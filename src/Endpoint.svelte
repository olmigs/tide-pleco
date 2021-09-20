<script>
    import { sendFEN } from '../scripts/utils';
    export let server, name, type;
    let fen;
</script>

<div>
    <div>
        <input type="text" bind:value={fen}/>
        <button on:click={() => sendFEN(fen, server)}>{name} {type}</button>
    </div>
</div>