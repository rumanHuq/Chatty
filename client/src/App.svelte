<script lang="ts">
  // https://github.com/improbable-eng/grpc-web/tree/master/client/grpc-web
  import { grpc } from "@improbable-eng/grpc-web";
  import { Empty } from "google-protobuf/google/protobuf/empty_pb";
  import { UserSchema } from "./grpc-service/chat_pb";
  import { Chat } from "./grpc-service/chat_pb_service";

  grpc.invoke<Empty, UserSchema, grpc.MethodDefinition<Empty, UserSchema>>(
    Chat.GetUsers,
    {
      host: "http://localhost:8080",
      onMessage(user) {
        users = [...users, user];
      },
      onEnd(code, status) {
        console.log(code, status);
      },
      request: new UserSchema(),
    }
  );

  export let name: string = "GRPC SERVER STREAM DEMO";
  export let users: UserSchema[] = [];
</script>

<!-- Template -->
<h2>{name}</h2>
{#each users.map((u) => u) as user (user.getId())}
  <p>{`id: ${user.getName()}`}</p>
  <p>{user.getActive()}</p>
  <p>{user.getCreatedAt().toDate()}</p>
{/each}
<!-- EOL Template -->
