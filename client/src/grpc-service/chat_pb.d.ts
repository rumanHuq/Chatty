// package: chat
// file: chat.proto

import * as jspb from "google-protobuf";
import * as google_protobuf_empty_pb from "google-protobuf/google/protobuf/empty_pb";
import * as google_protobuf_timestamp_pb from "google-protobuf/google/protobuf/timestamp_pb";

export class UserInput extends jspb.Message {
  getName(): string;
  setName(value: string): void;

  getActive(): ActiveMap[keyof ActiveMap];
  setActive(value: ActiveMap[keyof ActiveMap]): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserInput.AsObject;
  static toObject(includeInstance: boolean, msg: UserInput): UserInput.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: UserInput, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserInput;
  static deserializeBinaryFromReader(message: UserInput, reader: jspb.BinaryReader): UserInput;
}

export namespace UserInput {
  export type AsObject = {
    name: string,
    active: ActiveMap[keyof ActiveMap],
  }
}

export class UserSchema extends jspb.Message {
  getId(): number;
  setId(value: number): void;

  getName(): string;
  setName(value: string): void;

  getActive(): ActiveMap[keyof ActiveMap];
  setActive(value: ActiveMap[keyof ActiveMap]): void;

  hasCreatedAt(): boolean;
  clearCreatedAt(): void;
  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserSchema.AsObject;
  static toObject(includeInstance: boolean, msg: UserSchema): UserSchema.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: UserSchema, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserSchema;
  static deserializeBinaryFromReader(message: UserSchema, reader: jspb.BinaryReader): UserSchema;
}

export namespace UserSchema {
  export type AsObject = {
    id: number,
    name: string,
    active: ActiveMap[keyof ActiveMap],
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }
}

export class Message extends jspb.Message {
  getId(): number;
  setId(value: number): void;

  hasSenderId(): boolean;
  clearSenderId(): void;
  getSenderId(): UserSchema | undefined;
  setSenderId(value?: UserSchema): void;

  getMessageText(): string;
  setMessageText(value: string): void;

  hasCreatedAt(): boolean;
  clearCreatedAt(): void;
  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Message.AsObject;
  static toObject(includeInstance: boolean, msg: Message): Message.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Message, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Message;
  static deserializeBinaryFromReader(message: Message, reader: jspb.BinaryReader): Message;
}

export namespace Message {
  export type AsObject = {
    id: number,
    senderId?: UserSchema.AsObject,
    messageText: string,
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }
}

export class Users extends jspb.Message {
  clearUsersList(): void;
  getUsersList(): Array<UserSchema>;
  setUsersList(value: Array<UserSchema>): void;
  addUsers(value?: UserSchema, index?: number): UserSchema;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Users.AsObject;
  static toObject(includeInstance: boolean, msg: Users): Users.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Users, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Users;
  static deserializeBinaryFromReader(message: Users, reader: jspb.BinaryReader): Users;
}

export namespace Users {
  export type AsObject = {
    usersList: Array<UserSchema.AsObject>,
  }
}

export class Register extends jspb.Message {
  hasUser(): boolean;
  clearUser(): void;
  getUser(): UserSchema | undefined;
  setUser(value?: UserSchema): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Register.AsObject;
  static toObject(includeInstance: boolean, msg: Register): Register.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Register, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Register;
  static deserializeBinaryFromReader(message: Register, reader: jspb.BinaryReader): Register;
}

export namespace Register {
  export type AsObject = {
    user?: UserSchema.AsObject,
  }
}

export class ChatResults extends jspb.Message {
  hasMessage(): boolean;
  clearMessage(): void;
  getMessage(): Message | undefined;
  setMessage(value?: Message): void;

  hasUsers(): boolean;
  clearUsers(): void;
  getUsers(): Users | undefined;
  setUsers(value?: Users): void;

  getTypeCase(): ChatResults.TypeCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ChatResults.AsObject;
  static toObject(includeInstance: boolean, msg: ChatResults): ChatResults.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ChatResults, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ChatResults;
  static deserializeBinaryFromReader(message: ChatResults, reader: jspb.BinaryReader): ChatResults;
}

export namespace ChatResults {
  export type AsObject = {
    message?: Message.AsObject,
    users?: Users.AsObject,
  }

  export enum TypeCase {
    TYPE_NOT_SET = 0,
    MESSAGE = 1,
    USERS = 2,
  }
}

export interface ActiveMap {
  UNSPECIFIED: 0;
  OFF_LINE: 1;
  ACTIVE: 2;
  NOT_SEEN: 3;
}

export const Active: ActiveMap;

