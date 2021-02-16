<script lang="ts">
  // https://github.com/improbable-eng/grpc-web/tree/master/client/grpc-web
  import { grpc } from "@improbable-eng/grpc-web";
  import { Empty } from "google-protobuf/google/protobuf/empty_pb";
  import type { UserSchema } from "./grpc-service/chat_pb";
  import { Chat } from "./grpc-service/chat_pb_service";
  import Children from "./components/Children.svelte";
  grpc.invoke<Empty, UserSchema, grpc.MethodDefinition<Empty, UserSchema>>(
    Chat.GetUsers,
    {
      host: "http://localhost:8080",
      onMessage(user) {
        users = [...users, user];
      },
      onEnd(code, status) {
        if (code) console.log(code, status);
        name = "Done!";
      },
      request: new Empty(),
    }
  );

  export let name: string;
  export let users: UserSchema[] = [];
</script>

<!-- Template -->
<h2>{name}</h2>
<Children />
{#each users.map((u) => u) as user (user.getId())}
  <p>
    {`
      name: ${user.getName()},
      active: ${user.getActive()},
      createdAt: ${user.getCreatedAt()?.toDate()}
    `}
  </p>
{/each}

<!-- EOL Template -->
<style>
  h2 {
    background-color: blue;
  }
</style>
