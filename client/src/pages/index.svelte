<script lang="ts">
  // https://github.com/improbable-eng/grpc-web/tree/master/client/grpc-web
  import { grpc } from "@improbable-eng/grpc-web";
  import { Empty } from "google-protobuf/google/protobuf/empty_pb";
  import type { UserSchema } from "../grpc-service/chat_pb";
  import { Chat } from "../grpc-service/chat_pb_service";
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
<h2><span>{name}</span></h2>
{#each users.map((u) => u) as user (user.getId())}
  <p>
    {`
      name: ${user.getName()},
      active: ${user.getActive()},
      createdAt: ${user.getCreatedAt()?.toDate()}
    `}
  </p>
{/each}

<!-- Resource -->
<!-- https://www.creative-tim.com/blog/web-development/svelte-examples-components-templates/ -->

<!-- EOL Template -->
<style lang="scss">
  h2 {
    background-color: blue;

    span {
      color: red;
    }
  }
</style>
