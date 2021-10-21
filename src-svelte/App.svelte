<script>
    import Endpoint from './Endpoint.svelte';
    export let server, title;
    const fetchRoutes = (async () => {
		const response = await fetch(server + '/routes');
        // console.log(response);
        return await response.json();
	})();
</script>

<main>
    <h1>{title}</h1>
    {#await fetchRoutes}
        <p>...waiting</p>
    {:then routes}
        {#each routes as rt}
            <Endpoint 
                server={server} 
                name={rt.name} 
                method={rt.method} 
                type={rt.response_type} 
                path={rt.path}
            />
        {/each}
    {:catch error}
        <p>An error occurred!</p>
        <p>{error}</p>
    {/await}
</main>

<style>
    main {
        width: 100%;
        height: 100%;
    }
</style>