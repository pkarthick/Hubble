<script language="typescript">
  import { Card, Button } from "smelte";
  import { createEventDispatcher } from "svelte";
  import Folder from "./Folder.svelte";

  const dispatch = createEventDispatcher();
  export let path;
  export let showfiles = false;
  export let foldername = path.substr(path.lastIndexOf("/") + 1);
</script>

<div class="m-1 flex border border-solid border-primary-200">

  <Card.Card>
    <div slot="title" class="bg-primary-200">
      <i
        class="far fa-lg fa-times-circle text-error-500 float-right mr-5 mt-5"
        title="Close this folder"
        on:click={() => dispatch('closetab', { path: path })} />
      <Card.Title
        title={foldername ? foldername : path}
        subheader={path}
        class="whitespace-no-wrap"
        avatar="https://cdn.iconscout.com/icon/premium/png-256-thumb/folder-1905837-1614415.png" />

    </div>
    <div
      slot="media"
      class="flex flex-1 w-full p-2 overflow-auto"
      style="max-height: 75vh">
      <Folder on:message {path} {showfiles} />
    </div>
    <div slot="text" />
    <div slot="actions" class="flex flex-row flex-initial mt-10 self-center">

      <div class="flex self-start">
        <Button text>Hide Date</Button>
        <Button text>Only Large Files</Button>
      </div>

      <div class="flex self-end p-3">
        <button>
          <i class="fas fa-ellipsis-h" />
        </button>

      </div>

    </div>
  </Card.Card>

</div>
