<script lang="ts">
	import { grpc } from "@improbable-eng/grpc-web";
	import { Chat } from "./grpc-service/chat_pb_service";
	import { UserInput } from "./grpc-service/chat_pb";
	export let name: string;
	const host = "http://localhost:8080";
	const userInput = new UserInput();
	userInput.setName("ruman");
	userInput.setActive(3);

	grpc.unary(Chat.CreateUser, {
		host,
		request: userInput,
		onEnd(res) {
			console.log(res);
		},
	});
</script>

<main>
	<h1>Hello {name}!</h1>
	<p>
		Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to learn
		how to build Svelte apps.
	</p>
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>
