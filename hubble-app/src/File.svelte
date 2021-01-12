<script language="typescript">
  import { count } from "./store.ts";
  import { onMount } from "svelte";
  export let path = "... loading";
  export let size;
  export let modified_date;
  $: name = path.slice(path.lastIndexOf("\\") + 1);
  $: type = name.slice(path.lastIndexOf(".") + 1);

  let units = "B";
  let size_in_bytes = size;

  const sizes = ["B", "KB", "MB", "GB"];

  function getModifiedSince(modified_date) {
    let mod = Date.parse(modified_date);
    let now = Date.now();

    var diff = now - mod;

    diff /= 1000; //in seconds

    let seconds_per_hour = 3600;
    let seconds_per_day = seconds_per_hour * 24;

    var days = Math.floor(diff / seconds_per_day);
    diff -= days * seconds_per_day;

    var hours = Math.floor(diff / seconds_per_hour);
    diff -= hours * seconds_per_hour;

    var mins = Math.floor(diff / 60);
    diff -= mins * 60;

    var seconds = diff;

    if (days > 0) {
      return days + " d";
    } else if (hours > 0) {
      return hours + " h";
    } else if (mins > 0) {
      return mins + " m";
    } else return seconds + " s";
  }

  function format_size(sz, index) {
    if (sz <= 1024 || index === 3) {
      size = sz.toFixed();
      units = sizes[index];
    } else format_size(sz / 1024, index + 1);
  }
  onMount(function() {
    format_size(size, 0);
  });
</script>

<style>
  div.trunc {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
</style>

<div class="flex flex-row ml-3 whitespace-no-wrap">

  <div class="flex-grow-0" style="min-width: 50px; ">
    <span title={modified_date} class="text-left">
      <i>{getModifiedSince(modified_date)}</i>
    </span>
  </div>

  <div
    class="flex-auto font-bold text-right"
    style="background-image: url(https://svelte.dev/tutorial/icons/{type}.svg);
    min-width: 50px; max-width: 50px;">
    {#if units === 'B'}
      <span class="text-success-500" title={size_in_bytes}>{size}</span>
    {:else if units === 'KB'}
      <span class="text-alert-300" title={size_in_bytes}>{size}</span>
    {:else if units === 'MB' && size < 100}
      <span class="text-alert-800" title={size_in_bytes}>{size}</span>
    {:else if units === 'MB'}
      <span class="text-error-500" title={size_in_bytes}>{size}</span>
    {:else if units === 'GB'}
      <span class="text-error-900" title={size_in_bytes}>{size}</span>
    {:else}
      <span class="text-error-900" title={size_in_bytes}>{size}</span>
    {/if}

  </div>

  <div
    class="ml-1 font-bold text-left"
    style="min-width: 25px; max-width: 25px; ">
    {#if units === 'B'}
      <span class="text-success-500">{units}</span>
    {:else if units === 'KB'}
      <span class="text-alert-300">{units}</span>
    {:else if units === 'MB' && size < 100}
      <span class="text-alert-800">{units}</span>
    {:else if units === 'MB'}
      <span class="text-error-500">{units}</span>
    {:else if units === 'GB'}
      <span class="text-error-900">{units}</span>
    {:else}
      <span class="text-error-900">{units}</span>
    {/if}

  </div>

  <div
    class="ml-4 flex-auto text-right trunc"
    title={name}
    style="max-width: 150px;">
    <a href={path} target="_blank">{name}</a>

  </div>

</div>
