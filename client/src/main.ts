import App from './App.svelte';

const app = new App({
	target: document.body,
	props: {
		name: 'GRPC SERVER STREAM DEMO'
	}
});

export default app;
