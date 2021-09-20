import App from './App.svelte';

const app = new App({
    target: document.body,
    props: {
        server: 'http://localhost:8080',
        title: 'tide_pleco',
    },
});

export default app;
