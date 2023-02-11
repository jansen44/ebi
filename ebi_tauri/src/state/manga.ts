import { writable, type Writable } from "svelte/store";

export const mangaList: Writable<Array<any>> = writable([]);
