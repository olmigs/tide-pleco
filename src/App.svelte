<script>
    import Endpoint from './Endpoint.svelte';
    export let server;
</script>

<main>
    <Endpoint server={server} name="set" type="PUT" />
</main>

<style>
    main {
        width: 100%;
        height: 100%;
    }
</style>