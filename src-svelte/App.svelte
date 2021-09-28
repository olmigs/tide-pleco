<script>
    import routes from '../routes.json';
    import Endpoint from './Endpoint.svelte';
    export let server, title;
</script>

<main>
    <h1>{title}</h1>
    {#each routes as rt}
        <Endpoint 
            server={server} 
            name={rt.name} 
            method={rt.method} 
            type={rt.response_type} 
            path={rt.path}
        />
    {/each}
</main>

<style>
    main {
        width: 100%;
        height: 100%;
    }
</style>